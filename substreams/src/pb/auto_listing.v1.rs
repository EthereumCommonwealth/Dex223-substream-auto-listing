// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(uint64, tag="3")]
    pub timestamp: u64,
    #[prost(uint64, tag="4")]
    pub gas_used: u64,
    #[prost(string, tag="5")]
    pub gas_price: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub log_ordinal: u64,
    #[prost(string, tag="7")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub to: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenInfo {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub decimals: u64,
    #[prost(bool, tag="4")]
    pub in_converter: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub address_erc20: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address_erc223: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub token_info: ::core::option::Option<TokenInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeToken {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub token_info: ::core::option::Option<TokenInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20WrapperCreatedEvent {
    #[prost(message, optional, tag="1")]
    pub tx: ::core::option::Option<Transaction>,
    #[prost(string, tag="2")]
    pub address_erc20: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub address_erc223: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc223WrapperCreatedEvent {
    #[prost(message, optional, tag="1")]
    pub tx: ::core::option::Option<Transaction>,
    #[prost(string, tag="2")]
    pub address_erc20: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub address_erc223: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub listing_contract_updateds: ::prost::alloc::vec::Vec<ListingContractUpdated>,
    #[prost(message, repeated, tag="2")]
    pub listing_prices: ::prost::alloc::vec::Vec<ListingPrice>,
    #[prost(message, repeated, tag="3")]
    pub token_listeds: ::prost::alloc::vec::Vec<TokenListed>,
    #[prost(message, repeated, tag="4")]
    pub erc223_to_erc20: ::prost::alloc::vec::Vec<Erc20WrapperCreatedEvent>,
    #[prost(message, repeated, tag="5")]
    pub erc20_to_erc223: ::prost::alloc::vec::Vec<Erc223WrapperCreatedEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingContractUpdated {
    #[prost(string, tag="1")]
    pub auto_listing: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub meta: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="6")]
    pub timestamp: u64,
    #[prost(message, optional, tag="7")]
    pub tx: ::core::option::Option<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingPrice {
    #[prost(uint64, tag="1")]
    pub timestamp: u64,
    #[prost(string, tag="2")]
    pub auto_listing: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub fee_token: ::core::option::Option<FeeToken>,
    #[prost(string, tag="4")]
    pub price: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub tx: ::core::option::Option<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenListed {
    #[prost(uint64, tag="1")]
    pub timestamp: u64,
    #[prost(string, tag="2")]
    pub auto_listing: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub token: ::core::option::Option<Token>,
    #[prost(message, optional, tag="4")]
    pub tx: ::core::option::Option<Transaction>,
}
// @@protoc_insertion_point(module)
