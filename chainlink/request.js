const fs = require("fs");
const path = require("path");
const {
    SubscriptionManager,
    SecretsManager,
    simulateScript,
    ResponseListener,
    ReturnType,
    decodeResult,
    FulfillmentCode,
} = require("@chainlink/functions-toolkit");
const functionsConsumerAbi = require("../../abi/functionsClient.json");
const ethers = require("ethers");
require("@chainlink/env-enc").config();

const consumerAddress = "0x8dFf78B7EE3128D00E90611FBeD20A71397064D9"; // REPLACE this with your Functions consumer address
const subscriptionId = 3; // REPLACE this with your subscription ID

const makeRequestSepolia = async () => {
    // hardcoded for Ethereum Sepolia
    const routerAddress = "0xb83E47C2bC239B3bf370bc41e1459A34b41238D0";
    const linkTokenAddress = "0x779877A7B0D9E8603169DdbD7836e478b4624789";
    const donId = "fun-ethereum-sepolia-1";
    const explorerUrl = "https://sepolia.etherscan.io";
    const gatewayUrls = [
        "https://01.functions-gateway.testnet.chain.link/",
        "https://02.functions-gateway.testnet.chain.link/",
    ];

    // Initialize functions settings
    const source = fs
        .readFileSync(path.resolve(__dirname, "source.js"))
        .toString();

    const args = ["1", "USD"];
    const secrets = { apiKey: process.env.COINMARKETCAP_API_KEY };
    const slotIdNumber = 0; // slot ID where to upload the secrets
    const expirationTimeMinutes = 15; // expiration time in minutes of the secrets
    const gasLimit = 300000;

    // Initialize ethers signer and provider to interact with the contracts onchain
    const privateKey = process.env.PRIVATE_KEY; // fetch PRIVATE_KEY
    if (!privateKey)
        throw new Error(
            "private key not provided - check your environment variables"
        );

    const rpcUrl = process.env.ETHEREUM_SEPOLIA_RPC_URL; // fetch Sepolia RPC URL

    if (!rpcUrl)
        throw new Error(`rpcUrl not provided  - check your environment variables`);

    const provider = new ethers.providers.JsonRpcProvider(rpcUrl);

    const wallet = new ethers.Wallet(privateKey);
    const signer = wallet.connect(provider); // create ethers signer for signing transactions
}