pub mod domain;
pub mod helper;

use domain::{contracts::Contracts, player::Player};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let token = std::env::var("ACCESS_TOKEN").expect("Access token should be set.");

    let player = Player::player_info(&token).await?;

    let contract = Contracts::get_contracts(&token).await?;

    contract.accept_contract(0, &token).await?;

    dbg!(player);
    println!();
    dbg!(contract);

    Ok(())
}
