This example is heavily borrowed from https://docs.chain.link/chainlink-functions/tutorials/api-use-secrets.
1. Run `npm install` to install the dependencies.
2. Set up your on chain resources as shown here: https://docs.chain.link/chainlink-functions/tutorials/api-use-secrets#configure-your-onchain-resources.
3. Add the following environment variables, either in a `.env` file, or using `npx env-enc set`.
    ```
    CONSUMER_ADDRESS=0x8dFf78B7EE3128D00E90611FBeD20A71397064D9 # REPLACE this with your Functions consumer address
    SUBSCRIPTION_ID=3 # REPLACE this with your subscription ID
    LINK_TOKEN_ADDRESS=0x779877A7B0D9E8603169DdbD7836e478b4624789 # REPLACE this with your wallet address
    ETHEREUM_SEPOLIA_RPC_URL=
    PRIVATE_KEY=
    SXT_API_KEY=
    ```
4. Run `node example/request.js` to upload the secrets, run a simulation, and then submit a chainlink job.