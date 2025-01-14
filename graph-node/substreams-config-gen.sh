#!/usr/bin/env bash
echo "Generating substreams generated-config.toml"
export $(xargs < .env)

export NETWORK=${NETWORK}
export SUBSTREAMS_ENDPOINT=${SUBSTREAMS_ENDPOINT}
export SUBSTREAMS_API_TOKEN=${SUBSTREAMS_API_TOKEN}
export POSTGRES_DB=${POSTGRES_DB}
export POSTGRES_USER=${POSTGRES_USER}
export POSTGRES_PASSWORD=${POSTGRES_PASSWORD}

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

envsubst < ${ROOT}/config/substreams-generic.template.toml > ${ROOT}/generated-config.toml

echo "A new config.toml has been generated with the following values: 
NETWORK=${NETWORK}
SUBSTREAMS_ENDPOINT=${SUBSTREAMS_ENDPOINT}
SUBSTREAMS_API_TOKEN=${SUBSTREAMS_API_TOKEN}
POSTGRES_DB=${POSTGRES_DB}
POSTGRES_USER=${POSTGRES_USER}
POSTGRES_PASSWORD=${POSTGRES_PASSWORD}"
