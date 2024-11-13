#[cfg(feature = "client")]
mod auth;
#[cfg(feature = "client")]
pub use auth::get_access_token;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::{PostprocessingLevel, SxTClient};

#[cfg(feature = "client")]
mod substrate;
#[cfg(feature = "client")]
pub use substrate::query_commitments;

#[cfg(feature = "client")]
mod sxt_chain_runtime;

mod prover_query;
pub use prover_query::{plan_prover_query_dory, PlanProverQueryError};

mod verify;
pub use verify::{verify_prover_response, VerifyProverResponseError};

#[cfg(feature = "client")]
mod prover {
    tonic::include_proto!("sxt.core");
}

#[cfg(not(feature = "client"))]
mod prover {
    tonic::include_proto!("sxt.messages");
}