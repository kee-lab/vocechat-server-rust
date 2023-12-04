use gear_contract_api::user_have_subject_share;


#[tokio::main]
async fn main() {
    let user_have_subject_share = user_have_subject_share().await;
    println!("user_have_subject_share is:{:?}",user_have_subject_share);
}