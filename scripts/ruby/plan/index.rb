#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"

HERE = File.realpath(__dir__)
DISPATCH = {
  "plan" => "check-plan.rb"
}

def usage
  <<~TEXT
    usage: ruby scripts/ruby/plan/index.rb <plan>

    Public entrypoint to the plan validation domain.
  TEXT
end

def main
  if ARGV.empty?
    warn usage
    return 2
  end
  target = DISPATCH[ARGV[0]]
  unless target
    warn "plan index: unknown command #{ARGV[0]}"
    warn usage
    return 2
  end
  exec File.join(HERE, "logic", target), *ARGV.drop(1)
end

exit main
