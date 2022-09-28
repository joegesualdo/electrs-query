# Electrs Query 
> Request information from an Electrs Server

This library provides helpful functions to query common information about an Electrs server.

---

**⚠️ This is experimental. Please use at your own risk.⚠️**

---

## Install
> Add package to Cargo.toml file
```rust
[dependencies]
electrs-query = "0.1.1"
```

## Usage:
```rust
use electrs_query::{get_balance_for_address, get_relay_fee, get_utxos_for_address, Client};

// setup
let address = "127.0.0.1:50001";
let client = Client::new(address);

// get realy fee
let relay_fee = get_relay_fee(&client);
println!("relay fee result: {}", relay_fee);

let p2pkh_address = "mv7RvNNQ7HpQf2diQai5hgpeuzkFoAQP9G".to_string();
// get balance for p2pkh address
let address_balance = get_balance_for_address(&p2pkh_address, &client);
println!("balance: {:#?}", address_balance);
// get utxo for p2pkh address
let address_utxos = get_utxos_for_address(&p2pkh_address, &client);
println!("utxos: {:#?}", address_utxos);
// get historical transactions for p2pkh address
let historical_transactions = get_historical_transactions_for_address(&p2pkh_address, &client);
println!("historical transactions: {:#?}", historical_transactions);

let p2sh_address = "2MzBNKyJjx44BDJfwEevVzS3Q9Z5kSEYUZB".to_string();
// get balance for p2sh address
let address_balance = get_balance_for_address(&p2sh_address, &client);
println!("balance: {:#?}", address_balance);
// get utxos for p2sh address
let address_utxos = get_utxos_for_address(&p2pkh_address, &client);
println!("utxos: {:#?}", address_utxos);
// get historical transactions for p2sh address
let historical_transactions = get_historical_transactions_for_address(&p2sh_address, &client);
println!("historical transactions: {:#?}", historical_transactions);

let p2wpkh_address = "tb1qphdqqxupe6dzkz3z58twy5l4s0v24mle5gkp99".to_string();
// get balance for p2wpkh address
let address_balance = get_balance_for_address(&p2wpkh_address, &client);
println!("balance: {:#?}", address_balance);
// get utxos for p2wpkh address
let address_utxos = get_utxos_for_address(&p2pkh_address, &client);
println!("utxos: {:#?}", address_utxos);
// get historical transactions for p2wpkh address
let historical_transactions = get_historical_transactions_for_address(&p2wpkh_address, &client);
println!("historical transactions: {:#?}", historical_transactions);
```

## API
Find a list of all the functions available in the [documentation](https://docs.rs/electrs-query/latest/bitcoin_node_query/)

## Related
- [electrs-request](https://github.com/joegesualdo/electrs-request) - Type-safe wrapper around electrs RPC commands
- [bitcoin-node-query](https://github.com/joegesualdo/bitcoin-node-query) - Query Bitcoin Node for information
- [bitcoind-request](https://github.com/joegesualdo/bitcoind-request) - Type-safe wrapper around bitcoind RPC commands
- [bitcoin-terminal-dashboard](https://github.com/joegesualdo/bitcoin-terminal-dashboard) - Bitcoin Dashboard in the terminal

## License
MIT © [Joe Gesualdo]()
