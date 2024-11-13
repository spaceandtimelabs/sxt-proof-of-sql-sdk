mod auth;
pub use auth::get_access_token;
mod client;
pub use client::{PostprocessingLevel, SxTClient};
mod substrate;
pub use substrate::query_commitments;
pub mod sxt_chain_runtime;

mod prover_query;
pub use prover_query::{plan_prover_query_dory, PlanProverQueryError};

mod verify;
pub use verify::{verify_prover_response, VerifyProverResponseError};

mod prover {
    tonic::include_proto!("sxt.core");
}
