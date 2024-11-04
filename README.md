# Substream dex223-auto-listing

## Install tools

1. Install substream https://docs.substreams.dev/documentation/consume/installing-the-cli
2. Create account in https://app.pinax.network/ and you have permision geting SUBSTREAMS_API_TOKEN="<SUBSTREAMS_API_TOKEN>"
3. Install docker for up graph-node (only local)
4. Run all instruction from `./subgraph/README.md` and `./substreams/README.md`

### Graph-node local

1. Open new terminal in `./graph-node` and create `.env` file, check `.env.example`
2. Run command

   ```bash
   export $(xargs < .env)
   bash ./substreams-config-gen.sh
   docker-compose -f docker-compose.yml up -d
   ```

3. Open `./subgraph` in other terminal

   ```bash
   yarn create-local
   yarn deploy-local
   ```

### Publish to TheGraph

0. `./subgrpah`
1. Run command

   ```bash
   graph publish
   ```

   You see link `https://cli.thegraph.com/....` open this link input value in form and click `Deploy`

2. If you want update

   ```bash
   graph publish --subgraph-id <SUBGRAPH ID>
   ```
