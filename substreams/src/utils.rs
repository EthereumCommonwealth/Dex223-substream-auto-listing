use crate::abi;
use crate::pb::auto_listing::v1::Transaction;

use substreams::{hex,  Hex};
use substreams::scalar::{BigInt};

use substreams_ethereum::rpc::RpcBatch;
use substreams_ethereum::pb::eth::v2::{TransactionTrace, Log};


pub const AUTO_LISTING_REGISTRY_TRACKED_CONTRACT: [u8; 20] = hex!("4F55aF4162FBA4505D459d3B3Fd1926391F18349"); // eos testnet
pub const ADDRESS_CONVERTER: [u8; 20] = hex!("Dd90b13bcb92950CA9b6b3e0407d439533eA0df2");
pub const ADDRESS_ZERO: [u8; 20] = hex!("0000000000000000000000000000000000000000");


pub fn load_transaction(
    block_number: u64,
    timestamp: u64,
    log: &Log,
    transaction_trace: &TransactionTrace,
) -> Transaction {
    let mut transaction = Transaction {
        id: Hex(&transaction_trace.hash).to_string(),
        block_number,
        timestamp,
        gas_used: transaction_trace.gas_used,
        gas_price: Default::default(),
        from: Hex(&transaction_trace.from).to_string(),
        to: Hex(&transaction_trace.to).to_string(),
        address: Hex(&log.address).to_string(),
        log_ordinal: log.ordinal,
    };
    if let Some(gas_price) = &transaction_trace.gas_price {
        let gas_price: BigInt = BigInt::from_unsigned_bytes_be(&gas_price.bytes);
        transaction.gas_price = gas_price.to_string();
    }

    transaction
}


pub fn fetch_auto_listing_name(auto_listing_address: &String) -> String {
    let batch = RpcBatch::new();

    // Добавляем RPC-запрос для функции "name" автолистинга
    let responses = batch
        .add(abi::auto_listings::functions::GetName {}, hex::decode(auto_listing_address).unwrap())
        .execute()
        .unwrap()
        .responses;

    // Обрабатываем ответ и декодируем имя
    match RpcBatch::decode::<_, abi::auto_listings::functions::GetName>(&responses[0]) {
        Some(decoded_name) => {
            decoded_name
        }
        None => {
            "unknown".to_string()
        }
    }
}
