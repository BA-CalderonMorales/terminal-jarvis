# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# Based on Federico Terzi's approach: https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/

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