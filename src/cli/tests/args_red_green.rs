use super::*;

fn json_with_child_spawn_requires_dry_run(words: Vec<String>) -> bool {
    match parse_cli(words) {
        Ok(parsed) => {
            let child = matches!(
                parsed.action,
                Action::Run(_)
                    | Action::Direct { .. }
                    | Action::Install(_)
                    | Action::Update(Some(_))
                    | Action::SelfUpdate { .. }
            );
            !(child && parsed.options.output == OutputMode::Json && !parsed.options.dry_run)
        }
        _ => true,
    }
}

fn verbose_implies_check_or_version(words: Vec<String>) -> bool {
    match parse_cli(words) {
        Ok(parsed) if parsed.options.verbose => {
            matches!(parsed.action, Action::Check | Action::Version { .. })
        }
        _ => true,
    }
}

fn lifecycle_flags_require_lifecycle_action(words: Vec<String>) -> bool {
    match parse_cli(words) {
        Ok(parsed)
            if parsed.options.dry_run
                || parsed.options.no_input
                || parsed.options.confirm.is_some()
                || parsed.options.allow_dangerous =>
        {
            matches!(
                parsed.action,
                Action::Run(_)
                    | Action::Direct { .. }
                    | Action::Install(_)
                    | Action::Update(Some(_))
                    | Action::SelfUpdate { .. }
            )
        }
        _ => true,
    }
}

fn confirm_is_well_formed(words: Vec<String>) -> bool {
    match parse_cli(words) {
        Ok(parsed) => parsed.options.confirm.is_none_or(|value| {
            value
                .split_once(':')
                .is_some_and(|(operation, target)| !operation.is_empty() && !target.is_empty())
        }),
        _ => true,
    }
}

#[test]
fn options_and_validation_rules() {
    let props: &[fn(Vec<String>) -> bool] = &[
        json_with_child_spawn_requires_dry_run,
        verbose_implies_check_or_version,
        lifecycle_flags_require_lifecycle_action,
        confirm_is_well_formed,
    ];
    for property in props {
        quickcheck::quickcheck(*property);
    }
}
