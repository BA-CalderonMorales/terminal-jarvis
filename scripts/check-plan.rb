#!/usr/bin/env ruby
# frozen_string_literal: true

require "open3"
require "set"
require "time"
require "yaml"

ROOT = File.expand_path("..", __dir__)
PLAN_DIR = File.join(ROOT, "plan")
PHASE_PATTERN = /\A(\d{2})-[a-z0-9]+(?:-[a-z0-9]+)*\.md\z/
ALLOWED_STATUSES = %w[proposed in-progress blocked evidence-ready complete].freeze
EVIDENCE_STATUSES = %w[evidence-ready complete].freeze
EVIDENCE_RESULTS = %w[pass approved manual unsupported].freeze
EVIDENCE_COLUMNS = [
  "Covers", "Method", "Artifact", "Ref", "UTC", "Result", "Verified by"
].freeze
REQUIRED_SECTIONS = [
  "Objective", "Work", "Acceptance Criteria", "Evidence", "Exit"
].freeze
INDEX_KEYS = %w[
  target branch baseline scope owner delivery_mode maintainer_budget_usd
  status_source phases
].freeze

class PlanError < StandardError; end

def ensure_plan(condition, message)
  raise PlanError, message unless condition
end

def usage
  <<~TEXT
    usage: ruby scripts/check-plan.rb

    Validates the v0.1.13 phase graph, checklists, and completed evidence.
  TEXT
end

def read_document(path)
  text = File.read(path)
  match = text.match(/\A---\s*\n(.*?)\n---\s*\n/m)
  ensure_plan(match, "#{path}: missing YAML frontmatter")
  metadata = YAML.safe_load(match[1], permitted_classes: [], aliases: false)
  ensure_plan(metadata.is_a?(Hash), "#{path}: frontmatter must be a mapping")
  [metadata, text]
rescue Psych::SyntaxError => e
  raise PlanError, "#{path}: invalid YAML: #{e.message.lines.first.strip}"
end

def section(text, name, path)
  match = text.match(/^## #{Regexp.escape(name)}\s*$\n(.*?)(?=^## |\z)/m)
  ensure_plan(match, "#{path}: missing ## #{name}")
  match[1]
end

def criterion_lines(body)
  body.each_line.filter_map do |line|
    match = line.match(/^- \[[ xX]\] `([A-Z][A-Z0-9]*-\d{2})`\s*(.*)$/)
    [match[1], match[2]] if match
  end.to_h
end

def parse_table_line(line)
  line.strip.delete_prefix("|").delete_suffix("|").split("|").map(&:strip)
end

def evidence_rows(text, path)
  lines = section(text, "Evidence", path).each_line
    .map(&:strip).select { |line| line.start_with?("|") }
  ensure_plan(lines.length >= 3, "#{path}: evidence table is missing")
  ensure_plan(parse_table_line(lines[0]) == EVIDENCE_COLUMNS,
              "#{path}: evidence columns must be #{EVIDENCE_COLUMNS.join(', ')}")
  divider = parse_table_line(lines[1])
  ensure_plan(divider.length == EVIDENCE_COLUMNS.length &&
              divider.all? { |cell| cell.match?(/\A:?-{3,}:?\z/) },
              "#{path}: invalid evidence table divider")
  lines.drop(2).map { |line| parse_table_line(line) }
end

def git(*args)
  stdout, stderr, status = Open3.capture3("git", "-C", ROOT, *args)
  [stdout.strip, stderr.strip, status.success?]
end

def valid_commit?(ref)
  ref.match?(/\A[0-9a-f]{7,40}\z/) && git("cat-file", "-e", "#{ref}^{commit}")[2]
end

def ancestor_of_head?(ref)
  git("merge-base", "--is-ancestor", ref, "HEAD")[2]
end

def valid_artifact?(artifact, ref)
  return true if artifact.match?(%r{\Ahttps://[^\s]+\z})

  clean = artifact.delete_prefix("./")
  return false if clean.empty? || clean.start_with?("/") || clean.split("/").include?("..")

  git("cat-file", "-e", "#{ref}:#{clean}")[2]
end

def valid_utc?(value)
  return false unless value.match?(/\A\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z\z/)

  Time.iso8601(value)
  true
rescue ArgumentError
  false
end

def validate_evidence(phase, criteria)
  path = phase.fetch(:path)
  rows = evidence_rows(phase.fetch(:text), path)
  covered = Set.new

  rows.each_with_index do |cells, index|
    row = index + 1
    ensure_plan(cells.length == EVIDENCE_COLUMNS.length,
                "#{path}: evidence row #{row} must have #{EVIDENCE_COLUMNS.length} columns")
    ensure_plan(cells.none? { |cell| cell.empty? || cell == "pending" },
                "#{path}: evidence row #{row} contains a placeholder")

    covers, method, artifact, ref, utc, result, verifier = cells
    ids = covers.split(/\s*,\s*/)
    ensure_plan(ids.any?, "#{path}: evidence row #{row} covers no criteria")
    ids.each do |id|
      ensure_plan(criteria.key?(id), "#{path}: evidence row #{row} covers unknown #{id}")
      ensure_plan(covered.add?(id), "#{path}: criterion #{id} is covered more than once")
    end

    ensure_plan(method.length >= 3, "#{path}: evidence row #{row} method is too short")
    ensure_plan(valid_commit?(ref), "#{path}: evidence row #{row} has invalid ref #{ref}")
    ensure_plan(ancestor_of_head?(ref), "#{path}: evidence ref #{ref} is not an ancestor of HEAD")
    ensure_plan(valid_utc?(utc), "#{path}: evidence row #{row} has invalid UTC #{utc}")
    ensure_plan(EVIDENCE_RESULTS.include?(result),
                "#{path}: evidence row #{row} has invalid result #{result}")
    ensure_plan(verifier.length >= 2, "#{path}: evidence row #{row} has no verifier")
    ensure_plan(valid_artifact?(artifact, ref),
                "#{path}: evidence artifact #{artifact} does not exist at #{ref}")

    if %w[manual unsupported].include?(result)
      ids.each do |id|
        ensure_plan(criteria.fetch(id).match?(/manual|unsupported/i),
                    "#{path}: #{id} does not permit result #{result}")
      end
    end
  end

  missing = criteria.keys.to_set - covered
  ensure_plan(missing.empty?, "#{path}: evidence does not cover #{missing.to_a.sort.join(', ')}")
  rows
end

def validate_phase(phase, phases, target)
  metadata = phase.fetch(:metadata)
  text = phase.fetch(:text)
  path = phase.fetch(:path)
  id = phase.fetch(:id)

  %w[id target title status owner starts_after completion_requires
     independent_review_required].each do |key|
    ensure_plan(metadata.key?(key), "#{path}: missing frontmatter key #{key}")
  end
  ensure_plan(metadata["id"] == id, "#{path}: id must be #{id}")
  ensure_plan(metadata["target"] == target, "#{path}: target must be #{target}")
  ensure_plan(ALLOWED_STATUSES.include?(metadata["status"]),
              "#{path}: invalid status #{metadata['status']}")
  ensure_plan(metadata["owner"].is_a?(String) && metadata["owner"] != "pending",
              "#{path}: owner is required")
  ensure_plan([true, false].include?(metadata["independent_review_required"]),
              "#{path}: independent_review_required must be boolean")

  REQUIRED_SECTIONS.each { |name| section(text, name, path) }
  work = section(text, "Work", path)
  acceptance = section(text, "Acceptance Criteria", path)
  ensure_plan(work.match?(/^- \[[ xX]\] /), "#{path}: Work needs checkboxes")
  criteria = criterion_lines(acceptance)
  ensure_plan(criteria.any?, "#{path}: Acceptance Criteria needs criterion IDs")

  %w[starts_after completion_requires].each do |key|
    dependencies = metadata[key]
    ensure_plan(dependencies.is_a?(Array), "#{path}: #{key} must be an array")
    ensure_plan(dependencies.uniq == dependencies, "#{path}: duplicate #{key} dependency")
    dependencies.each do |dependency|
      ensure_plan(phases.key?(dependency), "#{path}: unknown #{key} phase #{dependency}")
      ensure_plan(dependency.to_i < id.to_i, "#{path}: #{key} must point backward")
    end
  end

  status = metadata["status"]
  if %w[in-progress evidence-ready complete].include?(status)
    metadata["starts_after"].each do |dependency|
      ensure_plan(phases.fetch(dependency)[:metadata]["status"] == "complete",
                  "#{path}: start dependency #{dependency} is incomplete")
    end
  end
  if EVIDENCE_STATUSES.include?(status)
    metadata["completion_requires"].each do |dependency|
      ensure_plan(phases.fetch(dependency)[:metadata]["status"] == "complete",
                  "#{path}: completion dependency #{dependency} is incomplete")
    end
    ensure_plan(!text.match?(/^- \[ \] /), "#{path}: evidence-ready phase has unchecked work")
    rows = validate_evidence(phase, criteria)
    if status == "complete" && metadata["independent_review_required"]
      reviewer = metadata["reviewer"]
      ensure_plan(reviewer.is_a?(String) && !reviewer.empty? && reviewer != "pending",
                  "#{path}: complete phase requires an independent reviewer")
      ensure_plan(reviewer != metadata["owner"], "#{path}: reviewer must differ from owner")
      ensure_plan(rows.any? { |row| row[0].split(/\s*,\s*/).include?("REL-10") && row[6] == reviewer },
                  "#{path}: independent reviewer must verify REL-10")
    end
  end

  if status == "blocked"
    blocker = section(text, "Blocker", path)
    ensure_plan(blocker.match?(/Owner:/) && blocker.match?(/Recovery:/),
                "#{path}: blocked phase needs Owner and Recovery")
  end

  criteria.keys
end

begin
  if ARGV.any?
    if ARGV == ["--help"] || ARGV == ["-h"]
      puts usage
      exit 0
    end
    warn usage
    exit 2
  end

  index_path = File.join(PLAN_DIR, "index.md")
  index, = read_document(index_path)
  INDEX_KEYS.each { |key| ensure_plan(index.key?(key), "plan/index.md: missing #{key}") }
  ensure_plan(index["target"] == "v0.1.13", "plan/index.md: target must be v0.1.13")
  ensure_plan(index["branch"] == "release/0.1.13", "plan/index.md: branch is incorrect")
  ensure_plan(index["baseline"] == "v0.1.12", "plan/index.md: baseline must be v0.1.12")
  ensure_plan(index["scope"] == "integration-hardening", "plan/index.md: scope is incorrect")
  ensure_plan(index["delivery_mode"] == "zero-host", "plan/index.md: delivery mode must be zero-host")
  ensure_plan(index["maintainer_budget_usd"] == 0, "plan/index.md: budget must be USD 0")
  ensure_plan(index["status_source"] == "phase-frontmatter", "plan/index.md: status source is incorrect")
  ensure_plan(git("cat-file", "-e", "#{index['baseline']}^{commit}")[2],
              "plan/index.md: baseline #{index['baseline']} is not a local commit")

  phase_paths = Dir.glob(File.join(PLAN_DIR, "[0-9][0-9]-*.md")).sort
  phases = {}
  phase_paths.each do |path|
    name = File.basename(path)
    match = name.match(PHASE_PATTERN)
    ensure_plan(match, "plan: invalid phase filename #{name}")
    id = match[1]
    ensure_plan(!phases.key?(id), "plan: duplicate phase #{id}")
    metadata, text = read_document(path)
    phases[id] = { id: id, path: path, metadata: metadata, text: text }
  end

  ensure_plan(index["phases"] == phases.keys,
              "plan/index.md: phases must be #{phases.keys.join(', ')} in order")
  extras = Dir.glob(File.join(PLAN_DIR, "*.md")).map { |path| File.basename(path) } -
           ["index.md", "deferred-hosted-demo.md"] - phase_paths.map { |path| File.basename(path) }
  ensure_plan(extras.empty?, "plan: unregistered markdown files: #{extras.join(', ')}")

  deferred, deferred_text = read_document(File.join(PLAN_DIR, "deferred-hosted-demo.md"))
  ensure_plan(deferred["status"] == "deferred", "deferred hosted demo must remain deferred")
  ensure_plan(deferred["activation"] == "explicit-maintainer-commission",
              "deferred hosted demo requires explicit maintainer commission")
  ensure_plan(deferred_text.include?("## Non-Negotiable Hosted Gates"),
              "deferred hosted demo must preserve its safety gates")

  all_ids = Set.new
  phases.each_value do |phase|
    validate_phase(phase, phases, index["target"]).each do |criterion|
      ensure_plan(all_ids.add?(criterion), "plan: duplicate criterion #{criterion}")
    end
  end

  statuses = phases.values.map { |phase| phase[:metadata]["status"] }
  overall = if statuses.all? { |status| status == "complete" }
              "complete"
            elsif statuses.include?("blocked")
              "blocked"
            elsif statuses.all? { |status| EVIDENCE_STATUSES.include?(status) }
              "evidence-ready"
            elsif statuses.any? { |status| status != "proposed" }
              "in-progress"
            else
              "proposed"
            end

  puts "check-plan: ok (#{phases.length} phases, #{all_ids.length} criteria, overall=#{overall})"
rescue PlanError => e
  warn "check-plan: #{e.message.sub(ROOT + '/', '')}"
  exit 1
end
