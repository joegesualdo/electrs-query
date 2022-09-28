use electrs_query::{
    get_balance_for_address, get_historical_transactions_for_address, get_relay_fee,
    get_utxos_for_address, Client,
};

fn main() {
    let address = "127.0.0.1:50001";
    let client = Client::new(address);

    let relay_fee = get_relay_fee(&client);
    println!("relay fee result: {}", relay_fee);
    // let relay_fee_method = "blockchain.relayfee";
    //let relay_fee_response = client.raw_call(relay_fee_method, vec![]).unwrap();
    // let response = BlockchainRelayFeeCommand::new().call(&client).unwrap();
    // println!("relay fee result: {:?}", response);

    let p2pkh_address = "mv7RvNNQ7HpQf2diQai5hgpeuzkFoAQP9G".to_string();
    let address_balance = get_balance_for_address(&p2pkh_address, &client);
    println!("balance: {:#?}", address_balance);
    let address_utxos = get_utxos_for_address(&p2pkh_address, &client);
    println!("utxos: {:#?}", address_utxos);
    let address_historical_transactions =
        get_historical_transactions_for_address(&p2pkh_address, &client);
    println!(
        "historical transactions: {:#?}",
        address_historical_transactions
    );

    let p2sh_address = "2MzBNKyJjx44BDJfwEevVzS3Q9Z5kSEYUZB".to_string();
    let address_balance = get_balance_for_address(&p2sh_address, &client);
    println!("balance: {:#?}", address_balance);
    let address_utxos = get_utxos_for_address(&p2pkh_address, &client);
    println!("utxos: {:#?}", address_utxos);

    let p2wpkh_address = "tb1qphdqqxupe6dzkz3z58twy5l4s0v24mle5gkp99".to_string();
    let address_balance = get_balance_for_address(&p2wpkh_address, &client);
    println!("balance: {:#?}", address_balance);
    let address_utxos = get_utxos_for_address(&p2pkh_address, &client);
    println!("utxos: {:#?}", address_utxos);
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
