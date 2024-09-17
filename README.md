# dex223 auto listing EOS

- This is a generated Substreams-powered-Subgraph

# Dependencies

## Get Substreams CLI (optional)

To try the Substreams directly, you need to install the `substreams CLI` (v1.7.2 or above).

You have many options as explained in this [installation guide](https://substreams.streamingfast.io/documentation/consume/installing-the-cli).

Check if `substreams` was installed successfully, you can run the following command:

```bash
substreams --version
> substreams version ...
```

## Get Substreams API Token

To try the Substreams directly or to run a local graph-node instance, you will need to get a Substreams API token.
Follow the instructions on the [authentification section](https://substreams.streamingfast.io/documentation/consume/authentication) in the `StreamingFast` documentation.

## Install Docker

To run a local `graph-node` instance, you will need to install Docker. You can do it by following the instructions on the [official Docker website](https://docs.docker.com/get-docker/).

## Install buf cli

To run the proto assembly script bindings, you will need to install the `buf` [cli](https://buf.build/docs/installation).

## Install npm and nodeJS packages

Run the following command in the `root` of the repository:

```bash
npm install
```

# Deploy a subgraph

## On a local dev environment

### Launch docker-compose environment

To deploy your subgraph locally, you need to run a local graph-node instance. To do so, export your `SUBSTREAMS_API_TOKEN` and
use the `launch-graph-node` script :

```bash
cd subgraph/graph-node && touch .env
```

add values to .env file

```bash
NETWORK=<network name>
SUBSTREAMS_ENDPOINT=https://sepolia.substreams.pinax.network:443
SUBSTREAMS_API_TOKEN=<api token>

ETH_MAINNET_RPC=<not required>
POSTGRES_DB=<db name for subgraph>
POSTGRES_USER=<db user for subgraph>
POSTGRES_PASSWORD=<db password for subgraph>
```

```bash
export $(xargs < .env)
cmod -x ./substreams-config-gen.sh && ./substreams-config-gen.sh
./start.sh
```

This script is running `docker compose` to create all necessary instances to launch properly the node locally, connecting to Streamingfast Substreams API.

### Deploy locally

Then, from another terminal in dir substream:

```bash
npm run create-local
npm run deploy-local
npm run remove-local
```

### Query a subgraph

Once you subgraph is deployed, you can query it! To do so, you can directly write your query locally on http://localhost:8000/subgraphs/name/{name_of_your_subgraph}/

### Test command

Run command in `substreams`

```bash
substreams run auto-listing-v0.1.0.spkg map_events -e sepolia.substreams.pinax.network:443 --start-block 6543110 --stop-block +3
```
