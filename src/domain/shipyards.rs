use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::helper::{api::call_api, structs::Meta};

use super::{headquarters::System, player::Player, ships::Ships};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Shipyards {
    data: Vec<System>,
    meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Shipyard {
    data: ShipyardDetails,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardDetails {
    symbol: String,
    ship_types: Vec<ShipType>,
    transactions: Vec<TransactionLog>,
    ships: Vec<Ships>,
    modifications_fee: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ShipType {
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLog {
    ship_symbol: String,
    waypoint_symbol: String,
    agent_symbol: String,
    price: u32,
    timestamp: String,
}

impl Shipyards {
    pub async fn find_shipyards(
        &mut self,
        player: &mut Player,
        token: &str,
    ) -> Result<(), reqwest::Error> {
        let waypoint = todo!();
        let (system_loc, _) = waypoint
            .rsplit_once('-')
            .expect("player arg should have been validated before being passed in.");

        let api = format!("/systems/{system_loc}/waypoints?traits=SHIPYARD");

        call_api(self, Method::GET, &api, token).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{player::Player, shipyards::Shipyards};
    use dotenv::dotenv;

    fn get_token() -> String {
        dotenv().ok();

        std::env::var("ACCESS_TOKEN").unwrap()
    }

    #[tokio::test]
    async fn test_find_shipyards() {
        let mut player: Player = Player::default();

        player.player_info(&get_token()).await.unwrap();
        let mut shipyards = Shipyards::default();
        shipyards
            .find_shipyards(&mut player, &get_token())
            .await
            .unwrap();

        dbg!(&shipyards);

        assert_ne!(shipyards, Shipyards::default());
    }
}
