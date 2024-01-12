#!/bin/bash

gcc -o fix_client fix_client.c -lforgefix_c -L ./target/debug/ \
    && LD_LIBRARY_PATH=./target/debug/ ./fix_client $1 $2 \
    && rm fix_client
