use crate::{get_access_token, query_commitments};
use proof_of_sql::{
    base::database::{OwnedTable, TableRef},
    proof_primitive::dory::{
        DoryScalar, DynamicDoryCommitment, DynamicDoryEvaluationProof, VerifierSetup,
    },
    sql::{parse::QueryExpr, proof::VerifiableQueryResult},
};
use prover::{ProverContextRange, ProverQuery, ProverResponse};
use reqwest::Client;
use std::{collections::HashMap, path::Path};

mod prover {
    tonic::include_proto!("sxt.core");
}

/// Space and Time (SxT) client
#[derive(Debug, Clone)]
pub struct SxTClient {
    /// Root URL for the Prover service
    ///
    /// This URL is used for interacting with the prover service.
    pub prover_root_url: String,

    /// Root URL for the Auth service
    ///
    /// Used for authentication requests. Generally the same as the Prover Root URL.
    pub auth_root_url: String,

    /// URL for the Substrate node service
    ///
    /// Specifies the Substrate node endpoint used for accessing commitment data.
    pub substrate_node_url: String,

    /// API Key for Space and Time (SxT) services
    ///
    /// The API key required for authorization with Space and Time services.
    pub sxt_api_key: String,

    /// Path to the verifier setup binary file
    ///
    /// Specifies the path to the `verifier_setup.bin` file required for verification.
    pub verifier_setup: String,
}

impl SxTClient {
    /// Create a new SxT client
    pub fn new(
        prover_root_url: String,
        auth_root_url: String,
        substrate_node_url: String,
        sxt_api_key: String,
        verifier_setup: String,
    ) -> Self {
        Self {
            prover_root_url,
            auth_root_url,
            substrate_node_url,
            sxt_api_key,
            verifier_setup,
        }
    }

    /// Query and verify a SQL query
    ///
    /// Run a SQL query and verify the result using Dynamic Dory.
    pub async fn query_and_verify(
        &self,
        query: &str,
        table: &str,
    ) -> Result<OwnedTable<DoryScalar>, Box<dyn core::error::Error>> {
        // Parse table_ref into TableRef struct
        let table_ref = TableRef::new(table.parse()?);
        let schema = table_ref.schema_id();
        // Load verifier setup
        let verifier_setup_path = Path::new(&self.verifier_setup);
        let verifier_setup = VerifierSetup::load_from_file(verifier_setup_path)?;
        // Accessor setup
        let accessor =
            query_commitments(&[table_ref.resource_id()], &self.substrate_node_url).await?;
        // Parse the SQL query
        let query_expr: QueryExpr<DynamicDoryCommitment> =
            QueryExpr::try_new(query.parse()?, schema, &accessor)?;
        let proof_plan = query_expr.proof_expr();
        let serialized_proof_plan = flexbuffers::to_vec(proof_plan)?;
        // Send the query to the prover
        let mut query_context = HashMap::new();
        let commitment_range = accessor[&table_ref].range();
        query_context.insert(
            table_ref.to_string().to_uppercase(),
            ProverContextRange {
                start: commitment_range.start as u64,
                ends: vec![commitment_range.end as u64],
            },
        );
        let prover_query = ProverQuery {
            proof_plan: serialized_proof_plan,
            query_context,
            commitment_scheme: 1,
        };
        let client = Client::new();
        let access_token = get_access_token(&self.sxt_api_key, &self.auth_root_url).await?;
        let response = client
            .post(format!("https://{}/v1/prove", &self.prover_root_url))
            .bearer_auth(&access_token)
            .json(&prover_query)
            .send()
            .await?
            .error_for_status()?;
        let serialized_prover_response = response.text().await?;
        let prover_response = serde_json::from_str::<ProverResponse>(&serialized_prover_response)
            .map_err(|_e| {
            format!(
                "Failed to parse prover response: {}",
                &serialized_prover_response
            )
        })?;
        let stringified_verifiable_result = prover_response.verifiable_result.clone();
        let verifiable_result: VerifiableQueryResult<DynamicDoryEvaluationProof> =
            flexbuffers::from_slice(&stringified_verifiable_result)?;
        // Verify the proof
        let proof = verifiable_result.proof.unwrap();
        let serialized_result = verifiable_result.provable_result.unwrap();
        let owned_table_result = proof
            .verify(proof_plan, &accessor, &serialized_result, &&verifier_setup)?
            .table;
        Ok(owned_table_result)
    }
}
