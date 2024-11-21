// Import the package
const SxTSDK = await import(
  "https://raw.githubusercontent.com/spaceandtimelabs/sxt-proof-of-sql-sdk/feat/import-sdk/chainlink-functions/source/index.js"
);

// Define test parameters
const queryString = "SELECT SUM(BLOCK_NUMBER), COUNT(*) FROM ETHEREUM.BLOCKS";
const table = "ETHEREUM.BLOCKS";
// TODO: This is currently hardcoded. But, we need to make it dynamic.
const commitmentKey =
  "0xca407206ec1ab726b2636c4b145ac28749505e273536fae35330b966dac69e86a4832a125c0464e066dd20add960efb518424c4f434b5320455448455245554d4a9e6f9b8d43f6ad008f8c291929dee201";

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
  const result = await proof.queryAndVerify(queryString, table, commitmentKey);
  console.log("Workflow completed successfully:", result);
  return Functions.encodeString("Verified");
} catch (error) {
  console.log("Workflow failed: ", error);
  return Functions.encodeString("Failed: ", error);
}
