#!/bin/bash

# Check if at least two arguments are provided
if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <lightning_dir> <addr>"
  exit 1
fi

# Assign the first argument as the lightning directory
LN_DIR="${CASHU_RS_MINT_DIR}/lightning/$1"
mkdir -p $LN_DIR

# Assign the second argument as the address
ADDR=$2

# Define other options, dynamically using the provided address
OPTIONS="--bitcoin-datadir=${CASHU_RS_MINT_DIR}/bitcoin --network=regtest --addr=${ADDR} --autolisten=true --log-level=debug"

# Stop lightningd if it's already running
lightning-cli --lightning-dir=$LN_DIR --regtest stop >/dev/null 2>&1

# Wait a bit to ensure lightningd stops
sleep 5

# Start lightningd with predefined options
lightningd --lightning-dir=$LN_DIR $OPTIONS
