//! Testing helpers/utilities for Fuel SDK.

use fuel_core::{
    chain_config::{ChainConfig, CoinConfig, StateConfig},
    model::{Coin, CoinStatus},
    service::{Config, DbType, FuelService},
};
use fuel_crypto::{PublicKey, SecretKey};
use fuel_gql_client::client::FuelClient;
use fuel_tx::{Address, Bytes32, UtxoId};
use rand::Fill;
use std::net::SocketAddr;

#[cfg(feature = "fuels-signers")]
mod signers;
#[cfg(feature = "fuels-signers")]
pub use signers::*;

// This constant is used to set a initial balance on wallets
// mainly used on tests
pub const DEFAULT_INITIAL_BALANCE: u64 = 1_000_000_000;

pub fn setup_address_and_coins(
    num_of_coins: usize,
    amount: u64,
) -> (SecretKey, Vec<(UtxoId, Coin)>) {
    let mut rng = rand::thread_rng();

    let secret = SecretKey::random(&mut rng);

    let public = PublicKey::from(&secret);
    let hashed = public.hash();

    let coins: Vec<(UtxoId, Coin)> = (1..=num_of_coins)
        .map(|_i| {
            let coin = Coin {
                owner: Address::from(*hashed),
                amount,
                asset_id: Default::default(),
                maturity: Default::default(),
                status: CoinStatus::Unspent,
                block_created: Default::default(),
            };

            let mut r = Bytes32::zeroed();
            r.try_fill(&mut rng).unwrap();
            let utxo_id = UtxoId::new(r, 0);
            (utxo_id, coin)
        })
        .collect();

    (secret, coins)
}

// Setup a test client with the given coins. We return the SocketAddr so the launched node
// client can be connected to more easily (even though it is often ignored).
pub async fn setup_test_client(coins: Vec<(UtxoId, Coin)>) -> (FuelClient, SocketAddr) {
    let coin_configs = coins
        .into_iter()
        .map(|(utxo_id, coin)| CoinConfig {
            tx_id: Some(*utxo_id.tx_id()),
            output_index: Some(utxo_id.output_index() as u64),
            block_created: Some(coin.block_created),
            maturity: Some(coin.maturity),
            owner: coin.owner,
            amount: coin.amount,
            asset_id: coin.asset_id,
        })
        .collect();

    // Setup node config with genesis coins and utxo_validation enabled
    let config = Config {
        chain_conf: ChainConfig {
            initial_state: Some(StateConfig {
                coins: Some(coin_configs),
                ..StateConfig::default()
            }),
            ..ChainConfig::local_testnet()
        },
        database_type: DbType::InMemory,
        utxo_validation: true,
        ..Config::local_node()
    };

    let srv = FuelService::new_node(config).await.unwrap();
    let client = FuelClient::from(srv.bound_address);

    (client, srv.bound_address)
}
