use gclient::{GearApi, Result, WSAddress, EventProcessor};
use gear_core::ids::ProgramId;
use parity_scale_codec::{Encode, Decode};
use scale_info::TypeInfo;

const WASM_PATH: &str = "./target/wasm32-unknown-unknown/release/gear_friend_share.opt.wasm";

pub async fn user_have_subject_share() -> Result<()> {
    // Create API instance
    let api = GearApi::init(WSAddress::new("wss://testnet.vara-network.io", 443)).await?;

    // Subscribe to events
    let mut listener = api.subscribe().await?;

    // Check that blocks are still running
    assert!(listener.blocks_running().await?);

    let hex_value = "0x1371d9c044ff3f249eb6a647c4807ed5e4f07ef98ea62a7043e9546b547503e5";
    let bytes = hex_to_bytes(&hex_value[2..]);
    let program_id:ProgramId = ProgramId::from(bytes.as_slice());

    let payload = br#"{ subject:"0xec59e48cf877dfab6e6ba04b24d29349f11cf0bcfa44d04d7b875397225a1b2a", user:"0xec59e48cf877dfab6e6ba04b24d29349f11cf0bcfa44d04d7b875397225a1b2a"}"#.to_vec();

    // Send the PING message
    let share_state:StateReply = api.read_state(program_id, payload).await.expect("read share error!");
    println!("share_state is:{:?}",share_state);

    Ok(())
}

#[derive(Debug,Encode, Decode, TypeInfo)]
pub enum StateReply {
    Price(u128),
    ShareAmount(u128),
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

#[tokio::test]
async fn test_example() -> Result<()> {
    let share_program_id = "0x3bc507b6d448d7f279e6f0edb80bd86e569d0543a84a6c5cde8c93ed01453161";
    // Create API instance
    // let api = GearApi::dev().await?;
    let api = GearApi::init(WSAddress::new("wss://testnet.vara-network.io", 443)).await?;

    // Subscribe to events
    let mut listener = api.subscribe().await?;

    // Check that blocks are still running
    assert!(listener.blocks_running().await?);

    // Calculate gas amount needed for initialization
    let gas_info = api
        .calculate_upload_gas(None, gclient::code_from_os(WASM_PATH)?, vec![], 0, true)
        .await?;

    // Upload and init the program
    let (message_id, program_id, _hash) = api
        .upload_program_bytes_by_path(
            WASM_PATH,
            gclient::now_micros().to_le_bytes(),
            vec![],
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let payload = b"PING".to_vec();

    // Calculate gas amount needed for handling the message
    let gas_info = api
        .calculate_handle_gas(None, program_id, payload.clone(), 0, true)
        .await?;

    // Send the PING message
    let (message_id, _hash) = api
        .send_message_bytes(program_id, payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    Ok(())
}
