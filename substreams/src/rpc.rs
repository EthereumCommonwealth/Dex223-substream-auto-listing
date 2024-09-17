use crate::abi;
use substreams_ethereum::rpc::RpcBatch;
use substreams::log;

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
            log::info!("Decoded name from contract: {}", decoded_name);
            decoded_name
        }
        None => {
            log::info!("Failed to fetch name for address: {}", auto_listing_address);
            "unknown".to_string()
        }
    }
}

// pub fetch_token_name(auto_listing_address: &String) -> String {
//     let batch = RpcBatch::new();

//     // Добавляем RPC-запрос для функции "name" токена
//     let responses = batch
//         .add(abi::erc20_erc223::functions::name {}, hex::decode(auto_listing_address).unwrap())
//         .execute()
//         .unwrap()
//         .responses;

//     // Обрабатываем ответ и декодируем имя
//     match RpcBatch::decode::<_, abi::erc20::functions::GetName>(&responses[0]) {
//         Some(decoded_name) => {
//             log::info!("Decoded name from contract: {}", decoded_name);
//             decoded_name
//         }
//         None => {
//             log::info!("Failed to fetch name for address: {}", auto_listing_address);
//             "unknown".to_string()
//         }
//     }
// }