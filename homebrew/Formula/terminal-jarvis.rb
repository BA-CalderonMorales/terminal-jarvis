# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# Based on Federico Terzi's approach: https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/

class TerminalJarvis < Formula
  desc "A unified command center for AI coding tools"
  homepage "https://github.com/BA-CalderonMorales/terminal-jarvis"
  
  if OS.mac?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.51/terminal-jarvis-mac.tar.gz"
    sha256 "e5d0f1153d127083ea26e75d5b3776de26351efa17f48bf24717c22ae17825a2"
  elsif OS.linux?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.51/terminal-jarvis-linux.tar.gz" 
    sha256 "e5d0f1153d127083ea26e75d5b3776de26351efa17f48bf24717c22ae17825a2"
  end
  
  version "0.0.51"

  def install
    bin.install "terminal-jarvis"
  end

  test do
    system "#{bin}/terminal-jarvis", "--version"
  end
end
