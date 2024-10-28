#[derive(Clone)]
pub struct StaticTokenDefinition {
    pub address: Vec<u8>,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub in_converter: bool,
}

pub fn get_static_token_definitions() -> Vec<StaticTokenDefinition> {
    vec![
        StaticTokenDefinition {
            address: hex_literal::hex!("e0b7927c4af23765cb51314a0e0521a9645f0e2a").to_vec(),
            symbol: "DGD".to_string(),
            name: "DGD".to_string(),
            decimals: 9,
            in_converter: false,
        },
        StaticTokenDefinition {
            address: hex_literal::hex!("7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9").to_vec(),
            symbol: "AAVE".to_string(),
            name: "Aave Token".to_string(),
            decimals: 18,
            in_converter: false,
        },
        StaticTokenDefinition {
            address: hex_literal::hex!("eb9951021698b42e4399f9cbb6267aa35f82d59d").to_vec(),
            symbol: "LIF".to_string(),
            name: "Lif".to_string(),
            decimals: 18,
            in_converter: false,
        },
        StaticTokenDefinition {
            address: hex_literal::hex!("bdeb4b83251fb146687fa19d1c660f99411eefe3").to_vec(),
            symbol: "SVD".to_string(),
            name: "savedroid".to_string(),
            decimals: 18,
            in_converter: false,
        },
        StaticTokenDefinition {
            address: hex_literal::hex!("bb9bc244d798123fde783fcc1c72d3bb8c189413").to_vec(),
            symbol: "TheDAO".to_string(),
            name: "TheDAO".to_string(),
            decimals: 16,
            in_converter: false,
        },
        StaticTokenDefinition {
            address: hex_literal::hex!("38c6a68304cdefb9bec48bbfaaba5c5b47818bb2").to_vec(),
            symbol: "HPB".to_string(),
            name: "HPBCoin".to_string(),
            decimals: 18,
            in_converter: false,
        },
    ]
}

pub fn get_static_definition(
    token_address: &[u8],
    static_definitions: &[StaticTokenDefinition],
) -> Option<StaticTokenDefinition> {
    for def in static_definitions {
        if def.address == token_address {
            return Some(def.clone());
        }
    }
    None
}
