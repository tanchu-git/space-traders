pub mod domain;
pub mod helper;

use domain::{contracts::ContractsVariantStruct, player::PlayerVariantStruct};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let mut player = PlayerVariantStruct::default();

    let token = std::env::var("ACCESS_TOKEN").expect("Access token should be set.");

    player.player_info(&token).await?;

    let mut contracts: ContractsVariantStruct = ContractsVariantStruct::default();
    contracts.get_contracts(&token).await?;
    // let accepted = contracts.accept_contract(0, &token).await?;

    dbg!(player);
    println!();
    dbg!(contracts);
    println!();
    // dbg!(accepted);

    Ok(())
}
