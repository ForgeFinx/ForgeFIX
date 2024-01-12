#!/bin/bash

if [ -z "$(ls -A ./quickfix)" ]; then
    git submodule init
    git submodule update 
fi

if [ ! -f quickfix/test/test.sh ]; then
    unzip quickfix-diff.zip
    cd quickfix
    git apply ../quickfix.diff
    cd ..
    rm quickfix.diff
fi

cd quickfix/test
./test.sh 9000
RES=$?
if [ $RES -ne 0 ]; then
    echo "Tests failed. Inspect quickfix/test/fix-engine-output.txt and quickfix/test/log."
    exit $RES
fi

cd ../..
git submodule deinit -f --all
