# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# Based on Federico Terzi's approach: https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/

class TerminalJarvis < Formula
  desc "A unified command center for AI coding tools"
  homepage "https://github.com/BA-CalderonMorales/terminal-jarvis"
  
  if OS.mac?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.48/terminal-jarvis-mac.tar.gz"
    sha256 "1b2409976f98ae2758eeca67faa1c374894beb8f809e2e417c641bc522dc7d40"
  elsif OS.linux?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.48/terminal-jarvis-linux.tar.gz" 
    sha256 "1b2409976f98ae2758eeca67faa1c374894beb8f809e2e417c641bc522dc7d40"
  end
  
  version "0.0.49"

  def install
    bin.install "terminal-jarvis"
  end

  test do
    system "#{bin}/terminal-jarvis", "--version"
  end
end
