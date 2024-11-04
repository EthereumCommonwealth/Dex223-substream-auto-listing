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
        // StaticTokenDefinition {
        //     address: hex_literal::hex!("e0b7927c4af23765cb51314a0e0521a9645f0e2a").to_vec(),
        //     symbol: "DGD".to_string(),
        //     name: "DGD".to_string(),
        //     decimals: 9,
        //     in_converter: false,
        // }
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
