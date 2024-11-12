use crate::prover::{CommitmentScheme, ProverContextRange, ProverQuery};
use proof_of_sql::{
    base::commitment::QueryCommitments,
    proof_primitive::dory::DynamicDoryCommitment,
    sql::parse::{ConversionError, QueryExpr},
};
use proof_of_sql_parser::ParseError;
use snafu::Snafu;

const DEFAULT_SCHEMA: &str = "PUBLIC";

#[derive(Snafu, Debug)]
pub enum PlanProverQueryError {
    #[snafu(display("unable to parse sql: {source}"), context(false))]
    ParseIdentifier { source: ParseError },
    #[snafu(
        display("unable to create a provable AST from query: {source}"),
        context(false)
    )]
    ProvableAst { source: ConversionError },
    #[snafu(display("unable to serialize proof plan: {source}"), context(false))]
    ProofPlanSerialization {
        source: flexbuffers::SerializationError,
    },
}

pub fn plan_prover_query_dory(
    query: &str,
    commitments: &QueryCommitments<DynamicDoryCommitment>,
) -> Result<(ProverQuery, QueryExpr<DynamicDoryCommitment>), PlanProverQueryError> {
    let query_expr: QueryExpr<DynamicDoryCommitment> =
        QueryExpr::try_new(query.parse()?, DEFAULT_SCHEMA.parse()?, commitments)?;
    let proof_plan = query_expr.proof_expr();
    let serialized_proof_plan = flexbuffers::to_vec(proof_plan)?;

    let query_context = commitments
        .iter()
        .map(|(table_ref, commitment)| {
            (
                table_ref.to_string().to_uppercase(),
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
