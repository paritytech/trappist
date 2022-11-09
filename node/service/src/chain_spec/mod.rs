use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use serde::{Deserialize, Serialize};

#[cfg(feature = "with-trappist-runtime")]
pub mod trappist;
#[cfg(feature = "with-base-runtime")]
pub mod base;