#!/bin/bash

# Define the data directory and other options
DATADIR="${CASHU_RS_MINT_DIR}/bitcoin"
OPTIONS="-regtest -fallbackfee=0.01"

# Stop bitcoind if it's already running
bitcoin-cli -regtest -datadir=$DATADIR stop >/dev/null 2>&1

# Wait a bit to ensure bitcoind stops
sleep 5

# Start bitcoind with predefined options
bitcoind -regtest -datadir=$DATADIR $OPTIONS
