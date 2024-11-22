#!/usr/bin/env bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cat $1/sxt_proof_of_sql_sdk_wasm.js $SCRIPT_DIR/index_tail.js > $SCRIPT_DIR/index.js
cp $1/sxt_proof_of_sql_sdk_wasm_bg.wasm $SCRIPT_DIR
