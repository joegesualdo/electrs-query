use bitcoin_address;
use bitcoin_utils::{
    self, get_pubkey_hash_from_p2wpkh_address, get_public_key_hash_from_address,
    get_script_hash_from_p2sh_address,
};
use electrs_request::{
    BlockchainRelayFeeCommand, BlockchainScriptHashGetBalanceCommand,
    BlockchainScriptHashGetHistoryCommand, BlockchainScriptHashListUnspentCommand,
    Client as ElectrsRequestClient,
};
use hex_utilities::convert_big_endian_hex_to_little_endian;

pub struct Client {
    electrs_request_client: ElectrsRequestClient,
}
impl Client {
    pub fn new(electrum_server_address: &str) -> Self {
        let electrs_request_client = ElectrsRequestClient::new(electrum_server_address);
        Client {
            electrs_request_client,
        }
    }
}

#[derive(Debug)]
pub struct AddressBalance {
    pub confirmed: u64,
    pub unconfirmed: u64,
}
#[derive(Debug)]
pub struct HistoricalTransaction {
    pub height: i64,
    pub tx_hash: String,
}

fn get_script_hash_for_p2pkh_address(p2pkh_address: &str) -> String {
    let p2pkh_address = p2pkh_address.to_string();
    let p2pkh_pk_hash = get_public_key_hash_from_address(&p2pkh_address);
    let p2pkh_script = format!("{}{}{}", "76a914", p2pkh_pk_hash, "88ac");
    let p2pkh_script_hash = bitcoin_utils::sha256_hex(&p2pkh_script);
    p2pkh_script_hash
}
fn get_script_hash_for_p2sh_address(p2sh_address: &str) -> String {
    let p2sh_address = p2sh_address.to_string();
    let p2sh_script_hash = get_script_hash_from_p2sh_address(&p2sh_address);
    let p2sh_script = format!("{}{}{}", "a914", p2sh_script_hash, "87");
    let p2sh_script_hash_sha256 = bitcoin_utils::sha256_hex(&p2sh_script);
    p2sh_script_hash_sha256
}
fn get_script_hash_for_p2wpkh_address(p2wpkh_address: &str) -> String {
    let p2wpkh_address = p2wpkh_address.to_string();
    let p2wpkh_pk_hash = get_public_key_hash_from_address(&p2wpkh_address);
    let p2wpkh_script = format!("{}{}", "0014", p2wpkh_pk_hash);
    let p2wpkh_script_hash = bitcoin_utils::sha256_hex(&p2wpkh_script);
    p2wpkh_script_hash
}

fn get_script_hash_for_address(address: &str) -> String {
    let address = &address.to_string();
    if bitcoin_address::is_p2pkh(address) {
        get_script_hash_for_p2pkh_address(address)
    } else if bitcoin_address::is_p2sh(address) {
        get_script_hash_for_p2sh_address(address)
    } else if bitcoin_address::is_p2wpkh(address) {
        get_script_hash_for_p2wpkh_address(address)
    } else {
        panic!("Address type not supported: {}", address);
    }
}
pub fn get_relay_fee(client: &Client) -> f64 {
    let relay_fee_response = BlockchainRelayFeeCommand::new()
        .call(&client.electrs_request_client)
        .unwrap();
    relay_fee_response.0
}
pub fn get_balance_for_address(address: &str, client: &Client) -> AddressBalance {
    let script_hash = get_script_hash_for_address(address);
    let script_hash_le = convert_big_endian_hex_to_little_endian(&script_hash);
    let balance_response = BlockchainScriptHashGetBalanceCommand::new(&script_hash_le)
        .call(&client.electrs_request_client)
        .unwrap();
    AddressBalance {
        unconfirmed: balance_response.unconfirmed,
        confirmed: balance_response.confirmed,
    }
}
pub fn get_historical_transactions_for_address(
    address: &str,
    client: &Client,
) -> Vec<HistoricalTransaction> {
    let script_hash = get_script_hash_for_address(address);
    let script_hash_le = convert_big_endian_hex_to_little_endian(&script_hash);
    let get_history_response = BlockchainScriptHashGetHistoryCommand::new(&script_hash_le)
        .call(&client.electrs_request_client)
        .unwrap();
    let historical_transactions = get_history_response
        .0
        .iter()
        .map(|historical_transaction| HistoricalTransaction {
            height: historical_transaction.height,
            tx_hash: historical_transaction.tx_hash.clone(),
        })
        .collect();
    historical_transactions
}

#[derive(Debug)]
pub struct Utxo {
    pub height: u64,
    pub tx_hash: String,
    pub tx_pos: u64,
    pub value: u64,
}
pub fn get_utxos_for_address(address: &str, client: &Client) -> Vec<Utxo> {
    let script_hash = get_script_hash_for_address(address);
    let script_hash_le = convert_big_endian_hex_to_little_endian(&script_hash);
    let list_unspent_response = BlockchainScriptHashListUnspentCommand::new(&script_hash_le)
        .call(&client.electrs_request_client)
        .unwrap();
    let utxos = list_unspent_response
        .0
        .iter()
        .map(|unspent| Utxo {
            height: unspent.height,
            tx_hash: unspent.tx_hash.clone(),
            tx_pos: unspent.tx_pos,
            value: unspent.value,
        })
        .collect();
    utxos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
