use std::collections::HashMap;
use serde_json::Value;

async fn login(){
    let endpoint = "login".to_string();
    let mut map = HashMap::new();
    map.insert("user", "wilson");
    map.insert("password", "tatqd3uX@");
    let handle = request(map, endpoint);
    println!("{:#?}", handle.await)

}

async fn signup(){
    let endpoint = "signup".to_string();
    let mut map = HashMap::new();
    map.insert("id", "dingi");
    map.insert("name", "toto dingi");
    map.insert("email","dingi@email.com");
    map.insert("password", "tatqd3uX@");
    map.insert("organization", "ICIPE");
    let handle = request(map, endpoint);
    println!("{:#?}", handle.await)
}

async fn request(params: HashMap<&str,&str>, endpoint: String) -> Result<Value, reqwest::Error> {
    let url = format!("http://localhost:3000/agvd/{}", endpoint);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let res = client.post(url)
        .json(&params)
        .send()
        .await?;
    let token = res.text().await?;
    let val : Value = serde_json::from_str(&token).unwrap();
    Ok(val)
}


#[tokio::main]
async fn main() {
    // login().await;
    signup().await;
}