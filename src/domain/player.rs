use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::helper::api;

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

impl Player {
    pub async fn player_info(token: &str) -> Result<Player, reqwest::Error> {
        let player: Player = api::call_api_get("/my/agent", token).await?.json().await?;

        Ok(player)
    }

    pub fn get_hq_waypoint(&self) -> &str {
        &self.data.headquarters
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::{domain::player::Player, helper::api::call_api_get_generic};

    fn get_token() -> String {
        dotenv().ok();

        std::env::var("ACCESS_TOKEN").unwrap()
    }

    #[tokio::test]
    async fn test_player_info() {
        let player_status = Player::player_info(&get_token()).await.unwrap();

        dbg!(player_status);
    }

    #[tokio::test]
    async fn test_call_api_get_generic() {
        let player = call_api_get_generic(Player::default(), "/my/agent", &get_token())
            .await
            .unwrap();

        dbg!(player);
    }
}
