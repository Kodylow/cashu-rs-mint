dev:
  export CASHU_RS_MINT=/tmp/cashu-rs-mint; mprocs -c mprocs.yaml

regtest:
  ./scripts/regtest_mint.sh /tmp/cashu-rs-mint
