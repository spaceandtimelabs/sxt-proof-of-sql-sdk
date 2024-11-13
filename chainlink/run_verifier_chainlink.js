const wasmRequest = await Functions.makeHttpRequest({
    url: "https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/blob/feat/wasm-bindgen/crates/proof-of-sql-sdk-wasm/pkg/sxt_proof_of_sql_sdk_wasm_bg.wasm",
    method: "GET",
    responseType: "arraybuffer",
});

if (wasmRequest.error || wasmRequest.status !== 200) {
    throw new Error("Error retrieving wasm file");
}

const wasmCode = new Uint8Array(wasmRequest.data);

const wasmInstance = (await WebAssembly.instantiate(wasmCode, imports)).instance;
const wasm = wasmInstance.exports;

const inputsRequest = await Functions.makeHttpRequest({
    url: "https://lavender-wrong-mastodon-391.mypinata.cloud/ipfs/QmP2njFmbxKRgamGSSNCXK8RzRRJjqMz9wDRQuVhTNiaBE",
    method: "GET",
    responseType: "json",
});

if (inputsRequest.error || inputsRequest.status !== 200) {
    throw new Error("Error retrieving inputs file");
}

const { query_input, schema_input, query_commitments_input, proof_input, serialized_result_input, dory_verifier_setup } = inputsRequest.data;
const convertBase64 = base64 => Uint8Array.from(atob(base64), c => c.charCodeAt(0));

let ret = verify(
    111,
    convertBase64(query_input),
    convertBase64(schema_input),
    convertBase64(query_commitments_input),
    convertBase64(proof_input),
    convertBase64(serialized_result_input),
    convertBase64(dory_verifier_setup)
);

const time_end = Date.now();
return Functions.encodeUint256(time_end - time_start);