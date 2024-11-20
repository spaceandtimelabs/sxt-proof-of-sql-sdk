// Import the package
const SxTProofModule = await import("npm:sxt-chain-sdk");

// Extract the default export (SxTProof class)
const SxTProof = SxTProofModule.default;

// Define test parameters
const queryString = 'SELECT SUM(BLOCK_NUMBER), COUNT(*) FROM ETHEREUM.BLOCKS';
const commitmentKey =
    '0xca407206ec1ab726b2636c4b145ac28749505e273536fae35330b966dac69e86a4832a125c0464e066dd20add960efb518424c4f434b5320455448455245554d4a9e6f9b8d43f6ad008f8c291929dee201';

if (!secrets.apiKey) {
    throw Error("Missing secret: apiKey");
}

// Initialize the SxTProof instance
const proof = new SxTProof(queryString, commitmentKey, secrets.apiKey);

try {
    // Kick off the proof and await execution
    const result = await proof.executeWorkflow();
    console.log("Workflow completed successfully:", result);
    return Functions.encodeString("Verified");
} catch (error) {
    console.log("Workflow failed:");
    return Functions.encodeString("Failed: ", error);
}