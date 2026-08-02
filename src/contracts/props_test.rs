use super::*;

fn capability_roundtrip(value: Capability) -> bool {
    Capability::parse(value.as_str()) == Some(value) && format!("{value}") == value.as_str()
}

fn capability_canonical(text: String) -> bool {
    let known = Capability::ALL.iter().any(|c| c.as_str() == text);
    Capability::parse(&text).is_some() == known
}

fn effect_roundtrip(value: Effect) -> bool {
    Effect::parse(value.as_str()) == Ok(value)
}

fn effect_canonical(text: String) -> bool {
    let known = ["read-only", "state-changing", "dangerous"].contains(&text.as_str());
    Effect::parse(&text).is_ok() == known
}

fn interaction_roundtrip(value: Interaction) -> bool {
    Interaction::parse(value.as_str()) == Ok(value)
}

fn interaction_canonical(text: String) -> bool {
    let known = ["noninteractive", "interactive"].contains(&text.as_str());
    Interaction::parse(&text).is_ok() == known
}

fn env_canonical(text: String) -> bool {
    let known = ["none", "any", "all"].contains(&text.as_str());
    EnvMode::parse(&text).is_ok() == known
}

fn support_roundtrip(value: SupportState) -> bool {
    SupportState::parse(value.as_str()) == Ok(value)
}

fn support_canonical(text: String) -> bool {
    let known = [
        "verified",
        "expected",
        "manual",
        "stub",
        "unsupported",
        "disabled",
        "unknown",
    ]
    .contains(&text.as_str());
    SupportState::parse(&text).is_ok() == known
}

fn evidence_roundtrip(value: EvidenceMode) -> bool {
    EvidenceMode::parse(value.as_str()) == Ok(value)
}

fn evidence_canonical(text: String) -> bool {
    let known =
        ["deterministic", "disposable-real", "manual", "unsupported"].contains(&text.as_str());
    EvidenceMode::parse(&text).is_ok() == known
}

#[test]
fn enum_roundtrip_and_canonical() {
    quickcheck::quickcheck(capability_roundtrip as fn(Capability) -> bool);
    quickcheck::quickcheck(capability_canonical as fn(String) -> bool);
    quickcheck::quickcheck(effect_roundtrip as fn(Effect) -> bool);
    quickcheck::quickcheck(effect_canonical as fn(String) -> bool);
    quickcheck::quickcheck(interaction_roundtrip as fn(Interaction) -> bool);
    quickcheck::quickcheck(interaction_canonical as fn(String) -> bool);
    quickcheck::quickcheck(env_canonical as fn(String) -> bool);
    quickcheck::quickcheck(support_roundtrip as fn(SupportState) -> bool);
    quickcheck::quickcheck(support_canonical as fn(String) -> bool);
    quickcheck::quickcheck(evidence_roundtrip as fn(EvidenceMode) -> bool);
    quickcheck::quickcheck(evidence_canonical as fn(String) -> bool);
}
