mod auth;
pub use auth::get_access_token;
mod client;
pub use client::{PostprocessingLevel, SxTClient};
mod substrate;
pub use substrate::query_commitments;
mod sxt_chain_runtime;

mod prover_query;
pub use prover_query::{plan_prover_query_dory, PlanProverQueryError};

mod prover {
    tonic::include_proto!("sxt.core");
}
