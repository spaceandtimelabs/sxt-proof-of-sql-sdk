use crate::prover::{CommitmentScheme, ProverContextRange, ProverQuery};
use proof_of_sql::{
    base::commitment::QueryCommitments,
    proof_primitive::dory::DynamicDoryCommitment,
    sql::parse::{ConversionError, QueryExpr},
};
use proof_of_sql_parser::ParseError;
use snafu::Snafu;
use sqlparser::ast::Ident;

/// Proof-of-sql requires a default schema to be provided when creating a QueryExpr.
/// This is used as the schema when tables referenced in the query don't have one.
pub const DEFAULT_SCHEMA: &str = "PUBLIC";

/// Errors that can occur when planning a query to the prover.
#[derive(Snafu, Debug)]
pub enum PlanProverQueryError {
    /// Unable to parse sql.
    #[snafu(display("unable to parse sql: {source}"), context(false))]
    ParseIdentifier { source: ParseError },
    /// Unable to create a provable AST from query.
    #[snafu(
        display("unable to create a provable AST from query: {source}"),
        context(false)
    )]
    ProvableAst { source: ConversionError },
    /// Unable to serialize proof plan.
    #[snafu(display("unable to serialize proof plan: {error}"))]
    ProofPlanSerialization { error: bincode::error::EncodeError },
}

impl From<bincode::error::EncodeError> for PlanProverQueryError {
    fn from(error: bincode::error::EncodeError) -> Self {
        PlanProverQueryError::ProofPlanSerialization { error }
    }
}

/// Create a query for the prover service from sql query text and commitments.
pub fn plan_prover_query_dory(
    query: &str,
    commitments: &QueryCommitments<DynamicDoryCommitment>,
) -> Result<(ProverQuery, QueryExpr), PlanProverQueryError> {
    let query_expr: QueryExpr =
        QueryExpr::try_new(query.parse()?, Ident::new(DEFAULT_SCHEMA), commitments)?;
    let proof_plan = query_expr.proof_expr();
    let serialized_proof_plan = bincode::serde::encode_to_vec(
        proof_plan,
        bincode::config::legacy()
            .with_fixed_int_encoding()
            .with_big_endian(),
    )?;

    let query_context = commitments
        .iter()
        .map(|(table_ref, commitment)| {
            (
                table_ref.to_string(),
                ProverContextRange {
                    start: commitment.range().start as u64,
                    ends: vec![commitment.range().end as u64],
                },
            )
        })
        .collect();

    Ok((
        ProverQuery {
            proof_plan: serialized_proof_plan,
            query_context,
            commitment_scheme: CommitmentScheme::DynamicDory.into(),
        },
        query_expr,
    ))
}
