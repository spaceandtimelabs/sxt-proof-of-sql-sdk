export class SxTClient {
  constructor(proverRootURL, authRootURL, substrateNodeURL, sxtApiKey) {
    this.proverRootURL = proverRootURL;
    this.authRootURL = authRootURL;
    this.substrateNodeURL = substrateNodeURL;
    this.sxtApiKey = sxtApiKey;
  }

  async #getAccessToken() {
    // Ensure the API key is available
    if (!this.sxtApiKey) {
      throw Error("API Key Not Found");
    }
    const authResponse = await postHttpRequest({
      url: this.authRootURL,
      headers: {
        apikey: this.sxtApiKey,
        "Content-Type": "application/json",
      },
    });
    if (!authResponse.ok) {
      throw new Error(
        `Error querying auth endpoint: ${authResponse.status}: ${authResponse.statusText}`,
      );
    }
    return authResponse.json();
  }
  async #getCommitment(commitmentKey, blockHash = null) {
    const params = blockHash ? [commitmentKey, blockHash] : [commitmentKey]

    const commitmentResponse = await postHttpRequest({
      url: this.substrateNodeURL,
      headers: {
        "Content-Type": "application/json",
      },
      data: {
        id: 1,
        jsonrpc: "2.0",
        method: "state_getStorage",
        params,
      },
    });

    if (!commitmentResponse.ok) {
      throw new Error(
        `Error querying RPC node: ${commitmentResponse.status}: ${commitmentResponse.statusText}`,
      );
    }

    return commitmentResponse.json();
  }
  async #getProof(accessToken, proverQuery) {
    const proverResponse = await postHttpRequest({
      url: this.proverRootURL,
      headers: {
        Authorization: "Bearer " + accessToken,
        "content-type": "application/json",
      },
      data: proverQuery,
    });

    if (!proverResponse.ok) {
      throw new Error(
        `Error querying prover: ${proverResponse.status}: ${proverResponse.statusText}`,
      );
    }

    return proverResponse.json();
  }

  async queryAndVerify(queryString, table, blockHash = null) {
    const commitmentKey = "0x" + commitment_storage_key_dory(table);
    const authResponse = await this.#getAccessToken();
    const accessToken = authResponse.accessToken;
    const commitmentResponse = await this.#getCommitment(commitmentKey, blockHash);
    const commitment = commitmentResponse.result.slice(2); // remove the 0x prefix

    let commitments = [new TableRefAndCommitment(table, commitment)];
    const plannedProverQuery = plan_prover_query_dory(queryString, commitments);
    const proverQuery = plannedProverQuery.prover_query_json;
    const queryExpr = plannedProverQuery.query_expr_json;
    commitments = plannedProverQuery.commitments;

    const proverResponseJson = await this.#getProof(accessToken, proverQuery);

    const result = verify_prover_response_dory(
      proverResponseJson,
      queryExpr,
      commitments,
    );
    return result;
  }
}

async function postHttpRequest({ url, headers = {}, data = null }) {
  const controller = new AbortController();
  const id = setTimeout(() => controller.abort(), 3000);
  const response = await fetch(url, {
    method: "POST",
    headers,
    body: data ? JSON.stringify(data) : undefined,
    signal: controller.signal,
  });
  clearTimeout(id);
  return response;
}
