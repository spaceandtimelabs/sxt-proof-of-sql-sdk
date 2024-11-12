use crate::{
    get_access_token,
    prover::{ProverContextRange, ProverQuery, ProverResponse},
    query_commitments,
};
use clap::ValueEnum;
use proof_of_sql::{
    base::database::{OwnedTable, TableRef},
    proof_primitive::dory::{
        DoryScalar, DynamicDoryCommitment, DynamicDoryEvaluationProof, VerifierSetup,
    },
    sql::{
        parse::QueryExpr,
        postprocessing::{apply_postprocessing_steps, OwnedTablePostprocessing},
        proof::VerifiableQueryResult,
    },
};
use reqwest::Client;
use std::{collections::HashMap, path::Path};

/// Level of postprocessing allowed
///
/// Some postprocessing steps are expensive so we allow the user to control the level of postprocessing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum PostprocessingLevel {
    /// No postprocessing allowed
    None,
    /// Only cheap postprocessing allowed
    Cheap,
    /// All postprocessing allowed
    All,
}

/// Space and Time (SxT) client
#[derive(Debug, Clone)]
pub struct SxTClient {
    /// Root URL for the Prover service
    pub prover_root_url: String,

    /// Root URL for the Auth service
    pub auth_root_url: String,

    /// URL for the Substrate node service
    pub substrate_node_url: String,

    /// API Key for Space and Time (SxT) services
    ///
    /// Please visit [Space and Time Studio](https://app.spaceandtime.ai/) to obtain an API key
    /// if you do not have one.
    pub sxt_api_key: String,

    /// Path to the verifier setup binary file
    pub verifier_setup: String,

    /// Level of postprocessing allowed. Default is [`PostprocessingLevel::Cheap`].
    pub postprocessing_level: PostprocessingLevel,
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
            postprocessing_level: PostprocessingLevel::Cheap,
        }
    }

    /// Set the level of postprocessing allowed
    pub fn with_postprocessing(mut self, postprocessing_level: PostprocessingLevel) -> Self {
        self.postprocessing_level = postprocessing_level;
        self
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
            .post(format!("{}/v1/prove", &self.prover_root_url))
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
        // Apply postprocessing steps
        let postprocessing = query_expr.postprocessing();
        let is_postprocessing_expensive = postprocessing.iter().any(|step| {
            matches!(
                step,
                OwnedTablePostprocessing::Slice(_) | OwnedTablePostprocessing::GroupBy(_)
            )
        });
        match (self.postprocessing_level, postprocessing.len(), is_postprocessing_expensive) {
            (_, 0, false) => Ok(owned_table_result),
            (PostprocessingLevel::All, _, _) | (PostprocessingLevel::Cheap, _, false) => {
                let transformed_result: OwnedTable<DoryScalar> =
                    apply_postprocessing_steps(owned_table_result, postprocessing)?;
                Ok(transformed_result)
            }
            _ => Err("Required postprocessing is not allowed. Please examine your query or change `PostprocessingLevel` using `SxTClient::with_postprocessing`".into()),
        }
    }
}
