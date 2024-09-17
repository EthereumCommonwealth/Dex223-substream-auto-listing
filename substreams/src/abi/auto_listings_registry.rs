const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
/// Contract's functions.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod functions {
    use super::INTERNAL_ERR;
    #[derive(Debug, Clone, PartialEq)]
    pub struct RecordListing {
        pub token_erc20: Vec<u8>,
        pub token_erc223: Vec<u8>,
    }
    impl RecordListing {
        const METHOD_ID: [u8; 4] = [233u8, 19u8, 61u8, 137u8];
        pub fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                    &[ethabi::ParamType::Address, ethabi::ParamType::Address],
                    maybe_data.unwrap(),
                )
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                token_erc20: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                token_erc223: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(
                &[
                    ethabi::Token::Address(
                        ethabi::Address::from_slice(&self.token_erc20),
                    ),
                    ethabi::Token::Address(
                        ethabi::Address::from_slice(&self.token_erc223),
                    ),
                ],
            );
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<bool, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<bool, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(
                values
                    .pop()
                    .expect("one output data should have existed")
                    .into_bool()
                    .expect(INTERNAL_ERR),
            )
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<bool> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr : address, data : self.encode(), }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME, err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for RecordListing {
        const NAME: &'static str = "recordListing";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<bool> for RecordListing {
        fn output(data: &[u8]) -> Result<bool, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct UpdateContractInfo {
        pub owner: Vec<u8>,
        pub url: String,
        pub metadata: Vec<u8>,
    }
    impl UpdateContractInfo {
        const METHOD_ID: [u8; 4] = [231u8, 200u8, 58u8, 68u8];
        pub fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                    &[
                        ethabi::ParamType::Address,
                        ethabi::ParamType::String,
                        ethabi::ParamType::Bytes,
                    ],
                    maybe_data.unwrap(),
                )
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                owner: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                url: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
                metadata: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bytes()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(
                &[
                    ethabi::Token::Address(ethabi::Address::from_slice(&self.owner)),
                    ethabi::Token::String(self.url.clone()),
                    ethabi::Token::Bytes(self.metadata.clone()),
                ],
            );
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<bool, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<bool, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(
                values
                    .pop()
                    .expect("one output data should have existed")
                    .into_bool()
                    .expect(INTERNAL_ERR),
            )
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<bool> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr : address, data : self.encode(), }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME, err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for UpdateContractInfo {
        const NAME: &'static str = "updateContractInfo";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<bool> for UpdateContractInfo {
        fn output(data: &[u8]) -> Result<bool, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct UpdateListingPrice {
        pub token: Vec<u8>,
        pub price: substreams::scalar::BigInt,
    }
    impl UpdateListingPrice {
        const METHOD_ID: [u8; 4] = [110u8, 71u8, 42u8, 236u8];
        pub fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                    &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                    maybe_data.unwrap(),
                )
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                token: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                price: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(
                &[
                    ethabi::Token::Address(ethabi::Address::from_slice(&self.token)),
                    ethabi::Token::Uint(
                        ethabi::Uint::from_big_endian(
                            match self.price.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                    panic!("negative numbers are not supported")
                                }
                            }
                                .as_slice(),
                        ),
                    ),
                ],
            );
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<bool, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<bool, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(
                values
                    .pop()
                    .expect("one output data should have existed")
                    .into_bool()
                    .expect(INTERNAL_ERR),
            )
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<bool> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr : address, data : self.encode(), }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME, err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for UpdateListingPrice {
        const NAME: &'static str = "updateListingPrice";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<bool> for UpdateListingPrice {
        fn output(data: &[u8]) -> Result<bool, String> {
            Self::output(data)
        }
    }
}
/// Contract's events.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod events {
    use super::INTERNAL_ERR;
    #[derive(Debug, Clone, PartialEq)]
    pub struct ListingContractUpdated {
        pub autolisting: Vec<u8>,
        pub owner: Vec<u8>,
        pub url: String,
        pub metadata: Vec<u8>,
    }
    impl ListingContractUpdated {
        const TOPIC_ID: [u8; 32] = [
            91u8,
            205u8,
            189u8,
            86u8,
            251u8,
            51u8,
            205u8,
            196u8,
            186u8,
            242u8,
            89u8,
            139u8,
            90u8,
            244u8,
            14u8,
            81u8,
            54u8,
            138u8,
            40u8,
            138u8,
            96u8,
            96u8,
            0u8,
            183u8,
            165u8,
            15u8,
            171u8,
            223u8,
            172u8,
            173u8,
            251u8,
            155u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 2usize {
                return false;
            }
            if log.data.len() < 160usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(
            log: &substreams_ethereum::pb::eth::v2::Log,
        ) -> Result<Self, String> {
            let mut values = ethabi::decode(
                    &[
                        ethabi::ParamType::Address,
                        ethabi::ParamType::String,
                        ethabi::ParamType::Bytes,
                    ],
                    log.data.as_ref(),
                )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                autolisting: ethabi::decode(
                        &[ethabi::ParamType::Address],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'autolisting' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                owner: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                url: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
                metadata: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bytes()
                    .expect(INTERNAL_ERR),
            })
        }
    }
    impl substreams_ethereum::Event for ListingContractUpdated {
        const NAME: &'static str = "ListingContractUpdated";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct ListingPrice {
        pub autolisting: Vec<u8>,
        pub token: Vec<u8>,
        pub price: substreams::scalar::BigInt,
    }
    impl ListingPrice {
        const TOPIC_ID: [u8; 32] = [
            117u8,
            69u8,
            148u8,
            33u8,
            198u8,
            17u8,
            124u8,
            125u8,
            58u8,
            6u8,
            23u8,
            109u8,
            195u8,
            196u8,
            253u8,
            69u8,
            178u8,
            136u8,
            5u8,
            187u8,
            27u8,
            127u8,
            39u8,
            250u8,
            96u8,
            178u8,
            99u8,
            122u8,
            235u8,
            31u8,
            157u8,
            68u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 3usize {
                return false;
            }
            if log.data.len() != 32usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(
            log: &substreams_ethereum::pb::eth::v2::Log,
        ) -> Result<Self, String> {
            let mut values = ethabi::decode(
                    &[ethabi::ParamType::Uint(256usize)],
                    log.data.as_ref(),
                )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                autolisting: ethabi::decode(
                        &[ethabi::ParamType::Address],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'autolisting' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                token: ethabi::decode(
                        &[ethabi::ParamType::Address],
                        log.topics[2usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'token' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                price: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
    }
    impl substreams_ethereum::Event for ListingPrice {
        const NAME: &'static str = "ListingPrice";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct TokenListed {
        pub listed_by: Vec<u8>,
        pub token_erc20: Vec<u8>,
        pub token_erc223: Vec<u8>,
    }
    impl TokenListed {
        const TOPIC_ID: [u8; 32] = [
            139u8,
            241u8,
            60u8,
            206u8,
            133u8,
            211u8,
            170u8,
            24u8,
            164u8,
            183u8,
            35u8,
            108u8,
            216u8,
            133u8,
            46u8,
            163u8,
            220u8,
            121u8,
            163u8,
            220u8,
            252u8,
            133u8,
            132u8,
            155u8,
            61u8,
            185u8,
            230u8,
            124u8,
            56u8,
            108u8,
            102u8,
            102u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 4usize {
                return false;
            }
            if log.data.len() != 0usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(
            log: &substreams_ethereum::pb::eth::v2::Log,
        ) -> Result<Self, String> {
            Ok(Self {
                listed_by: ethabi::decode(
                        &[ethabi::ParamType::Address],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'listed_by' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                token_erc20: ethabi::decode(
                        &[ethabi::ParamType::Address],
                        log.topics[2usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'token_erc20' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                token_erc223: ethabi::decode(
                        &[ethabi::ParamType::Address],
                        log.topics[3usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'token_erc223' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
    }
    impl substreams_ethereum::Event for TokenListed {
        const NAME: &'static str = "TokenListed";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
}