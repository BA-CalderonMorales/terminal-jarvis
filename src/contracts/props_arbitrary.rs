use super::*;
use quickcheck::{Arbitrary, Gen};

pub(super) fn choose<T: Copy>(gen: &mut Gen, values: &[T]) -> T {
    *gen.choose(values).unwrap()
}

impl Arbitrary for Capability {
    fn arbitrary(gen: &mut Gen) -> Self {
        choose(gen, &Capability::ALL)
    }
}

impl Arbitrary for Effect {
    fn arbitrary(gen: &mut Gen) -> Self {
        choose(
            gen,
            &[Effect::ReadOnly, Effect::StateChanging, Effect::Dangerous],
        )
    }
}

impl Arbitrary for Interaction {
    fn arbitrary(gen: &mut Gen) -> Self {
        choose(
            gen,
            &[Interaction::Noninteractive, Interaction::Interactive],
        )
    }
}

impl Arbitrary for EnvMode {
    fn arbitrary(gen: &mut Gen) -> Self {
        choose(gen, &[EnvMode::None, EnvMode::Any, EnvMode::All])
    }
}

impl Arbitrary for SupportState {
    fn arbitrary(gen: &mut Gen) -> Self {
        choose(
            gen,
            &[
                SupportState::Verified,
                SupportState::Expected,
                SupportState::Manual,
                SupportState::Stub,
                SupportState::Unsupported,
                SupportState::Disabled,
                SupportState::Unknown,
            ],
        )
    }
}

impl Arbitrary for EvidenceMode {
    fn arbitrary(gen: &mut Gen) -> Self {
        choose(
            gen,
            &[
                EvidenceMode::Deterministic,
                EvidenceMode::DisposableReal,
                EvidenceMode::Manual,
                EvidenceMode::Unsupported,
            ],
        )
    }
}
