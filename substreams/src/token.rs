use crate::utils;
use crate::abi;
use crate::pb;


pub fn get_token_info(token_address: &str) -> Option<pb::auto_listing::v1::TokenInfo> {
    let token = hex::decode(token_address).unwrap();
    let converter_address = utils::ADDRESS_CONVERTER.to_vec();  // Адрес конвертера
    let batch = substreams_ethereum::rpc::RpcBatch::new();
    if token == utils::ADDRESS_ZERO.to_vec() {
        substreams::log::debug!("Token address is zero");
        return Some(pb::auto_listing::v1::TokenInfo {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            in_converter: false,
        });
    }
    let responses = batch
        .add(
            abi::erc20_erc223::functions::Decimals {},
            token.clone(),
        )
        .add(
            abi::erc20_erc223::functions::Name {},
            token.clone(),
        )
        .add(
            abi::erc20_erc223::functions::Symbol {},
            token.clone(),
        ).add(
            abi::token_converter::functions::GetErc223WrapperFor { token: token.clone() }, converter_address.clone())
        .add(
            abi::token_converter::functions::GetErc20WrapperFor { token: token.clone() }, converter_address.clone())
        .execute()
        .unwrap()
        .responses;

    let decimals: u64;
    let mut name: String = "unknown".to_string();
    let mut symbol: String = "unknown".to_string();
    let mut in_converter: bool = false;

    match substreams_ethereum::rpc::RpcBatch::decode::<_, abi::erc20_erc223::functions::Decimals>(&responses[0]) {
        Some(decoded_decimals) => {
            decimals = decoded_decimals.to_u64();
            substreams::log::debug!("decoded_decimals ok: {}", decimals);
        }
        None => {
            substreams::log::debug!("failed to get decimals");
            return None;
        }
    };

    match substreams_ethereum::rpc::RpcBatch::decode::<_, abi::erc20_erc223::functions::Name>(&responses[1]) {
        Some(decoded_name) => {
            name = decoded_name;
            substreams::log::debug!("decoded_name ok: {}", name);
        }
        None => {
            substreams::log::debug!("failed to get name");
        }
    };

    match substreams_ethereum::rpc::RpcBatch::decode::<_, abi::erc20_erc223::functions::Symbol>(&responses[2]) {
        Some(decoded_symbol) => {
            symbol = decoded_symbol;
            substreams::log::debug!("decoded_symbol ok: {}", symbol);
        }
        None => {
            substreams::log::debug!("failed to get symbol");
        }
    };
    match substreams_ethereum::rpc::RpcBatch::decode::<_, abi::token_converter::functions::GetErc223WrapperFor>(&responses[3]) {
        Some(erc223_result) => {
            if erc223_result.0 != utils::ADDRESS_ZERO.to_vec() {
                in_converter = true;
                substreams::log::info!("Found ERC223 wrapper for token: {}", token_address);
            }
        }
        None => {
            substreams::log::info!("Failed to get ERC223");
            match substreams_ethereum::rpc::RpcBatch::decode::<_, abi::token_converter::functions::GetErc20WrapperFor>(&responses[4]) {
                Some(erc20_result) => {
                    if erc20_result.0 != utils::ADDRESS_ZERO.to_vec() {
                        in_converter = true;
                        substreams::log::info!("Found ERC20 wrapper for token: {}", token_address);
                    }
                }
                None => {
                    substreams::log::info!("Failed to get ERC20");
                }
            };
        }
    };

    substreams::log::info!("Token in converter: {}", in_converter);
    Some(pb::auto_listing::v1::TokenInfo {
        name,
        symbol,
        decimals,
        in_converter,
    })
}


pub fn get_token_with_fallback(token_address_erc20: &str, token_address_erc223: &str) -> Option<pb::auto_listing::v1::Token> {
    // Сначала пробуем получить информацию по адресу ERC20
    if let Some(token_info) = get_token_info(token_address_erc20) {
        return Some(pb::auto_listing::v1::Token {
            address_erc20: token_address_erc20.to_string(),
            address_erc223: token_address_erc223.to_string(),
            token_info: Some(token_info) // Оберните в Some
        });
    }
    
    // Если информация по ERC20 не найдена, пробуем для адреса ERC223
    if let Some(token_info) = get_token_info(token_address_erc223) {
        return Some(pb::auto_listing::v1::Token {
            address_erc20: token_address_erc20.to_string(),
            address_erc223: token_address_erc223.to_string(),
            token_info: Some(token_info) // Оберните в Some
        });
    }
    
    // Если информации нет ни по одному из адресов, возвращаем None
    None
}