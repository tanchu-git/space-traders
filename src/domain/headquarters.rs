use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::domain::player::Player;
use crate::helper::api::call_api;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Headquarters {
    data: System,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
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
    orbits: String,
    is_under_construction: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
struct Orbitals {
    symbol: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
struct Traits {
    symbol: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
struct Chart {
    submitted_by: String,
    submitted_on: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
struct Faction {
    symbol: String,
}

impl Headquarters {
    pub async fn get_player_headquarters(
        &mut self,
        player: &Player,
        token: &str,
    ) -> Result<(), reqwest::Error> {
        let waypoint = player.get_hq_waypoint();
        let (system_loc, _) = waypoint
            .rsplit_once('-')
            .expect("player arg should have been validated before being passed in.");

        let api = format!("/systems/{system_loc}/waypoints/{waypoint}");

        call_api(self, Method::GET, &api, token).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::domain::{headquarters::Headquarters, player::Player};

    fn get_token() -> String {
        dotenv().ok();

        std::env::var("ACCESS_TOKEN").unwrap()
    }

    #[tokio::test]
    async fn test_get_headquarters() {
        let mut player: Player = Player::default();

        player.player_info(&get_token()).await.unwrap();
        let mut hq = Headquarters::default();

        hq.get_player_headquarters(&player, &get_token())
            .await
            .unwrap();

        dbg!(&hq);

        assert_ne!(hq, Headquarters::default());
    }
}
