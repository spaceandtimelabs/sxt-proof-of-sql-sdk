#!/usr/bin/env bash

curl $1/sxt_proof_of_sql_sdk_wasm.js | cat - ./index_tail.js > ./index.js
curl $1/sxt_proof_of_sql_sdk_wasm_bg.wasm > ./sxt_proof_of_sql_sdk_wasm_bg.wasm
