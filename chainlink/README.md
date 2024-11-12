// CHAINLINK INTEGRATION

## TODO

- [ ] [upload secret](https://github.com/smartcontractkit/smart-contract-examples/blob/73ae3e1a0400fcf35c560acdf35a8f2ac77eab96/functions-examples/examples/5-use-secrets-threshold/request.js#L36) API key to chainlink
- [x] call to get bearer token
- [x] call to get the commitment for table being queried from substrate
- [ ] call to download wasm binary - initially on public endpoint, eventually will be a release artifact
- [ ] call to prover - for request to prover, once we have commitments, wasm can formulate request, requires schema m.d. and row count m.d.

## NOTES

- 10 second execution time limit
- file size limit
- wasm not supported by chainlink but deno is
- verification might be too expensive, might need to kick off proof via chainlink job to avoid time limit

## RESOURCES

https://functions.chain.link/playground
https://github.com/spaceandtimelabs/proofs/tree/davidsebek/build-wasm-lib/crates/proofs-wasm
https://github.com/spaceandtimelabs/proofs/tree/davidsebek/build-wasm-lib_v2/crates/proofs-wasm
