#!/bin/bash

killall forgefix-at

CARGO="cargo run -- --sender-comp-id ARCA --target-comp-id TW --addr 127.0.0.1:9823 --store /tmp/store --log ./logs"

rm -fr /tmp/store* 
rm -fr ./fix-rs/executor/store

export LD_LIBRARY_PATH=/usr/local/lib
./forgefix/executor/executor forgefix/executor/qf.ini & 
PROCID=$!

${CARGO} &
PROCID_RUST=$!

trap ctrl_c INT

function ctrl_c() {
    kill $PROCID
    kill $PROCID_RUST
}

sleep 1h

