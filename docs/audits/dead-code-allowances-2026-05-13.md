# Dead Code Allowance Audit

Date: 2026-05-13
Issue: #50

## Summary

Current scan:

```bash
rg '#!?\[allow\(dead_code\)\]' src tests --glob '*.rs' | wc -l
```

Result: 121 allowances.

This is an audit-only slice. It does not remove code. The goal is to split the
work into reviewable follow-up domains so future PRs can remove, activate, or
feature-gate code with local context.

## Domain Inventory

| Domain | Files | Allowances | Recommended first action |
| --- | ---: | ---: | --- |
| Services | 4 | 25 | Split package operations, NPM operations, GitHub integration, and service re-exports. Remove unused re-export allowances only after call sites are checked. |
| Theme | 5 | 22 | Review as presentation cleanup. Remove unused format/layout helpers that have no planned UI path; keep only documented theme API surface. |
| Tools/catalog | 6 | 22 | Block broad cleanup on #31. After `ConfigStore` direction lands, remove bridge/config compatibility allowances or move them behind explicit migration boundaries. |
| Auth | 4 | 13 | Decide whether warning, API-key, and environment helpers are active auth flow or migration-only code. Add focused tests before removal. |
| API | 4 | 10 | Decide before #60 whether `src/api` becomes the HTTP client foundation. If not, remove placeholder API modules and preserve patterns in an ADR. |
| Tests | 2 | 9 | Prefer test fixtures or helper traits over suppressing dead test fields/methods. Safe to handle independently. |
| DB/migration | 2 | 6 | Keep TOML importer allowances until #31 migration path is clear; audit query builder separately. |
| Config | 1 | 3 | Tie cleanup to #31 because TOML config loaders are currently fallback compatibility code. |
| Progress/utilities | 2 | 5 | Remove unused helpers or add direct call sites in progress UI tests. |
| Installation | 1 | 2 | Verify install/update privilege paths; remove compatibility helpers if unused. |
| CLI | 1 | 1 | Check autocomplete helper usage and remove if stale. |
| Error extensions | 1 | 1 | Either expose as real public utility API or delete the unused extension traits. |

## File Counts

```text
12 src/services/services_package_operations.rs
10 src/services/services_entry_point.rs
 9 src/theme/theme_entry_point.rs
 8 src/tools/tools_command_mapping.rs
 7 src/auth_manager/auth_entry_point.rs
 6 tests/codex_functionality_tests.rs
 6 src/tools/tools_config.rs
 5 src/theme/theme_utilities.rs
 5 src/db/migration/toml_importer.rs
 4 src/tools/tools_db_bridge.rs
 4 src/theme/theme_text_formatting.rs
 4 src/progress_utils.rs
 4 src/api/api_base.rs
 3 tests/opencode_input_focus_tests.rs
 3 src/theme/theme_background_layout.rs
 3 src/services/services_tool_configuration.rs
 3 src/config/config_file_operations.rs
 3 src/auth_manager/auth_environment_detection.rs
 3 src/api/mod.rs
 2 src/tools/tools_entry_point.rs
 2 src/services/services_npm_operations.rs
 2 src/installation_arguments.rs
 2 src/auth_manager/auth_warning_system.rs
 2 src/api/api_tool_operations.rs
 1 src/tools/tools_display.rs
 1 src/tools/tools_dashboard_scanner.rs
 1 src/theme/theme_global_config.rs
 1 src/services/services_github_integration.rs
 1 src/error/result_ext.rs
 1 src/db/core/query_builder.rs
 1 src/cli_logic/cli_logic_autocomplete.rs
 1 src/auth_manager/auth_api_key_management.rs
 1 src/api/api_client.rs
```

## Follow-Up PR Split

1. Tests-only cleanup
   - Files: `tests/opencode_input_focus_tests.rs`, `tests/codex_functionality_tests.rs`
   - Goal: replace dead fixture fields/methods with explicit test helpers or remove unused helper members.

2. API decision
   - Files: `src/api/*`
   - Goal: decide whether this becomes #60's HTTP client foundation. If yes, activate the narrow client surface; if no, remove placeholder modules.

3. Config and catalog migration
   - Files: `src/tools/tools_db_bridge.rs`, `src/tools/tools_config.rs`, `src/tools/tools_command_mapping.rs`, `src/config/config_file_operations.rs`, `src/db/migration/toml_importer.rs`
   - Goal: wait for #31's ADR, then align allowances with the chosen `ConfigStore` migration path.

4. Services cleanup
   - Files: `src/services/*`
   - Goal: separate service re-export allowances from genuinely unused package, NPM, and GitHub integration code.

5. Auth cleanup
   - Files: `src/auth_manager/*`
   - Goal: classify helpers as active auth flow, migration helpers, or stale code before removal.

6. Theme and presentation cleanup
   - Files: `src/theme/*`, `src/progress_utils.rs`
   - Goal: remove unused UI helper APIs or add direct tests for intended public formatting/layout behavior.

7. Small utility cleanup
   - Files: `src/installation_arguments.rs`, `src/cli_logic/cli_logic_autocomplete.rs`, `src/error/result_ext.rs`, `src/db/core/query_builder.rs`
   - Goal: remove isolated allowances or document a real public API contract.

## Done Condition For #50

#50 should stay open until each follow-up domain is either:

- removed,
- activated by a real call site and test,
- moved behind a feature flag,
- or documented as a deliberate public compatibility boundary.
