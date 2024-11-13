const wasmRequest = await Functions.makeHttpRequest({
    url: "https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/blob/feat/wasm-bindgen/crates/proof-of-sql-sdk-wasm/pkg/sxt_proof_of_sql_sdk_wasm_bg.wasm",
    method: "GET",
    responseType: "arraybuffer",
});

// Check if the request was successful
if (wasmRequest.status === 200) {
    return Functions.encodeString("200");
} else {
    // If there was an error, return the status code as a string
    return Functions.encodeString(wasmRequest.status.toString());
}