use super::*;

#[test]
fn retirement_is_distinct_from_installation_with_fresh_support() {
    let bin = temp_bin("retirement");
    // Every combination of installed/absent and the three retirement states.
    // Only a maintained, installed tool is ready; a retirement is a fact about
    // the tool, not about whether its binary happens to resolve.
    let states = [
        None,
        Some(crate::contracts::SunsetState::Done),
        Some(crate::contracts::SunsetState::Scheduled),
    ];
    for retirement in states {
        for installed in [false, true] {
            let mut local = input(&bin);
            local.harnesses[0].retirement = retirement;
            local.harnesses[0].binary = if installed {
                "retirement"
            } else {
                "absent-target"
            }
            .into();
            let result = collect(&local, &Redactor::new(None, None));
            let readiness = result
                .records
                .iter()
                .find(|r| r.key == "harness.xh.readiness")
                .unwrap();
            let expected = match (retirement, installed) {
                (Some(_), _) => Code::Unsupported,
                (None, true) => Code::Ready,
                (None, false) => Code::Missing,
            };
            assert_eq!(readiness.code, expected);
            let word = match retirement {
                Some(crate::contracts::SunsetState::Done) => "retired",
                Some(crate::contracts::SunsetState::Scheduled) => "retiring",
                None => {
                    assert_ne!(readiness.value, "retired");
                    assert_ne!(readiness.value, "retiring");
                    "not-ready"
                }
            };
            if retirement.is_some() {
                assert_eq!(readiness.value, word, "readiness must name the state");
            }
            assert_eq!(
                result.ready.contains("xh"),
                installed && retirement.is_none()
            );
        }
    }
    fs::remove_dir_all(bin).unwrap();
}
