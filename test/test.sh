#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

TEST_DIR="/tmp/forgefix-test"
if ! [ -d $TEST_DIR ]; then
    mkdir $TEST_DIR
fi

TEST_NUM="$(date +'%Y%m%d-%H:%M:%S')"
SCRIPT_DIR=$(pwd)
TARGET_DIR=${SCRIPT_DIR}/../target

cargo build --all --target-dir $TARGET_DIR
RES=$?
if [ $RES -ne 0 ]; then
    echo -e "${RED}TEST FAILED${NC}: Failed to build ForgeFIX..."
    exit $RES
fi

export FORGE_FIX_AT_RUST=${TARGET_DIR}/debug/forgefix-at
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${TARGET_DIR}/debug
export FORGE_FIX_AT_C=${TARGET_DIR}/debug/forgefix-c-at
export FORGE_FIX_AT_OUT="${SCRIPT_DIR}/output/forgefix-test-${TEST_NUM}"

mkdir -p $FORGE_FIX_AT_OUT

if ! [ -f ${TEST_DIR}/quickfix.diff ]; then 
    unzip quickfix-diff.zip -d ${TEST_DIR}
fi

cd $TEST_DIR

if ! [ -d ./quickfix ]; then 
    git clone -b v.1.14.4 https://github.com/quickfix/quickfix.git
    RES=$?
    if [ $RES -ne 0 ]; then
        echo -e "${RED}TEST FAILED${NC}: Failed to clone quickfix..."
        exit $RES
    fi
fi

cd quickfix

if ! [ -f ./test/test.sh ]; then 
    git apply ../quickfix.diff
    if [ $RES -ne 0 ]; then
        echo -e "${RED}TEST FAILED${NC}: Failed to apply diff to quickfix..."
        exit $RES
    fi
fi

cd test

./test.sh 9000
RES=$?
if [ $RES -ne 0 ]; then 
    echo -e "${RED}TEST FAILED${NC}: Check output above and investigate output in ${FORGE_FIX_AT_OUT}"
    exit $RES
fi
    
echo -e "${GREEN}TEST SUCCESSFUL${NC}: Feel free to remove output files in ${FORGE_FIX_AT_OUT}"

cd ${SCRIPT_DIR}
rm -rf ${TEST_DIR}

exit 0
