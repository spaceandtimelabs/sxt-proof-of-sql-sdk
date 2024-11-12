// This code constructs a payload to retrieve commitments. This data can also be retrieved here:
// https://polkadot.js.org/apps/#/chainstate
// By setting:
// selected state query: commitments
// Then for SxtCoreTablesTableIdentifier, set the following:
// name: BLOCKS
// namespace: ETHEREUM
// ProofOfSqlCommitmentMapCommitmentScheme: DynamicDory
//
// You can run this code at https://functions.chain.link/playground, it requires the following argument:
// 0xca407206ec1ab726b2636c4b145ac28749505e273536fae35330b966dac69e86a4832a125c0464e066dd20add960efb518424c4f434b5320455448455245554d4a9e6f9b8d43f6ad008f8c291929dee201
//
// This string is a hash of the storage key of the dory commitment in ETHEREUM.BLOCKS, laid out in the following manner: 
// xxHash("Commitments") + xxHash("CommitmentStorageMap" + blake2_128("ETHEREUM") + blake2_128("BLOCKS") + blake2_128(01),
//
const apiResponse = await Functions.makeHttpRequest({
    url: "https://rpc.testnet.sxt.network/",
    method: "POST",
    headers: {
        "Content-Type": "application/json"
    },
    data: {
        id: 1,
        jsonrpc: "2.0",
        method: "state_getStorage",
        params: [args[0]]
    }
});

// Return the result property from the response data as an encoded string
const truncatedResult = apiResponse.data.result.slice(0, 256);
return Functions.encodeString(truncatedResult);