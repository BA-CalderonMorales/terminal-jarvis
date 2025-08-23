# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# Based on Federico Terzi's approach: https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/
#
# AUTOMATED RELEASE PROCESS:
# This file is automatically updated by the CD pipeline when you push a tag:
# 1. Push a git tag: git tag v0.0.X && git push origin v0.0.X
# 2. GitHub Actions (cd-multiplatform.yml) triggers on tag push
# 3. Pipeline builds binaries, creates GitHub release with assets
# 4. update-homebrew-tap job automatically updates BA-CalderonMorales/homebrew-terminal-jarvis
# 5. Real Formula gets updated with correct version, URLs, and SHA256 checksums
#
# This local file is just a staging template - the live Formula is in the homebrew-terminal-jarvis repo

class TerminalJarvis < Formula
  desc "A unified command center for AI coding tools"
  homepage "https://github.com/BA-CalderonMorales/terminal-jarvis"
  version "0.0.61"
  license "MIT"

  on_macos do
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.61/terminal-jarvis-mac.tar.gz"
    sha256 "1b531f95493211c322c13f81aef7bc47c60794db3eea72ab11119bdf3acd00d8"
  end

  on_linux do
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.61/terminal-jarvis-linux.tar.gz"
    sha256 "86b9374b464f5a0e65f0c484c160d394223e8a05af31e459bb4dc87e404ee06f"
  end

  def install
    bin.install "terminal-jarvis"
  end

  test do
    system "#{bin}/terminal-jarvis", "--version"
  end
end