//! Public face of the contracts domain: the shared data shapes every other
//! domain depends on. Consumers import from `contracts`, never from
//! `contracts::structs` internals.

pub use crate::contracts::structs::capability::Capability;
pub use crate::contracts::structs::command::CommandPlan;
pub use crate::contracts::structs::effect::{Effect, Interaction};
pub use crate::contracts::structs::environment::EnvMode;
pub use crate::contracts::structs::harness::{CapabilityPlan, Harness};
pub use crate::contracts::structs::support::{EvidenceMode, SupportState};
