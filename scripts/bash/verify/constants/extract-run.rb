#!/usr/bin/env ruby
# Print a workflow step's `run:` body, located by step name, so a rehearsal can
# execute the workflow's real bytes instead of a copy that can drift from it.
#   usage: extract-run.rb <workflow.yml> <step name>
require "yaml"
doc = YAML.unsafe_load_file(ARGV[0])
doc.fetch("jobs").each_value do |job|
  (job["steps"] || []).each do |step|
    next unless step["name"] == ARGV[1]
    print step.fetch("run")
    exit 0
  end
end
warn "step not found: #{ARGV[1]}"
exit 1
