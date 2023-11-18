use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

use crate::helper::api::call_api;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct NewPlayer<'a> {
    symbol: &'a str,
    faction: &'a str,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TokenData {
    data: Token,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Token {
    token: String,
}

#[allow(dead_code)]
impl<'a> NewPlayer<'a> {
    async fn new_player(call_sign: &'a str, faction: &'a str) -> Result<Token, reqwest::Error> {
        let new_player = Self {
            symbol: call_sign,
            faction,
        };

        let token: TokenData = Client::new()
            .post("https://api.spacetraders.io/v2/register")
            .header("Content-Type", "application/json")
            .json(&new_player)
            .send()
            .await?
            .json()
            .await?;

        Ok(token.data)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Player {
    data: Data,
    #[serde(default)]
    current_waypoint: CurrentWaypoint,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    account_id: String,
    symbol: String,
    headquarters: String,
    credits: u32,
    starting_faction: String,
    ship_count: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CurrentWaypoint {
    sector: String,
    system: String,
    location: String,
}

impl Player {
    pub async fn player_info(&mut self, token: &str) -> Result<(), reqwest::Error> {
        call_api(self, Method::GET, "/my/agent", token).await?;

        Ok(())
    }

    pub fn get_hq_waypoint(&self) -> &str {
        &self.data.headquarters
    }

    pub fn get_current_waypoint(&mut self) -> &str {
        self.current_waypoint.sector.push('-');
        self.current_waypoint
            .sector
            .push_str(&self.current_waypoint.system);
        self.current_waypoint.sector.push('-');
        self.current_waypoint
            .sector
            .push_str(&self.current_waypoint.location);
        &self.current_waypoint.sector
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::domain::player::Player;

    fn get_token() -> String {
        dotenv().ok();

        std::env::var("ACCESS_TOKEN").unwrap()
    }

    #[tokio::test]
    async fn test_player_info() {
        let mut player: Player = Player::default();
        player.player_info(&get_token()).await.unwrap();

        dbg!(&player);

        assert_ne!(player, Player::default());
    }
}
