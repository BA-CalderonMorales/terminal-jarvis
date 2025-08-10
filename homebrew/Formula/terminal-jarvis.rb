# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# Based on Federico Terzi's approach: https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/

class TerminalJarvis < Formula
  desc "A unified command center for AI coding tools"
  homepage "https://github.com/BA-CalderonMorales/terminal-jarvis"
  
  if OS.mac?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.50/terminal-jarvis-mac.tar.gz"
    sha256 "f0c0e271f5fa49339411e97898b723be39235bb58243107a29f8abb8d56ef35f"
  elsif OS.linux?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.50/terminal-jarvis-linux.tar.gz" 
    sha256 "f0c0e271f5fa49339411e97898b723be39235bb58243107a29f8abb8d56ef35f"
  end
  
  version "0.0.50"

  def install
    bin.install "terminal-jarvis"
  end

  test do
    system "#{bin}/terminal-jarvis", "--version"
  end
end
