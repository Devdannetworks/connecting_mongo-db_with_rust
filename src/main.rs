use std::env;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::error::Error;
use tokio;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let client_uri = env::var("MONGO_URI").expect("No uri provided");
    println!("{}", client_uri);

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
    println!("{:?}", options);
    let client = Client::with_options(options)?;

    println!("Databases");
    for db in client.list_database_names(None, None).await? {
        println!("- {}", db);
    }
 
    Ok(())

}
