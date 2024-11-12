// This code constructs a payload to fetch an accesstoken to be used for
// api access to the prover. It is a required component of the payload to 
// receive a proof.
// This code can be run at https://functions.chain.link/playground
// Set the secrets field to an apiKey that you own for your sxt account.

// Ensure the API key is available
if (!secrets.apiKey) {
    throw Error("API Key Not Found");
}

// Execute the API request using Functions.makeHttpRequest
const apiResponse = await Functions.makeHttpRequest({
    url: "https://proxy.api.spaceandtime.dev/auth/apikey",
    method: "POST",
    headers: {
        "apikey": secrets.apiKey,
        "Content-Type": "application/json"
    }
});

// Extract the access token, truncate it to 256 characters, and return it as an encoded string
const truncatedAccessToken = apiResponse.data.accessToken.slice(0, 256);
return Functions.encodeString(truncatedAccessToken);
