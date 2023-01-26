use std::collections::HashMap;
use serde_json::Value;
use std::env;
use clap::{Args, Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(about="African Genome Variation Database")]
#[command(propagate_version = true)]
struct CommandReader {
    #[command(subcommand)]
    command: Commands

}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Login User
    Login {
        /// User's unique ID
        #[arg(short,long)]
        user: String,
        /// User's Password
        #[arg(short,long)]
        password: Option<String>,
    },
    /// User Signup
    Signup {
        /// User id
        #[arg(short,long)]
        id: String,
        /// Full name
        #[arg(short,long)]
        name: String,
        /// Email
        #[arg(short,long)]
        email: String,
        /// Preferred password
        #[arg(short,long)]
        password: String,
        /// Affiliate organization
        #[arg(short,long)]
        organization: String,
    },
    /// Variant Query
    Query {
        /// User access token
        #[arg(short,long)]
        token: String,
        /// User ID
        #[arg(short,long)]
        user: String,
        /// User Password
        #[arg(short,long)]
        password: String,
        /// Variant ID
        #[arg(short,long)]
        id: String,
    }
}

#[derive(Args, Debug)]
struct Login {
    #[arg(short,long)]
    user: String,
    password: String,
}


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

async fn query() {
    let endpoint = "variant".to_string();
    let mut map = HashMap::new();
    map.insert("user", "wilson");
    map.insert("password", "tatqd3uX@");
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

fn reader(arg: CommandReader){
    // let args : Vec<String> = env::args().collect();
    // arg.iter().map(|x| println!("ARGS {}", x)).collect()
    match arg.command {
        Commands::Login {user, password} => {
            println!("User wants to Login {} {}", user, password.unwrap_or("".to_string()))
        },
        Commands::Signup {id, email, name, organization,password} => {
            println!("User wants to Signup")
        },
        Commands::Query {token, user, password, id} => {
            println!("User wants to query variant")
        }
    }

}

#[tokio::main]
async fn main() {
    // login().await;
    // signup().await;
    // query().await;
    // let args : Vec<String> = env::args().collect();
    let args = CommandReader::parse();
    // println!("{:#?}", args);
    reader(args)
}