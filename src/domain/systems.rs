use serde::{Deserialize, Serialize};

use crate::domain::player::Player;
use crate::helper::{api::call_api_get, structs::Meta};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Systems {
    data: System,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct System {
    system_symbol: String,
    symbol: String,
    r#type: String,
    x: i32,
    y: i32,
    orbitals: Vec<Orbitals>,
    traits: Vec<Traits>,
    modifiers: Vec<String>,
    chart: Chart,
    faction: Faction,
    #[serde(default)]
    orbits: String,
    is_under_construction: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Orbitals {
    symbol: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Traits {
    symbol: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Chart {
    submitted_by: String,
    submitted_on: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Faction {
    symbol: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Shipyards {
    data: Vec<System>,
    meta: Meta,
}

impl Systems {
    pub async fn find_shipyards(player: Player, token: &str) -> Result<Shipyards, reqwest::Error> {
        let waypoint = player.get_hq_waypoint();
        let (system_loc, _) = waypoint
            .rsplit_once('-')
            .expect("player arg should have been checked for validity before being passed in.");

        let api = format!("/systems/{system_loc}/waypoints?traits=SHIPYARD");

        let shipyards: Shipyards = call_api_get(&api, token).await?.json().await?;

        Ok(shipyards)
    }

    pub async fn get_player_headquarters(
        player: Player,
        token: &str,
    ) -> Result<Systems, reqwest::Error> {
        let waypoint = player.get_hq_waypoint();
        let (system_loc, _) = waypoint
            .rsplit_once('-')
            .expect("player arg should have been checked for validity before being passed in.");

        let api = format!("/systems/{system_loc}/waypoints/{waypoint}");

        let hq: Systems = call_api_get(&api, token).await?.json().await?;

        Ok(hq)
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::domain::{player::Player, systems::Systems};

    fn get_token() -> String {
        dotenv().ok();

        std::env::var("ACCESS_TOKEN").unwrap()
    }

    #[tokio::test]
    async fn test_get_headquarters() {
        let token = get_token();

        let player = Player::player_info(&token).await.unwrap();
        let hq = Systems::get_player_headquarters(player, &token)
            .await
            .unwrap();

        dbg!(hq);
    }

    #[tokio::test]
    async fn test_find_shipyards() {
        let token = get_token();

        let player = Player::player_info(&token).await.unwrap();
        let shipyards = Systems::find_shipyards(player, &token).await.unwrap();

        dbg!(shipyards);
    }
}
