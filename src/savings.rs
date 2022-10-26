use crate::util::build_signed_request;
use crate::model::{AssetDetail, CoinInfo, DepositAddress};
use crate::client::Client;
use crate::errors::Result;
use std::collections::{BTreeMap, HashMap};
use crate::api::API;
use crate::api::Sapi;

#[derive(Clone)]
pub struct Savings {
    pub client: Client,
    pub recv_window: u64,
}

impl Savings {
    /// Get all coins available for deposit and withdrawal
    pub fn get_all_coins(&self) -> Result<Vec<CoinInfo>> {
        let request = build_signed_request(HashMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Savings(Sapi::AllCoins), Some(request))
    }

    /// Fetch details of assets supported on Binance.
    pub fn asset_detail(&self, asset: Option<String>) -> Result<BTreeMap<String, AssetDetail>> {
        let mut parameters = HashMap::new();
        if let Some(asset) = asset {
            parameters.insert("asset", asset);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Savings(Sapi::AssetDetail), Some(request))
    }

    /// Fetch deposit address with network.
    ///
    /// You can get the available networks using `get_all_coins`.
    /// If no network is specified, the address for the default network is returned.
    pub fn deposit_address<S>(&self, coin: S, network: Option<String>) -> Result<DepositAddress>
    where
        S: Into<String>,
    {
        let mut parameters = HashMap::new();
        parameters.insert("coin", coin.into());
        if let Some(network) = network {
            parameters.insert("network", network);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Savings(Sapi::DepositAddress), Some(request))
    }
}
