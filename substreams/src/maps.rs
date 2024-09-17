use crate::abi;
use crate::pb;
use crate::rpc;
use crate::token;
use hex_literal::hex;
use pb::auto_listing::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams::log;
use substreams_ethereum::Event;

// use substreams_entity_change::pb::entity::EntityChanges;
// use substreams_entity_change::tables::Tables;


substreams_ethereum::init!();

const AUTO_LISTING_REGISTRY_TRACKED_CONTRACT: [u8; 20] = hex!("3941ff18ff902b88b16ca8029c0d133ef262a196");

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    log::info!("Processing block number: {}", blk.number);
    
    let mut events = contract::Events::default();

    for trx in blk.transactions() {
        // Используем as_ref() для получения ссылки на Option
        if let Some(receipt) = trx.receipt.as_ref() {
            for log in &receipt.logs {
                if log.address == AUTO_LISTING_REGISTRY_TRACKED_CONTRACT {
                    process_listing_contract_updated(log, blk.timestamp_seconds(), &mut events)?;
                    process_listing_price_updated(log, blk.timestamp_seconds(), &mut events)?;
                    process_token_listed(log, blk.timestamp_seconds(), &mut events)?;
                }
            }
        }
    }

    Ok(events)
}

fn process_listing_contract_updated(log: &eth::Log, block_timestamp: u64, events: &mut contract::Events) -> Result<(), substreams::errors::Error> {
    if let Some(event) = abi::auto_listings_registry::events::ListingContractUpdated::match_and_decode(log) {
        log::info!("Decoded ListingContractUpdated event with owner: {}", Hex(&event.owner).to_string());
        let auto_listing_address = Hex(&event.autolisting).to_string();
        let name = rpc::fetch_auto_listing_name(&auto_listing_address);

        let event = contract::ListingContractUpdated {
            auto_listing: auto_listing_address,
            name,
            url: event.url,
            owner: Hex(&event.owner).to_string(),
            meta: event.metadata,
            timestamp: block_timestamp,
        };
        events.listing_contract_updateds.push(event);
    }
    Ok(())
}

fn process_listing_price_updated(log: &eth::Log, block_timestamp: u64, events: &mut contract::Events) -> Result<(), substreams::errors::Error> {
    if let Some(event) = abi::auto_listings_registry::events::ListingPrice::match_and_decode(log) {
        log::info!("Decoded ListingPrice event with auto_listing: {}", Hex(&event.autolisting).to_string());
        let auto_listing_address = Hex(&event.autolisting).to_string();
        let token_address = Hex(&event.token).to_string();
        // let fee_token = token::get_fee_token(&token_address);
        Some(match token::get_token_info(&token_address) {
            Some(token) => {
                let event = contract::ListingPrice {
                    auto_listing: auto_listing_address,
                    fee_token: Some(pb::auto_listing::v1::FeeToken {
                        address: token_address.clone(),
                        token_info: Some(token),
                    }),
                    price: event.price.to_string(),
                    timestamp: block_timestamp,
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


fn process_token_listed(log: &eth::Log, block_timestamp: u64, events: &mut contract::Events) -> Result<(), substreams::errors::Error> {
    if let Some(event) = abi::auto_listings_registry::events::TokenListed::match_and_decode(log) {
        let auto_listing_address = Hex(&event.listed_by).to_string();
        log::info!("Decoded ListingPrice event with auto_listing: {}", auto_listing_address);
        let token_address_erc20 = Hex(&event.token_erc20).to_string();
        let token_address_erc223 = Hex(&event.token_erc223).to_string();
        Some(match token::get_token_with_fallback(&token_address_erc20, &token_address_erc223) {
            Some(token) => {
                let event = contract::TokenListed {
                    timestamp: block_timestamp,
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