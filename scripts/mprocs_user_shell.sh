#!/bin/bash

CASHU_RS_MINT_DIR=/tmp/cashu-rs-mint
BITCOIN_DIR=$CASHU_RS_MINT_DIR/bitcoin
LIGHTNING_DIR=$CASHU_RS_MINT_DIR/lighting

alias btc="bitcoin-cli -regtest -datadir=$BITCOIN_DIR"
alias ln1="lightning-cli --network=regtest --lightning-dir=$LIGHTNING_DIR/ln_1"
alias ln2="lightning-cli --network=regtest --lightning-dir=$LIGHTNING_DIR/ln_2"

while true; do
  blockcount=$(btc getblockcount) || { blockcount=-1; }
  if [[ $blockcount != "-1" ]]; then
    echo "bitcoind started"
    break
  fi
  echo "Waiting for bitcoind to start"
  sleep 1
done

btc loadwallet "test" || btc createwallet "test" || echo "Wallet already loaded"

address=$(btc getnewaddress)
btc generatetoaddress 101 $address

echo "Waiting for ln1 to start"
while true; do
  ln_1_info=$(ln1 getinfo) || { ln_1_info=-1; }
  if [[ $ln_1_info != "-1" ]]; then
    echo "ln1 started"
    break
  fi
  echo "Waiting for ln1 to start"
  sleep 1
done

echo "Funding ln1 onchain wallet"
ln1_address=$(ln1 newaddr | jq -r '.bech32')
btc generatetoaddress 101 $ln1_address

echo "Waiting for ln2 to start"
while true; do
  ln_2_info=$(ln2 getinfo) || { ln_2_info=-1; }
  if [[ $ln_2_info != "-1" ]]; then
    echo "ln2 started"
    break
  fi
  echo "Waiting for ln2 to start"
  sleep 1
done

echo "Funding ln2 onchain wallet"
ln2_address=$(ln2 newaddr | jq -r '.bech32')
btc generatetoaddress 101 $ln2_address

# Peer the nodes
ln1_id=$(ln1 getinfo | jq -r '.id')
ln2_id=$(ln2 getinfo | jq -r '.id')
ln2 connect $ln1_id@127.0.0.1:19846

# Open channels
ln1 fundchannel $ln2_id 1000000000
ln2 fundchannel $ln1_id 1000000000

echo """
Regtest environment ready.
Bitcoin node started, ln1 and ln2 connected, funded, and channels opened.

Aliases:
btc         - bitcoin-cli -regtest -datadir=$BITCOIN_DIR
ln1         - lightning-cli --lightning-dir=$LIGHTNING_DIR/ln_1 --regtest
ln2         - lightning-cli --lightning-dir=$LIGHTNING_DIR/ln_2 --regtest

Run any command with --help to see available options
"""
