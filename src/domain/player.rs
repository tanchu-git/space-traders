use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

use crate::helper::{api::call_api, structs::APIError};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NewPlayer<'a> {
    symbol: &'a str,
    faction: &'a str,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum NewPlayerVariant {
    Data(Token),
    Error(APIError),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(default)]
struct NewPlayerVariantStruct {
    data: Token,
    error: APIError,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Token {
    token: String,
}

#[allow(dead_code)]
impl<'a> NewPlayer<'a> {
    pub async fn new_player(
        call_sign: &'a str,
        faction: &'a str,
    ) -> Result<NewPlayerVariant, reqwest::Error> {
        let new_player = Self {
            symbol: call_sign,
            faction,
        };

        let data_or_error: NewPlayerVariantStruct = Client::new()
            .post("https://api.spacetraders.io/v2/register")
            .header("Content-Type", "application/json")
            .json(&new_player)
            .send()
            .await?
            .json()
            .await?;

        match data_or_error.error.is_empty() {
            true => Ok(NewPlayerVariant::Data(data_or_error.data)),
            false => Ok(NewPlayerVariant::Error(data_or_error.error)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum PlayerVariant {
    Data(Data),
    Error(APIError),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
#[serde(default)]
pub struct PlayerVariantStruct {
    data: Data,
    error: APIError,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    account_id: String,
    symbol: String,
    headquarters: String,
    credits: u32,
    starting_faction: String,
    ship_count: u16,
}

impl PlayerVariantStruct {
    pub async fn player_info(&mut self, token: &str) -> Result<PlayerVariant, reqwest::Error> {
        call_api(self, Method::GET, "/my/agent", token).await?;

        match self.error.is_empty() {
            true => Ok(PlayerVariant::Data(self.data.clone())),
            false => Ok(PlayerVariant::Error(self.error.clone())),
        }
    }

    pub fn get_hq_waypoint(&self) -> &str {
        &self.data.headquarters
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::domain::player::{NewPlayer, PlayerVariantStruct};

    fn get_token() -> String {
        dotenv().ok();

        std::env::var("ACCESS_TOKEN").unwrap()
    }

    #[tokio::test]
    async fn test_new_player() {
        let player = NewPlayer::new_player("Yonda", "COSMIC").await.unwrap();

        dbg!(&player);
    }

    #[tokio::test]
    async fn test_player_info() {
        let mut player = PlayerVariantStruct::default();
        let player = player.player_info(&get_token()).await.unwrap();

        dbg!(&player);
    }
}
