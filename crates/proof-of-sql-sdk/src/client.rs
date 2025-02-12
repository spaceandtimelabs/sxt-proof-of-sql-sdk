use crate::{
    get_access_token, query_commitments,
    substrate::{verify_attestations_for_block, AttestationError, SxtConfig},
};
use clap::ValueEnum;
use proof_of_sql::{
    base::database::OwnedTable,
    proof_primitive::dory::{DoryScalar, DynamicDoryEvaluationProof, VerifierSetup},
    sql::postprocessing::{apply_postprocessing_steps, OwnedTablePostprocessing},
};
use reqwest::Client;
use std::path::Path;
use subxt::Config;
use sxt_proof_of_sql_sdk_local::{
    plan_prover_query_dory, prover::ProverResponse, verify_prover_response,
};

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

    /// Query and verify a SQL query at the given SxT block.
    ///
    /// Run a SQL query and verify the result using Dynamic Dory.
    ///
    /// If `block_ref` is `None`, the latest block is used.
    pub async fn query_and_verify(
        &self,
        query: &str,
        table: &str,
        block_ref: Option<<SxtConfig as Config>::Hash>,
    ) -> Result<OwnedTable<DoryScalar>, Box<dyn core::error::Error>> {
        // Parse table into `TableRef` struct
        let table_ref = table.try_into()?;

        // Load verifier setup
        let verifier_setup_path = Path::new(&self.verifier_setup);
        let verifier_setup = VerifierSetup::load_from_file(verifier_setup_path)?;
        // Accessor setup
        let accessor = query_commitments(&[table_ref], &self.substrate_node_url, block_ref).await?;

        let (prover_query, query_expr) = plan_prover_query_dory(query, &accessor)?;

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

        let verified_table_result = verify_prover_response::<DynamicDoryEvaluationProof>(
            &prover_response,
            &query_expr,
            &accessor,
            &&verifier_setup,
        )?;

        // Apply postprocessing steps
        let postprocessing = query_expr.postprocessing();
        let is_postprocessing_expensive = postprocessing.iter().any(|step| {
            matches!(
                step,
                OwnedTablePostprocessing::Slice(_) | OwnedTablePostprocessing::GroupBy(_)
            )
        });
        match (self.postprocessing_level, postprocessing.len(), is_postprocessing_expensive) {
            (_, 0, false) => Ok(verified_table_result),
            (PostprocessingLevel::All, _, _) | (PostprocessingLevel::Cheap, _, false) => {
                let transformed_result: OwnedTable<DoryScalar> =
                    apply_postprocessing_steps(verified_table_result, postprocessing)?;
                Ok(transformed_result)
            }
            _ => Err("Required postprocessing is not allowed. Please examine your query or change `PostprocessingLevel` using `SxTClient::with_postprocessing`".into()),
        }
    }

    /// Verify attestations for a specific block number
    ///
    /// This method uses the `verify_attestations_for_block` function to validate
    /// attestations for a given block number.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The block number for which attestations need to be verified.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all attestations are valid and consistent. Otherwise, it returns an
    /// `AttestationError` describing the failure.
    pub async fn verify_attestations(&self, block_number: u32) -> Result<(), AttestationError> {
        verify_attestations_for_block(&self.substrate_node_url, block_number).await
    }
}
