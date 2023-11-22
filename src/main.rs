pub mod domain;
pub mod helper;

use domain::{contracts::ContractsVariantStruct, player::PlayerVariantStruct};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let mut player = PlayerVariantStruct::default();

    let token = std::env::var("ACCESS_TOKEN").expect("Access token should be set.");

    let player = player.player_info(&token).await?;

    let mut contracts: ContractsVariantStruct = ContractsVariantStruct::default();
    let contract = contracts.get_contracts(&token).await?;
    let accepted = contracts.accept_contract_struct(0, &token).await?;

    println!("{player:#?}");
    println!();
    println!("{contract:#?}");
    println!();
    println!("{accepted:#?}");
    Ok(())
}
