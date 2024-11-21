// Import the package
const SxTSDK = await import(
  "https://raw.githubusercontent.com/spaceandtimelabs/sxt-proof-of-sql-sdk/feat/import-sdk/chainlink-functions/source/index.js"
);

// Define test parameters
const queryString = "SELECT SUM(BLOCK_NUMBER), COUNT(*) FROM ETHEREUM.BLOCKS";
const table = "ETHEREUM.BLOCKS";

if (!secrets.apiKey) {
  throw Error("Missing secret: apiKey");
}

// Initialize the SxTProof instance
const proof = new SxTSDK.SxTClient(
  "https://api.spaceandtime.dev/v1/prove",
  "https://proxy.api.spaceandtime.dev/auth/apikey",
  "https://rpc.testnet.sxt.network/",
  secrets.apiKey,
);

try {
  // Kick off the proof and await execution
  const result = await proof.queryAndVerify(queryString, table);
  console.log("Workflow completed successfully:", result);
  return Functions.encodeString("Verified");
} catch (error) {
  console.log("Workflow failed: ", error);
  return Functions.encodeString("Failed: ", error);
}
