use serde::{Deserialize, Serialize};

use crate::helper::{
    api::{call_api_get, call_api_post},
    structs::Meta,
};

use super::player;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Contracts {
    data: Vec<Contract>,
    meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Contract {
    id: String,
    faction_symbol: String,
    r#type: String,
    terms: Terms,
    accepted: bool,
    fulfilled: bool,
    expiration: String,
    deadline_to_accept: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Terms {
    deadline: String,
    payment: Payment,
    deliver: Vec<ContractFulfilment>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Payment {
    on_accepted: u32,
    on_fulfilled: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ContractFulfilment {
    trade_symbol: String,
    destination_symbol: String,
    units_required: u32,
    units_fulfilled: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AcceptedContract {
    data: ContractDetails,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ContractDetails {
    contract: Contract,
    agent: player::Data,
}

impl Contracts {
    pub async fn get_contracts(token: &str) -> Result<Contracts, reqwest::Error> {
        let contracts: Contracts = call_api_get("/my/contracts", token).await?.json().await?;

        Ok(contracts)
    }

    pub async fn accept_contract(
        &self,
        contract_id: usize,
        token: &str,
    ) -> Result<AcceptedContract, reqwest::Error> {
        let api = format!("/my/contracts/{}/accept", self.data[contract_id].id);
        let accepted_contract: AcceptedContract = call_api_post(&api, token).await?.json().await?;

        Ok(accepted_contract)
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::domain::contracts::Contracts;

    fn get_token() -> String {
        dotenv().ok();

        std::env::var("ACCESS_TOKEN").unwrap()
    }

    #[tokio::test]
    async fn test_get_player_contracts() {
        let contracts = Contracts::get_contracts(&get_token()).await.unwrap();

        dbg!(contracts);
    }

    #[tokio::test]
    #[ignore]
    async fn test_accept_contract() {
        let contracts = Contracts::get_contracts(&get_token()).await.unwrap();
        let accepted_contract = contracts.accept_contract(0, &get_token()).await.unwrap();

        dbg!(accepted_contract);
    }
}
