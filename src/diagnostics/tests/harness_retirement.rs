use super::*;

#[test]
fn retirement_is_distinct_from_installation_with_fresh_support() {
    let bin = temp_bin("retirement");
    for sunset in [false, true] {
        for installed in [false, true] {
            let mut local = input(&bin);
            local.harnesses[0].sunset = sunset;
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
            let expected = match (sunset, installed) {
                (true, _) => Code::Unsupported,
                (false, true) => Code::Ready,
                (false, false) => Code::Missing,
            };
            assert_eq!(readiness.code, expected);
            assert_eq!(readiness.value == "retired", sunset);
            assert_eq!(result.ready.contains("xh"), installed && !sunset);
        }
    }
    fs::remove_dir_all(bin).unwrap();
}
