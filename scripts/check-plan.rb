#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "time"
require "yaml"

ROOT = File.expand_path("..", __dir__)
PLAN_DIR = File.join(ROOT, "plan")
PAGE_PATTERN = /\A(\d{2})-tj-hardening-[a-z0-9]+(?:-[a-z0-9]+)*\.md\z/
ALLOWED_STATUSES = %w[proposed ready in-progress blocked review complete].freeze
ALLOWED_MODES = %w[pending-page-12-decision kernel-hosted cloudflare-hosted zero-host].freeze
COMPLETE_RESULTS = %w[pass approved not-selected not-selected-zero-host].freeze
COST_CLASSES = %w[
  maintainer-zero external-free-tier user-metered maintainer-metered
].freeze
OPT_IN_ID_PATTERN = /\A[a-z0-9]+(?:-[a-z0-9]+)*\z/
SURFACE_KINDS = %w[
  local-static pages killercoda codespaces analytics feedback hosted-terminal
].freeze

class PlanError < StandardError; end

def ensure_plan(condition, message)
  raise PlanError, message unless condition
end

def read_document(path)
  text = File.read(path)
  match = text.match(/\A---\r?\n(?<yaml>.*?)\r?\n---\r?\n/m)
  ensure_plan(match, "#{path}: missing YAML front matter")
  metadata = YAML.safe_load(
    match[:yaml], permitted_classes: [Time], permitted_symbols: [], aliases: false
  )
  ensure_plan(metadata.is_a?(Hash), "#{path}: front matter must be a mapping")
  [metadata, text]
rescue Psych::SyntaxError => e
  raise PlanError, "#{path}: invalid YAML front matter: #{e.message}"
end

def read_json(path)
  ensure_plan(File.file?(path), "#{path}: required JSON file is missing")
  ensure_plan(!File.symlink?(path), "#{path}: JSON records cannot be symlinks")
  value = JSON.parse(File.read(path))
  ensure_plan(value.is_a?(Hash), "#{path}: JSON root must be an object")
  value
rescue JSON::ParserError => e
  raise PlanError, "#{path}: invalid JSON: #{e.message}"
end

def page_criteria(path, text)
  criteria = text.scan(/^- \[[ xX]\] `([A-Z][A-Z0-9]{2}-\d{2})`/).flatten
  evidence_rows = text.lines.filter_map do |line|
    next unless line.match?(/^\| [A-Z][A-Z0-9]{2}-\d{2} \|/)

    line.strip.split("|", -1)[1...-1].map(&:strip)
  end
  evidence = evidence_rows.map(&:first)
  ensure_plan(criteria.any?, "#{path}: no acceptance criteria")
  ensure_plan(criteria == evidence, "#{path}: acceptance criteria and evidence rows differ")
  ensure_plan(criteria.uniq == criteria, "#{path}: duplicate acceptance criterion")
  [criteria, evidence_rows]
end

def git_object_exists?(object)
  system("git", "-C", ROOT, "cat-file", "-e", object,
         out: File::NULL, err: File::NULL)
end

def git_tracks?(relative_path)
  system("git", "-C", ROOT, "ls-files", "--error-unmatch", "--", relative_path,
         out: File::NULL, err: File::NULL)
end

def git_ancestor?(ancestor)
  system("git", "-C", ROOT, "merge-base", "--is-ancestor", ancestor, "HEAD",
         out: File::NULL, err: File::NULL)
end

def validate_ownership(pages)
  path = File.join(PLAN_DIR, "ownership.json")
  record = read_json(path)
  ensure_plan(record["schema_version"] == 1, "#{path}: schema_version must be 1")
  assignments = record["assignments"]
  ensure_plan(assignments.is_a?(Array), "#{path}: assignments must be an array")
  ensure_plan(git_tracks?("plan/ownership.json"), "#{path}: file must be tracked by git")

  by_page = {}
  assignments.each do |assignment|
    ensure_plan(assignment.is_a?(Hash), "#{path}: every assignment must be an object")
    required = %w[
      page owner_role reviewer_role owner_name reviewer_name owner_kind reviewer_kind
    ]
    required.each do |field|
      ensure_plan(assignment.key?(field), "#{path}: assignment missing #{field}")
    end
    page_id = assignment["page"]
    ensure_plan(pages.key?(page_id), "#{path}: assignment has unknown page #{page_id}")
    ensure_plan(!by_page.key?(page_id), "#{path}: duplicate assignment for page #{page_id}")
    page = pages.fetch(page_id)
    ensure_plan(assignment["owner_role"] == page[:metadata]["owner"] &&
                assignment["reviewer_role"] == page[:metadata]["reviewer"],
                "#{path}: assignment roles do not match page #{page_id}")
    ensure_plan(assignment["owner_name"].is_a?(String) &&
                !assignment["owner_name"].empty? &&
                assignment["reviewer_name"].is_a?(String) &&
                !assignment["reviewer_name"].empty? &&
                assignment["owner_name"] != assignment["reviewer_name"],
                "#{path}: page #{page_id} needs distinct named humans")
    ensure_plan(assignment["owner_kind"] == "human" &&
                assignment["reviewer_kind"] == "human",
                "#{path}: page #{page_id} owner/reviewer kinds must be human")
    by_page[page_id] = assignment
  end
  ensure_plan(by_page.keys.sort == pages.keys.sort,
              "#{path}: assignments must cover every numbered page exactly once")
  by_page
end

def validate_complete_evidence(path, rows, assignment)
  rows.each do |cells|
    ensure_plan(cells.length == 7, "#{path}: evidence row #{cells.first} must have 7 columns")
    criterion, method, artifact, ref, utc, result, reviewer = cells
    values = [method, artifact, ref, utc, result, reviewer]
    ensure_plan(values.all? { |value| !value.empty? && value !~ /\bpending\b/i },
                "#{path}: evidence row #{criterion} has an empty/pending value")
    ensure_plan(ref.match?(/\A[0-9a-f]{7,40}\z/),
                "#{path}: evidence row #{criterion} ref must be a commit SHA")
    ensure_plan(git_object_exists?("#{ref}^{commit}"),
                "#{path}: evidence row #{criterion} ref is not a local commit")
    ensure_plan(git_ancestor?(ref),
                "#{path}: evidence row #{criterion} ref is not an ancestor of HEAD")
    begin
      parsed_time = Time.iso8601(utc)
      ensure_plan(utc.end_with?("Z") && parsed_time.utc?,
                  "#{path}: evidence row #{criterion} UTC must end in Z")
      ensure_plan(parsed_time <= Time.now.utc + 300,
                  "#{path}: evidence row #{criterion} UTC is in the future")
    rescue ArgumentError
      raise PlanError, "#{path}: evidence row #{criterion} has invalid ISO-8601 UTC"
    end
    ensure_plan(COMPLETE_RESULTS.include?(result),
                "#{path}: evidence row #{criterion} has invalid result #{result}")
    unless artifact.match?(/\Ahttps:\/\//)
      artifact_path = artifact.split("#", 2).first
      local_artifact = File.expand_path(artifact_path, ROOT)
      ensure_plan(!artifact_path.start_with?("/") &&
                  local_artifact.start_with?("#{ROOT}/") &&
                  File.file?(local_artifact),
                  "#{path}: evidence row #{criterion} artifact must be HTTPS or a repository-relative file")
      relative_path = local_artifact.delete_prefix("#{ROOT}/")
      ensure_plan(artifact_path == relative_path && git_tracks?(relative_path),
                  "#{path}: evidence row #{criterion} artifact must be tracked by git")
      ensure_plan(git_object_exists?("#{ref}:#{relative_path}"),
                  "#{path}: evidence row #{criterion} artifact is absent at ref #{ref}")
    end
    ensure_plan(reviewer == assignment["reviewer_name"],
                "#{path}: evidence row #{criterion} reviewer must match the page assignment")
  end
end

def validate_opt_in(id, provider)
  ensure_plan(id.is_a?(String) && id.match?(OPT_IN_ID_PATTERN),
              "provider opt-in ID must be a lowercase slug")
  path = File.join(ROOT, "demo", "opt-ins", "#{id}.json")
  record = read_json(path)
  ensure_plan(git_tracks?("demo/opt-ins/#{id}.json"), "#{path}: opt-in must be tracked by git")
  required = %w[
    id kind status provider purpose owner reviewer owner_kind reviewer_kind
    account_owner credential_location regions max_sessions max_concurrency
    max_total_usd max_monthly_usd approved_at expires_at cleanup_owner
    cleanup_deadline kill_switch billable_usage_authorized cost_class
  ]
  required.each { |field| ensure_plan(record.key?(field), "#{path}: missing #{field}") }
  ensure_plan(record["id"] == id && record["kind"] == "provider" && record["status"] == "approved",
              "#{path}: opt-in identity/kind/status is invalid")
  ensure_plan(record["provider"] == provider, "#{path}: provider does not match manifest")
  ensure_plan(record["owner"].is_a?(String) && record["reviewer"].is_a?(String) &&
              record["owner"] != record["reviewer"], "#{path}: owner/reviewer are invalid")
  ensure_plan(record["owner_kind"] == "human" && record["reviewer_kind"] == "human",
              "#{path}: provider opt-in requires human owner/reviewer")
  %w[purpose account_owner credential_location cleanup_owner kill_switch].each do |field|
    ensure_plan(record[field].is_a?(String) && !record[field].empty?,
                "#{path}: #{field} must be a nonempty string")
  end
  ensure_plan(record["regions"].is_a?(Array) && !record["regions"].empty? &&
              record["regions"].all? { |region| region.is_a?(String) && !region.empty? },
              "#{path}: regions must be a nonempty string array")
  %w[max_sessions max_concurrency].each do |field|
    ensure_plan(record[field].is_a?(Integer) && record[field].positive?,
                "#{path}: #{field} must be a positive integer")
  end
  %w[max_total_usd max_monthly_usd].each do |field|
    ensure_plan(record[field].is_a?(Numeric) && record[field] >= 0,
                "#{path}: #{field} must be nonnegative")
  end
  ensure_plan([true, false].include?(record["billable_usage_authorized"]),
              "#{path}: billable_usage_authorized must be boolean")
  unless record["billable_usage_authorized"]
    ensure_plan(record["max_total_usd"] == 0 && record["max_monthly_usd"] == 0,
                "#{path}: non-billable opt-in must cap spend at USD 0")
  end
  expected_cost_class = if record["billable_usage_authorized"]
                          "maintainer-metered"
                        else
                          "external-free-tier"
                        end
  ensure_plan(record["cost_class"] == expected_cost_class,
              "#{path}: cost_class does not match billable authorization")
  times = %w[approved_at expires_at cleanup_deadline].to_h do |field|
    ensure_plan(record[field].is_a?(String) && record[field].end_with?("Z"),
                "#{path}: #{field} must be UTC ending in Z")
    [field, Time.iso8601(record[field])]
  rescue ArgumentError, TypeError
    raise PlanError, "#{path}: #{field} must be ISO-8601"
  end
  ensure_plan(times["approved_at"] < times["expires_at"] &&
              times["expires_at"] <= times["cleanup_deadline"],
              "#{path}: approval/expiry/cleanup times are out of order")
  now = Time.now.utc
  ensure_plan(times["approved_at"] <= now,
              "#{path}: approval cannot start in the future")
  ensure_plan(times["expires_at"] > now && times["cleanup_deadline"] > now,
              "#{path}: provider opt-in is expired or past its cleanup deadline")
  record
end

def validate_manifest(mode)
  path = File.join(ROOT, "demo", "manifest.json")
  manifest = read_json(path)
  ensure_plan(git_tracks?("demo/manifest.json"), "#{path}: manifest must be tracked by git")
  required = %w[
    schema_version mode provider fixture_version fixture_hash binary_ref
    binary_checksum protocol_version budget_policy kill_switch_state
    rollback_manifest provider_opt_in_id requires_explicit_paid_opt_in
  ]
  required.each { |field| ensure_plan(manifest.key?(field), "#{path}: missing #{field}") }
  ensure_plan(manifest["schema_version"].is_a?(Integer) && manifest["schema_version"] > 0,
              "#{path}: schema_version must be positive")
  ensure_plan(manifest["mode"] == mode, "#{path}: mode does not match page 12")
  %w[fixture_version provider protocol_version kill_switch_state rollback_manifest].each do |field|
    ensure_plan((manifest[field].is_a?(String) || manifest[field].is_a?(Integer)) &&
                !manifest[field].to_s.empty?, "#{path}: #{field} must be nonempty")
  end
  ensure_plan(manifest["fixture_hash"].is_a?(String) &&
              manifest["binary_checksum"].is_a?(String) &&
              manifest["fixture_hash"].match?(/\A[0-9a-f]{64}\z/) &&
              manifest["binary_checksum"].match?(/\A[0-9a-f]{64}\z/),
              "#{path}: fixture/binary checksums must be SHA-256")
  ensure_plan(manifest["binary_ref"].is_a?(String) &&
              manifest["binary_ref"].match?(/\A[0-9a-f]{7,40}\z/),
              "#{path}: binary_ref must be a commit SHA")
  ensure_plan(git_object_exists?("#{manifest['binary_ref']}^{commit}") &&
              git_ancestor?(manifest["binary_ref"]),
              "#{path}: binary_ref must be a local ancestor commit")
  ensure_plan(manifest["budget_policy"].is_a?(Hash), "#{path}: budget_policy must be an object")
  budget_fields = %w[maintainer_budget_usd max_total_usd max_monthly_usd max_sessions max_concurrency]
  budget_fields.each do |field|
    ensure_plan(manifest["budget_policy"].key?(field),
                "#{path}: budget_policy missing #{field}")
  end
  ensure_plan(manifest["requires_explicit_paid_opt_in"] == true,
              "#{path}: explicit paid opt-in must remain required")
  rollback_path = manifest["rollback_manifest"]
  rollback = File.expand_path(rollback_path, ROOT)
  ensure_plan(rollback_path.is_a?(String) && !rollback_path.start_with?("/") &&
              rollback_path == rollback.delete_prefix("#{ROOT}/") &&
              rollback_path != "demo/manifest.json" &&
              rollback.start_with?("#{ROOT}/") && File.file?(rollback) &&
              git_tracks?(rollback_path),
              "#{path}: rollback_manifest must be a distinct tracked repository-relative file")
  rollback_data = read_json(rollback)
  ensure_plan(rollback_data["mode"] == "zero-host" && rollback_data["provider"] == "none" &&
              rollback_data.dig("budget_policy", "maintainer_budget_usd") == 0 &&
              rollback_data.dig("budget_policy", "max_total_usd") == 0 &&
              rollback_data.dig("budget_policy", "max_monthly_usd") == 0 &&
              rollback_data.dig("budget_policy", "max_sessions") == 0 &&
              rollback_data.dig("budget_policy", "max_concurrency") == 0 &&
              rollback_data["kill_switch_state"] == "hosted-disabled",
              "#{path}: rollback_manifest must be a verified zero-host manifest")

  if mode == "zero-host"
    ensure_plan(manifest["provider"] == "none" && manifest["provider_opt_in_id"].nil?,
                "#{path}: zero-host must use provider none and no opt-in")
    ensure_plan(manifest["budget_policy"]["maintainer_budget_usd"] == 0,
                "#{path}: zero-host maintainer budget must be USD 0")
    %w[max_total_usd max_monthly_usd max_sessions max_concurrency].each do |field|
      ensure_plan(manifest["budget_policy"][field] == 0,
                  "#{path}: zero-host #{field} must be zero")
    end
    ensure_plan(manifest["kill_switch_state"] == "hosted-disabled",
                "#{path}: zero-host must disable hosted execution")
  else
    provider = mode.delete_suffix("-hosted")
    ensure_plan(manifest["provider"] == provider, "#{path}: hosted provider/mode mismatch")
    ensure_plan(manifest["provider_opt_in_id"].is_a?(String) &&
                !manifest["provider_opt_in_id"].empty?, "#{path}: hosted mode needs an opt-in ID")
    opt_in = validate_opt_in(manifest["provider_opt_in_id"], provider)
    expected_budget = {
      "maintainer_budget_usd" => opt_in["max_total_usd"],
      "max_total_usd" => opt_in["max_total_usd"],
      "max_monthly_usd" => opt_in["max_monthly_usd"],
      "max_sessions" => opt_in["max_sessions"],
      "max_concurrency" => opt_in["max_concurrency"]
    }
    ensure_plan(expected_budget.all? { |field, value| manifest["budget_policy"][field] == value },
                "#{path}: budget_policy must exactly match the provider opt-in")
  end
end

def validate_surface_opt_in(id, kind)
  ensure_plan(id.is_a?(String) && id.match?(OPT_IN_ID_PATTERN),
              "surface opt-in ID must be a lowercase slug")
  path = File.join(ROOT, "demo", "opt-ins", "#{id}.json")
  record = read_json(path)
  ensure_plan(git_tracks?("demo/opt-ins/#{id}.json"), "#{path}: opt-in must be tracked by git")
  required = %w[
    id kind status owner reviewer owner_kind reviewer_kind approved_at expires_at cost_class
  ]
  required.each { |field| ensure_plan(record.key?(field), "#{path}: missing #{field}") }
  ensure_plan(record["id"] == id && record["kind"] == kind && record["status"] == "approved",
              "#{path}: surface opt-in identity/kind/status is invalid")
  ensure_plan(record["owner"].is_a?(String) && record["reviewer"].is_a?(String) &&
              record["owner"] != record["reviewer"], "#{path}: owner/reviewer are invalid")
  ensure_plan(record["owner_kind"] == "human" && record["reviewer_kind"] == "human",
              "#{path}: surface opt-in requires human owner/reviewer")
  ensure_plan(COST_CLASSES.include?(record["cost_class"]),
              "#{path}: invalid cost_class #{record['cost_class']}")
  times = %w[approved_at expires_at].to_h do |field|
    ensure_plan(record[field].is_a?(String) && record[field].end_with?("Z"),
                "#{path}: #{field} must be UTC ending in Z")
    [field, Time.iso8601(record[field])]
  rescue ArgumentError, TypeError
    raise PlanError, "#{path}: #{field} must be ISO-8601"
  end
  now = Time.now.utc
  ensure_plan(times["approved_at"] < times["expires_at"] &&
              times["approved_at"] <= now && times["expires_at"] > now,
              "#{path}: surface approval is future-dated, expired, or out of order")
  if kind == "publish"
    ensure_plan(record["cost_class"] != "user-metered",
                "#{path}: publish opt-in cannot use the user-metered cost class")
    publish_fields = %w[
      account_owner payment_owner permissions data_handling rollback_owner
      max_total_usd max_monthly_usd billable_usage_authorized
    ]
    publish_fields.each { |field| ensure_plan(record.key?(field), "#{path}: missing #{field}") }
    %w[account_owner payment_owner permissions data_handling rollback_owner].each do |field|
      ensure_plan(record[field].is_a?(String) && !record[field].empty?,
                  "#{path}: #{field} must be a nonempty string")
    end
    %w[max_total_usd max_monthly_usd].each do |field|
      ensure_plan(record[field].is_a?(Numeric) && record[field] >= 0,
                  "#{path}: #{field} must be nonnegative")
    end
    ensure_plan([true, false].include?(record["billable_usage_authorized"]),
                "#{path}: billable_usage_authorized must be boolean")
    if record["billable_usage_authorized"]
      ensure_plan(record["cost_class"] == "maintainer-metered",
                  "#{path}: billable publication must be maintainer-metered")
    else
      ensure_plan(record["max_total_usd"] == 0 && record["max_monthly_usd"] == 0 &&
                  record["cost_class"] != "maintainer-metered",
                  "#{path}: non-billable publication must cap spend at USD 0")
    end
  else
    ensure_plan(record["disclosure"].is_a?(String) && !record["disclosure"].empty?,
                "#{path}: user-metered opt-in needs a visitor billing disclosure")
    ensure_plan(record["billing_owner"] == "visitor" &&
                record["maintainer_sponsorship"] == false &&
                record["cost_class"] == "user-metered",
                "#{path}: user-metered surface must be visitor-funded")
  end
  record
end

def validate_surfaces(mode)
  path = File.join(ROOT, "demo", "surfaces.json")
  registry = read_json(path)
  ensure_plan(git_tracks?("demo/surfaces.json"), "#{path}: registry must be tracked by git")
  ensure_plan(registry["schema_version"] == 1 &&
              registry["surfaces"].is_a?(Array), "#{path}: invalid registry shape")
  ids = []
  kinds = []
  local_static = false
  hosted_terminal_count = 0
  registry["surfaces"].each do |surface|
    required = %w[id kind active cost_class location owner opt_in_id]
    required.each { |field| ensure_plan(surface.key?(field), "#{path}: surface missing #{field}") }
    ensure_plan(surface["id"].is_a?(String) && !surface["id"].empty?,
                "#{path}: surface id must be nonempty")
    ensure_plan(!ids.include?(surface["id"]), "#{path}: duplicate surface #{surface['id']}")
    ids << surface["id"]
    ensure_plan(!kinds.include?(surface["kind"]),
                "#{path}: duplicate surface kind #{surface['kind']}")
    kinds << surface["kind"]
    ensure_plan(surface["owner"].is_a?(String) && !surface["owner"].empty?,
                "#{path}: surface #{surface['id']} owner must be nonempty")
    ensure_plan(COST_CLASSES.include?(surface["cost_class"]),
                "#{path}: surface #{surface['id']} has invalid cost class")
    ensure_plan([true, false].include?(surface["active"]),
                "#{path}: surface #{surface['id']} active must be boolean")
    ensure_plan(SURFACE_KINDS.include?(surface["kind"]),
                "#{path}: surface #{surface['id']} has unknown kind #{surface['kind']}")
    unless surface["active"]
      ensure_plan(surface["opt_in_id"].nil?,
                  "#{path}: inactive surface #{surface['id']} must clear opt_in_id")
      next
    end

    case surface["kind"]
    when "local-static"
      ensure_plan(surface["cost_class"] == "maintainer-zero" && surface["opt_in_id"].nil?,
                  "#{path}: local static surface cannot require spend/opt-in")
      location = surface["location"]
      local_path = File.expand_path(location.to_s, ROOT)
      ensure_plan(location.is_a?(String) && !location.start_with?("/") &&
                  local_path.start_with?("#{ROOT}/") && File.file?(local_path) &&
                  !File.symlink?(local_path) &&
                  git_tracks?(local_path.delete_prefix("#{ROOT}/")),
                  "#{path}: local static location must be a tracked repository file")
      local_static = true
    when "codespaces"
      ensure_plan(surface["location"].is_a?(String) && surface["location"].match?(/\Ahttps:\/\//),
                  "#{path}: active Codespaces location must be HTTPS")
      ensure_plan(surface["opt_in_id"].is_a?(String) && !surface["opt_in_id"].empty?,
                  "#{path}: active Codespaces surface needs an opt-in ID")
      record = validate_surface_opt_in(surface["opt_in_id"], "user-metered")
      ensure_plan(surface["cost_class"] == "user-metered" &&
                  record["cost_class"] == surface["cost_class"],
                  "#{path}: Codespaces cost class is invalid")
    when "hosted-terminal"
      ensure_plan(surface["location"].is_a?(String) && surface["location"].match?(/\Ahttps:\/\//),
                  "#{path}: active hosted-terminal location must be HTTPS")
      manifest = read_json(File.join(ROOT, "demo", "manifest.json"))
      ensure_plan(surface["opt_in_id"] == manifest["provider_opt_in_id"],
                  "#{path}: hosted-terminal opt-in must match the selected manifest")
      record = validate_opt_in(surface["opt_in_id"], manifest["provider"])
      expected_cost = record["billable_usage_authorized"] ? "maintainer-metered" : "external-free-tier"
      ensure_plan(surface["cost_class"] == expected_cost,
                  "#{path}: hosted-terminal cost class does not match opt-in")
      hosted_terminal_count += 1
    else
      ensure_plan(surface["location"].is_a?(String) && surface["location"].match?(/\Ahttps:\/\//),
                  "#{path}: active surface #{surface['id']} location must be HTTPS")
      ensure_plan(surface["opt_in_id"].is_a?(String) && !surface["opt_in_id"].empty?,
                  "#{path}: active surface #{surface['id']} needs a publish opt-in ID")
      record = validate_surface_opt_in(surface["opt_in_id"], "publish")
      ensure_plan(record["cost_class"] == surface["cost_class"],
                  "#{path}: surface #{surface['id']} cost class/opt-in mismatch")
    end
  end
  ensure_plan(kinds.sort == SURFACE_KINDS.sort,
              "#{path}: registry must contain each supported surface kind exactly once")
  ensure_plan(local_static, "#{path}: active local-static surface is required")
  expected_hosted_terminals = mode == "zero-host" ? 0 : 1
  ensure_plan(hosted_terminal_count == expected_hosted_terminals,
              "#{path}: exactly #{expected_hosted_terminals} hosted-terminal surface(s) required")
end

begin
  ensure_plan(Dir.exist?(PLAN_DIR), "plan directory is missing")
  index_path = File.join(PLAN_DIR, "index.md")
  ensure_plan(File.file?(index_path), "plan/index.md is missing")

  page_paths = Dir.children(PLAN_DIR).filter_map do |name|
    match = PAGE_PATTERN.match(name)
    [match[1], File.join(PLAN_DIR, name)] if match
  end.sort_by(&:first)
  ensure_plan(page_paths.any?, "no numbered plan pages found")

  expected_ids = (1..page_paths.length).map { |number| format("%02d", number) }
  ensure_plan(page_paths.map(&:first) == expected_ids, "numbered plan pages must be contiguous")
  registered_names = page_paths.map { |_, path| File.basename(path) }
  extra_markdown = Dir.glob(File.join(PLAN_DIR, "*.md")).map { |path| File.basename(path) } -
                   ["index.md"] - registered_names
  ensure_plan(extra_markdown.empty?, "unregistered plan pages: #{extra_markdown.join(', ')}")

  index_metadata, index_text = read_document(index_path)
  %w[
    target branch status status_source default_delivery_mode
    default_maintainer_budget_usd provider_opt_in_required
  ].each do |field|
    ensure_plan(index_metadata.key?(field), "plan/index.md: missing #{field}")
  end
  target = index_metadata["target"]
  ensure_plan(target.is_a?(String) && target.match?(/\Av\d+\.\d+\.\d+\z/),
              "plan/index.md: target must be vX.Y.Z")
  ensure_plan(index_metadata["branch"] == "release/#{target.delete_prefix('v')}",
              "plan/index.md: branch does not match target")
  ensure_plan(ALLOWED_STATUSES.include?(index_metadata["status"]),
              "plan/index.md: invalid status")
  ensure_plan(index_metadata["status_source"] == "derived-from-child-pages",
              "plan/index.md: status_source must be derived-from-child-pages")
  ensure_plan(index_metadata["default_delivery_mode"] == "zero-host",
              "plan/index.md: default_delivery_mode must fail closed to zero-host")
  ensure_plan(index_metadata["default_maintainer_budget_usd"] == 0,
              "plan/index.md: default maintainer budget must be USD 0")
  ensure_plan(index_metadata["provider_opt_in_required"] == true,
              "plan/index.md: provider opt-in must be required")

  pages = {}
  all_criteria = []
  page_paths.each do |id, path|
    metadata, text = read_document(path)
    required = %w[id target title status owner reviewer depends_on blocks]
    required.each { |field| ensure_plan(metadata.key?(field), "#{path}: missing #{field}") }
    ensure_plan(metadata["id"] == id, "#{path}: id does not match filename")
    ensure_plan(metadata["target"] == target, "#{path}: target does not match index")
    ensure_plan(ALLOWED_STATUSES.include?(metadata["status"]), "#{path}: invalid status")
    ensure_plan(metadata["owner"].is_a?(String) && !metadata["owner"].empty?,
                "#{path}: owner is missing")
    ensure_plan(metadata["reviewer"].is_a?(String) && metadata["reviewer"] != metadata["owner"],
                "#{path}: reviewer must be present and distinct from owner")
    ensure_plan(metadata["depends_on"].is_a?(Array) && metadata["depends_on"].all?(String),
                "#{path}: depends_on must be a string array")
    ensure_plan(metadata["blocks"].is_a?(Array) && metadata["blocks"].all?(String),
                "#{path}: blocks must be a string array")
    ensure_plan(metadata["depends_on"].uniq == metadata["depends_on"],
                "#{path}: duplicate dependency")
    ensure_plan(metadata["blocks"].uniq == metadata["blocks"], "#{path}: duplicate block")

    ["Objective", "Acceptance Criteria", "Evidence", "Completion Gate"].each do |heading|
      ensure_plan(text.include?("## #{heading}"), "#{path}: missing #{heading}")
    end
    ensure_plan(text.match?(/^## (Risks|Abort).*Rollback/), "#{path}: missing rollback section")

    criteria, evidence_rows = page_criteria(path, text)
    all_criteria.concat(criteria)
    if metadata["status"] == "complete"
      ensure_plan(!text.match?(/^- \[ \]/), "#{path}: complete page has unchecked work")
      pending_rows = text.lines.select do |line|
        line.start_with?("|") && line.match?(/\|\s*pending\s*\|/i)
      end
      ensure_plan(pending_rows.empty?, "#{path}: complete page has pending table values")
    end

    if id.to_i >= 12
      ensure_plan(ALLOWED_MODES.include?(metadata["delivery_mode"]),
                  "#{path}: invalid or missing delivery_mode")
    end
    pages[id] = { path: path, metadata: metadata, text: text, evidence: evidence_rows }
  end
  ensure_plan(all_criteria.uniq == all_criteria, "acceptance criterion IDs must be globally unique")

  complete_pages = pages.select { |_, page| page[:metadata]["status"] == "complete" }
  if complete_pages.any?
    ownership = validate_ownership(pages)
    complete_pages.each_value do |page|
      assignment = ownership.fetch(page[:metadata]["id"])
      validate_complete_evidence(page[:path], page[:evidence], assignment)
    end
  end

  pages.each do |id, page|
    page[:metadata]["depends_on"].each do |dependency|
      ensure_plan(pages.key?(dependency), "#{page[:path]}: unknown dependency #{dependency}")
      ensure_plan(dependency.to_i < id.to_i, "#{page[:path]}: dependencies must point backward")
      if page[:metadata]["status"] != "proposed"
        ensure_plan(pages[dependency][:metadata]["status"] == "complete",
                    "#{page[:path]}: active page has incomplete dependency #{dependency}")
      end
    end
    page[:metadata]["blocks"].each do |blocked|
      next if blocked == "index-completion"

      ensure_plan(pages.key?(blocked) && blocked.to_i > id.to_i,
                  "#{page[:path]}: invalid blocked page #{blocked}")
      ensure_plan(pages[blocked][:metadata]["depends_on"].include?(id),
                  "#{page[:path]}: blocked page #{blocked} lacks reciprocal dependency")
    end
  end

  mode_pages = pages.select { |id, _| id.to_i >= 12 }.values
  modes = mode_pages.map { |page| page[:metadata]["delivery_mode"] }.uniq
  ensure_plan(modes.length == 1, "pages 12 onward must use one delivery_mode")
  if pages.fetch("12")[:metadata]["status"] == "complete"
    ensure_plan(modes.first != "pending-page-12-decision",
                "completed provider selection must propagate a delivery_mode")
    validate_manifest(modes.first)
    sel_08 = pages.fetch("12")[:evidence].find { |row| row.first == "SEL-08" }
    ensure_plan(sel_08, "#{pages.fetch('12')[:path]}: missing SEL-08 evidence")
    expected_results = case modes.first
                       when "zero-host" then ["not-selected-zero-host"]
                       when "kernel-hosted" then ["not-selected"]
                       when "cloudflare-hosted" then %w[pass approved]
                       end
    ensure_plan(expected_results.include?(sel_08[5]),
                "#{pages.fetch('12')[:path]}: SEL-08 result is invalid for #{modes.first}")
  else
    ensure_plan(modes.first == "pending-page-12-decision",
                "delivery_mode must remain pending until page 12 is complete")
  end
  validate_surfaces(modes.first) if pages.fetch("13")[:metadata]["status"] == "complete"

  registry = index_text.scan(
    /^\| (\d{2}) \| \[[^\]]+\]\(([^)]+)\) \| [^|]+ \| [^|]+ \| ([a-z-]+) \|$/
  )
  ensure_plan(registry.length == pages.length, "plan/index.md: registry does not match page count")
  registry.each do |id, filename, status|
    ensure_plan(pages.key?(id), "plan/index.md: registry has unknown page #{id}")
    ensure_plan(File.basename(pages[id][:path]) == filename,
                "plan/index.md: registry filename mismatch for #{id}")
    ensure_plan(pages[id][:metadata]["status"] == status,
                "plan/index.md: registry status mismatch for #{id}")
  end
  ensure_plan(registry.map(&:first).sort == pages.keys.sort,
              "plan/index.md: registry IDs do not match child pages")

  checklist = index_text.scan(/^- \[([ xX])\] (\d{2}) .+ complete$/)
  ensure_plan(checklist.length == pages.length, "plan/index.md: master checklist is incomplete")
  checklist.each do |mark, id|
    ensure_plan(pages.key?(id), "plan/index.md: checklist has unknown page #{id}")
    checked = mark.match?(/[xX]/)
    complete = pages[id][:metadata]["status"] == "complete"
    ensure_plan(checked == complete, "plan/index.md: checklist status mismatch for #{id}")
  end

  children_complete = pages.values.all? { |page| page[:metadata]["status"] == "complete" }
  index_complete = index_metadata["status"] == "complete"
  ensure_plan(index_complete == children_complete,
              "plan/index.md: complete status must be derived from all child pages")
  if index_complete
    ensure_plan(modes.first != "pending-page-12-decision",
                "plan/index.md: complete plan has a pending delivery_mode")
    ensure_plan(!index_text.match?(/^- \[ \]/), "plan/index.md: complete plan has unchecked items")
    completion = index_text.split("## Completion Record", 2).last
    ensure_plan(completion && !completion.match?(/\bpending\b/i),
                "plan/index.md: completion record is pending")
    ensure_plan(completion.match?(/^\| All child pages complete \| yes \|$/),
                "plan/index.md: completion record must confirm all child pages")
    decisions = index_text.split("## Decision Log", 2).last&.split("## Completion Record", 2)&.first
    ensure_plan(decisions && !decisions.match?(/\|\s*pending\s*\|/i),
                "plan/index.md: decision log is unresolved")
    begin
      reviewed = index_metadata["last_reviewed_utc"]
      reviewed.is_a?(Time) ? reviewed : Time.iso8601(reviewed)
    rescue ArgumentError, TypeError
      raise PlanError, "plan/index.md: last_reviewed_utc must be ISO-8601 when complete"
    end
  end

  puts "check-plan: ok (#{pages.length} pages, #{all_criteria.length} criteria, mode #{modes.first})"
rescue PlanError => e
  warn "check-plan: #{e.message}"
  exit 1
end
