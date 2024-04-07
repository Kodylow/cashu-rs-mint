#!/bin/bash

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 cashu_rs_mint_dir"
  exit 1
fi

CASHU_RS_MINT_DIR=$1
BITCOIN_DIR=${CASHU_RS_MINT_DIR}/bitcoin
LIGHTNING_DIR=${CASHU_RS_MINT_DIR}/lightning

mkdir -p ${CASHU_RS_MINT_DIR}
mkdir -p ${BITCOIN_DIR}
mkdir -p ${LIGHTNING_DIR}

alias btc="bitcoin-cli -regtest -datadir=$BITCOIN_DIR"
alias ln1="lightning-cli --lightning-dir=$LIGHTNING_DIR/ln_1 --regtest"
alias ln2="lightning-cli --lightning-dir=$LIGHTNING_DIR/ln_2 --regtest"

blockcount=$(btc getblockcount) || { blockcount=-1; }
if [[ $blockcount == "-1" ]]; then
  echo "Starting bitcoind"
  bitcoind -regtest -datadir=${BITCOIN_DIR} -fallbackfee=0.01 -daemon
  sleep 1
else
  echo "bitcoind already started"
fi

btc loadwallet "test" || btc createwallet "test" || echo "Wallet already loaded"

address=$(btc getnewaddress)
btc generatetoaddress 101 $address

ln_1_info=$(ln1 getinfo) || { ln_1_info=-1; }

if [[ $ln_1_info == "-1" ]]; then
  echo "Starting ln1"
  lightningd --bitcoin-datadir=${bitcoin_dir} --network=regtest --lightning-dir=${lightning_dir}/ln_1 --addr=127.0.0.1:19846 --autolisten=true --log-level=debug --log-file=./debug.log --daemon
  sleep 1
else
  echo "ln1 already started"
fi

ln_2_info=$(ln2 getinfo) || { ln_2_info=-1; }
if [[ $ln_2_info == "-1" ]]; then
  echo "Starting ln2"
  lightningd --bitcoin-datadir=${bitcoin_dir} --network=regtest --lightning-dir=${lightning_dir}/ln_2 --addr=127.0.0.1:80888 --autolisten=true --log-level=debug --log-file=./debug.log --daemon
  sleep 1
else
  echo "ln2 already started"
fi

echo """
Aliases:
btc         - bitcoin-cli -regtest -datadir=$BITCOIN_DIR
ln1         - lightning-cli --lightning-dir=$LIGHTNING_DIR/ln_1 --regtest
ln2         - lightning-cli --lightning-dir=$LIGHTNING_DIR/ln_2 --regtest

Example usage (mine 101 blocks paying coinbase tx output to ln1's onchain wallet):
$ btc generatetoaddress 101 $(ln1 getnewaddress)

Run any command with --help to see available options
"""
