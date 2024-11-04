# Substream dex223 auto-listing

## Change network

This substream you can use only in EVM networks.

1. `./src/static_token_definition.rs` add custom values for StaticTokenDefinition.

   ```rust
   ...
   StaticTokenDefinition {
       address: hex_literal::hex!("e0b7927c4af23765cb51314a0e0521a9645f0e2a").to_vec(),
       symbol: "DGD".to_string(),
       name: "DGD".to_string(),
       decimals: 9,
   },
   ...

   ```

2. `./src/utils.rs` change address for `AUTO_LISTING_REGISTRY_TRACKED_CONTRACT` and `ADDRESS_CONVERTER`

   ```rust
    pub const AUTO_LISTING_REGISTRY_TRACKED_CONTRACT: [u8; 20] = hex!("4F55aF4162FBA4505D459d3B3Fd1926391F18349"); // eos testnet
    pub const ADDRESS_CONVERTER: [u8; 20] = hex!("Dd90b13bcb92950CA9b6b3e0407d439533eA0df2");
   ```

3. `./substreams.yaml`

   - Change all `initialBlock`. Block numbar should be block number when deploy smart contract `AUTO_LISTING_REGISTRY_TRACKED_CONTRACT` or `ADDRESS_CONVERTER` (use minimal value for all modules)
   - Change `network: <NETWORK_NAME>`, you can find actual values in pinax.network

4. Run command.

   ```bash
   make all
   ```

   You will create new `dex223-auto-listingv0.1.0.spkg`

## Test run

1.4 You can run test in dir `./substreams`

> [!NOTE] > `SUBSTREAMS_API_TOKEN` You should get token from dashbord pinax.network

```bash
export SUBSTREAMS_API_TOKEN=<SUBSTREAMS_API_TOKEN>
```

```bash
substreams run auto-listing-v0.1.0.spkg map_events -e eosevm.substreams.pinax.network:443 --start-block 48410098 --stop-block +3
```

And you watch logs
