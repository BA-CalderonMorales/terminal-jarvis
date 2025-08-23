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
  version "0.0.62"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.62/terminal-jarvis-mac.tar.gz"
      sha256 "e1f72b82d080268837f5b55c5fc20730436a0f22739993806bfc0a0db2203f30"
    elsif Hardware::CPU.arm?
      url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.62/terminal-jarvis-mac.tar.gz"
      sha256 "e1f72b82d080268837f5b55c5fc20730436a0f22739993806bfc0a0db2203f30"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.62/terminal-jarvis-linux.tar.gz"
      sha256 "962b7f58a07db6ce3880fdfdff286b85da5eaa664a921fce8d4158bef1adfe4b"
    else
      # Fallback for other Linux architectures
      url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.62/terminal-jarvis-linux.tar.gz"
      sha256 "962b7f58a07db6ce3880fdfdff286b85da5eaa664a921fce8d4158bef1adfe4b"
    end
  end

  def install
    bin.install "terminal-jarvis"
  end

  test do
    system "#{bin}/terminal-jarvis", "--version"
  end
end