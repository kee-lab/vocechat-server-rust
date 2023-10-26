pub fn get_client(proxy:Option<String>)->Client{
    
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("http://127.0.0.1:10080")?)
        .build()?;
}