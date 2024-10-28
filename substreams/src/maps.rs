use crate::abi;
use crate::pb;
use crate::token;
use crate::utils;

use pb::auto_listing::v1 as contract;
use substreams::Hex;
use substreams::log;
use substreams_ethereum::Event;
use substreams_ethereum::pb::eth::v2 as eth;


substreams_ethereum::init!();


#[substreams::handlers::map]
fn map_events(block: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    for trx in block.transactions() {
        // Используем as_ref() для получения ссылки на Option
        if let Some(receipt) = trx.receipt.as_ref() {
            for log in &receipt.logs {
                let block_number = block.number;
                let block_timestamp = block
                    .header
                    .as_ref()
                    .unwrap()
                    .timestamp
                    .as_ref()
                    .unwrap()
                    .seconds as u64;

                let transaction = utils::load_transaction(block_number, block_timestamp, &log, &trx);
                if log.address == utils::AUTO_LISTING_REGISTRY_TRACKED_CONTRACT.to_vec() {
                    process_listing_contract_updated(log, transaction.clone(), &mut events)?;
                    process_listing_price_updated(log, transaction.clone(), &mut events)?;
                    process_token_listed(log, transaction.clone(), &mut events)?;
                }
                if log.address == utils::ADDRESS_CONVERTER.to_vec() {
                    process_token_convert(log, transaction.clone(), &mut events)?;
                }
            }
        }
    }

    Ok(events)
}

fn process_listing_contract_updated(log: &eth::Log, tx: contract::Transaction, events: &mut contract::Events) -> Result<(), substreams::errors::Error> {
    if let Some(event) = abi::auto_listings_registry::events::ListingContractUpdated::match_and_decode(log) {
        let auto_listing_address = Hex(&event.autolisting).to_string();
        let name = utils::fetch_auto_listing_name(&auto_listing_address);

        let event = contract::ListingContractUpdated {
            tx: Some(tx.clone()),
            auto_listing: auto_listing_address,
            name: name,
            url: event.url,
            owner: Hex(&event.owner).to_string(),
            meta: event.metadata,
            timestamp: tx.timestamp,
        };
        events.listing_contract_updateds.push(event);
    }
    Ok(())
}

fn process_listing_price_updated(log: &eth::Log, tx: contract::Transaction, events: &mut contract::Events) -> Result<(), substreams::errors::Error> {
    if let Some(event) = abi::auto_listings_registry::events::ListingPrice::match_and_decode(log) {
        let auto_listing_address = Hex(&event.autolisting).to_string();
        let token_address = Hex(&event.token).to_string();
        // let fee_token = token::get_fee_token(&token_address);
        Some(match token::get_token_info_by_address(&token_address) {
            Some(token_info) => {
                let event = contract::ListingPrice {
                    tx: Some(tx.clone()),
                    auto_listing: auto_listing_address,
                    fee_token: Some(pb::auto_listing::v1::FeeToken {
                        address: token_address.clone(),
                        token_info: Some(token_info),
                    }),
                    price: event.price.to_string(),
                    timestamp: tx.timestamp,
                };
                events.listing_prices.push(event);
            }
            None => {
                    log::info!("Ignoring creation of pool addr: {}", auto_listing_address);
                }
            });
    }
    Ok(())
}


fn process_token_listed(log: &eth::Log, tx: contract::Transaction, events: &mut contract::Events) -> Result<(), substreams::errors::Error> {
    if let Some(event) = abi::auto_listings_registry::events::TokenListed::match_and_decode(log) {
        let auto_listing_address = Hex(&event.listed_by).to_string();
        let token_address_erc20 = Hex(&event.token_erc20).to_string();
        let token_address_erc223 = Hex(&event.token_erc223).to_string();
        Some(match token::get_token_with_fallback(&token_address_erc20, &token_address_erc223) {
            Some(token) => {
                let event = contract::TokenListed {
                    tx: Some(tx.clone()),
                    timestamp: tx.timestamp,
                    auto_listing: auto_listing_address,
                    token: Some(token),
                };
                events.token_listeds.push(event);
            }
            None => {
                    log::info!("Ignoring creation of pool addr: {}", auto_listing_address);
                }
            });
    }
    Ok(())
}

fn process_token_convert(log: &eth::Log, tx: contract::Transaction, events: &mut contract::Events) -> Result<(), substreams::errors::Error> {   
    if let Some(event) = abi::token_converter::events::Erc20WrapperCreated::match_and_decode(log) {
        events.erc223_to_erc20.push(contract::Erc20WrapperCreatedEvent {
            tx: Some(tx.clone()),
            address_erc20: hex::encode(&event.erc20_wrapper),
            address_erc223: hex::encode(&event.token),
        });

    }
    if let Some(event) = abi::token_converter::events::Erc223WrapperCreated::match_and_decode(log) {
        events.erc20_to_erc223.push(contract::Erc223WrapperCreatedEvent {
            tx: Some(tx.clone()),
            address_erc20: hex::encode(&event.token),
            address_erc223: hex::encode(&event.erc223_wrapper),
        });     
    }
    Ok(())
}
