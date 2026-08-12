use super::*;

#[test]
fn rendered_never_doubles_punctuation() {
    let plain = Failure::new(
        4,
        "capability_unknown",
        "vibe:download is unknown",
        "run `terminal-jarvis plan vibe download`",
    );
    assert_eq!(
        plain.rendered(),
        "vibe:download is unknown: run `terminal-jarvis plan vibe download`"
    );
    let dotted = Failure::new(
        4,
        "capability_unknown",
        "claude:download is unknown; Install Claude from its documented distribution channel.",
        "run `terminal-jarvis plan claude download`",
    );
    assert_eq!(
        dotted.rendered(),
        "claude:download is unknown; Install Claude from its documented distribution channel. run `terminal-jarvis plan claude download`"
    );
}
