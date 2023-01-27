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
        password: String,
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
        token: Option<String>,
        /// User ID
        #[arg(short,long)]
        user: Option<String>,
        /// User Password
        #[arg(short,long)]
        password: Option<String>,
        /// Variant ID
        #[arg(short,long)]
        id: Option<String>,
    }
}

#[derive(Args, Debug)]
struct Login {
    #[arg(short,long)]
    user: String,
    password: String,
}

fn mapper(params: Vec<String>) -> HashMap<String, String>{
    let mut args = HashMap::new();
    for arg in params.iter(){
        let split = arg.split_whitespace().collect::<Vec<_>>();
        args.insert(split[0].to_string(),split[1].to_string());
    }
    args
}

async fn login(params: Vec<String>){
    let endpoint = "login".to_string();
    let handle = request(mapper(params), endpoint);
    println!("{:#?}", handle.await)

}

async fn signup(params: Vec<String>){
    let endpoint = "signup".to_string();
    let handle = request(mapper(params), endpoint);
    println!("{:#?}", handle.await)
}

async fn query(params: Vec<String>) {
    let endpoint = "variant".to_string();
    let handle = request(mapper(params), endpoint);
    println!("{:#?}", handle.await)
}

async fn request(params: HashMap<String,String>, endpoint: String) -> Result<Value, reqwest::Error> {
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

async fn reader(arg: CommandReader){
    match arg.command {
        Commands::Login {user, password} => {
            let params = vec![format!("user {}",user),format!("password {}",password)];
            login(params).await;
        },
        Commands::Signup {id, email, name, organization,password} => {
            let params = vec![
                format!("id {}",id),
                format!("email {}",email),
                format!("organization {}",organization),
                format!("name {}",name),
                format!("password {}",password)];
            signup(params).await;
        },
        Commands::Query {token, user, password, id} => {
            let params = vec![token,user,password,id]
                .iter().map(|x| x.clone().unwrap_or("".to_string())).collect();
            query(params).await;
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
    reader(args).await;
}