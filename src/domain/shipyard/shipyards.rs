use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    domain::headquarters::System,
    helper::{api::call_api, structs::Meta},
};

use super::ships::Ships;

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
#[serde(rename_all = "camelCase", default)]
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
        system: &str,
        token: &str,
    ) -> Result<(), reqwest::Error> {
        let api = format!("/systems/{system}/waypoints?traits=SHIPYARD");

        call_api(self, Method::GET, &api, token).await?;

        Ok(())
    }
}

impl Shipyard {
    pub async fn view_available_ships(
        &mut self,
        waypoint: &str,
        token: &str,
    ) -> Result<(), reqwest::Error> {
        let (system, _) = waypoint
            .rsplit_once('-')
            .expect("waypoint arg should have been validated before being passed in.");

        let api = format!("/systems/{system}/waypoints/{waypoint}/shipyard");

        call_api(self, Method::GET, &api, token).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::shipyard::shipyards::{Shipyard, Shipyards};
    use dotenv::dotenv;

    fn get_token() -> String {
        dotenv().ok();

        std::env::var("ACCESS_TOKEN").unwrap()
    }

    #[tokio::test]
    async fn test_find_shipyards() {
        // let mut player: Player = Player::default();

        // player.player_info(&get_token()).await.unwrap();
        let mut shipyards = Shipyards::default();
        shipyards
            .find_shipyards("X1-H7", &get_token())
            .await
            .unwrap();

        dbg!(&shipyards);

        assert_ne!(shipyards, Shipyards::default());
    }

    #[tokio::test]
    async fn test_view_available_ships() {
        // let mut player: Player = Player::default();

        // player.player_info(&get_token()).await.unwrap();
        let mut shipyard = Shipyard::default();
        shipyard
            .view_available_ships("X1-NY50-C45", &get_token())
            .await
            .unwrap();

        dbg!(&shipyard);

        assert_ne!(shipyard, Shipyard::default());
    }
}
