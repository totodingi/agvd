use std::collections::HashMap;
use serde_json::Value;

async fn request() -> Result<(), reqwest::Error> {
    let url = "http://localhost:3000/agvd/variant";
    let mut map = HashMap::new();
    map.insert("user","wilson");
    map.insert("password", "tatqd3uX@");

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let res = client.post(url)
        .json(&map)
        .send()
        .await?;
    let token = res.text().await?;
    let val : Value = serde_json::from_str(&token).unwrap();
    println!("{:#?}",val[0]);

    Ok(())
}


#[tokio::main]
async fn main() {
    request().await.unwrap();
}