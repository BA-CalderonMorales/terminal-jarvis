#!/usr/bin/env ruby
# frozen_string_literal: true

require "yaml"

ROOT = File.expand_path("..", __dir__)
PLAN_DIR = File.join(ROOT, "plan")
PAGE_PATTERN = /\A(\d{2})-tj-hardening-[a-z0-9]+(?:-[a-z0-9]+)*\.md\z/
ALLOWED_STATUSES = %w[proposed ready in-progress blocked review complete].freeze
ALLOWED_MODES = %w[pending-page-12-decision kernel-hosted cloudflare-hosted zero-host].freeze

class PlanError < StandardError; end

def ensure_plan(condition, message)
  raise PlanError, message unless condition
end

def read_document(path)
  text = File.read(path)
  match = text.match(/\A---\r?\n(?<yaml>.*?)\r?\n---\r?\n/m)
  ensure_plan(match, "#{path}: missing YAML front matter")
  metadata = YAML.safe_load(
    match[:yaml], permitted_classes: [], permitted_symbols: [], aliases: false
  )
  ensure_plan(metadata.is_a?(Hash), "#{path}: front matter must be a mapping")
  [metadata, text]
rescue Psych::SyntaxError => e
  raise PlanError, "#{path}: invalid YAML front matter: #{e.message}"
end

def page_criteria(path, text)
  criteria = text.scan(/^- \[[ xX]\] `([A-Z][A-Z0-9]{2}-\d{2})`/).flatten
  evidence = text.scan(/^\| ([A-Z][A-Z0-9]{2}-\d{2}) \|/).flatten
  ensure_plan(criteria.any?, "#{path}: no acceptance criteria")
  ensure_plan(criteria == evidence, "#{path}: acceptance criteria and evidence rows differ")
  ensure_plan(criteria.uniq == criteria, "#{path}: duplicate acceptance criterion")
  criteria
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
  %w[target branch status status_source].each do |field|
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

    criteria = page_criteria(path, text)
    all_criteria.concat(criteria)
    if metadata["status"] == "complete"
      ensure_plan(!text.match?(/^- \[ \]/), "#{path}: complete page has unchecked work")
      pending_rows = text.lines.select do |line|
        line.start_with?("|") && line.match?(/\|\s*pending\s*\|/i)
      end
      ensure_plan(pending_rows.empty?, "#{path}: complete page has pending table values")
    end

    if id.to_i >= 13
      ensure_plan(ALLOWED_MODES.include?(metadata["delivery_mode"]),
                  "#{path}: invalid or missing delivery_mode")
    end
    pages[id] = { path: path, metadata: metadata, text: text }
  end
  ensure_plan(all_criteria.uniq == all_criteria, "acceptance criterion IDs must be globally unique")

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

  mode_pages = pages.select { |id, _| id.to_i >= 13 }.values
  modes = mode_pages.map { |page| page[:metadata]["delivery_mode"] }.uniq
  ensure_plan(modes.length == 1, "pages 13 onward must use one delivery_mode")
  if pages.fetch("12")[:metadata]["status"] == "complete"
    ensure_plan(modes.first != "pending-page-12-decision",
                "completed provider selection must propagate a delivery_mode")
  end

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
  end

  puts "check-plan: ok (#{pages.length} pages, #{all_criteria.length} criteria, mode #{modes.first})"
rescue PlanError => e
  warn "check-plan: #{e.message}"
  exit 1
end
