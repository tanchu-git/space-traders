use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::helper::{
    api::call_api,
    structs::{APIError, Meta},
};

use super::player;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Contracts {
    data: Vec<Contract>,
    meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
struct Terms {
    deadline: String,
    payment: Payment,
    deliver: Vec<ContractFulfilment>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
struct Payment {
    on_accepted: u32,
    on_fulfilled: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
struct ContractFulfilment {
    trade_symbol: String,
    destination_symbol: String,
    units_required: u32,
    units_fulfilled: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum AcceptContractVariant {
    Data(Box<ContractDetails>),
    Error(APIError),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "lowercase", default)]
pub struct AcceptContractVariantStruct {
    data: ContractDetails,
    error: APIError,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ContractDetails {
    contract: Contract,
    agent: player::Data,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContractData {
    contract_id: String,
}

impl Contracts {
    pub async fn get_contracts(&mut self, token: &str) -> Result<(), reqwest::Error> {
        call_api(self, Method::GET, "/my/contracts", token).await?;

        Ok(())
    }

    pub async fn accept_contract_struct(
        &mut self,
        contract_id: usize,
        token: &str,
    ) -> Result<AcceptContractVariant, reqwest::Error> {
        let api = format!("/my/contracts/{}/accept", self.data[contract_id].id);
        let mut accepted_contract_variant = AcceptContractVariantStruct::default();
        call_api(&mut accepted_contract_variant, Method::POST, &api, token).await?;

        match accepted_contract_variant.error.is_empty() {
            true => Ok(AcceptContractVariant::Data(Box::new(
                accepted_contract_variant.data,
            ))),
            false => Ok(AcceptContractVariant::Error(
                accepted_contract_variant.error,
            )),
        }
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
        let mut contracts: Contracts = Contracts::default();
        contracts.get_contracts(&get_token()).await.unwrap();

        dbg!(&contracts);
        assert_ne!(contracts, Contracts::default());
    }

    #[tokio::test]
    async fn test_accept_contract() {
        let mut contracts: Contracts = Contracts::default();
        contracts.get_contracts(&get_token()).await.unwrap();
        let accepted_contract = contracts
            .accept_contract_struct(0, &get_token())
            .await
            .unwrap();

        dbg!(accepted_contract);
    }
}
