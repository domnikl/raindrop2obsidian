mod highlights;
mod raindrop;

use dotenvy::dotenv;
use raindrop::RaindropClient;
use std::env;
use std::error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let raindrop_access_token =
        env::var("RAINDROP_ACCESS_TOKEN").expect("RAINDROP_ACCESS_TOKEN must be set");

    let raindrop_client = RaindropClient::new(raindrop_access_token).unwrap();
    let highlights = raindrop_client.highlights().await?;

    println!("{:?}", highlights);

    Ok(())
}
