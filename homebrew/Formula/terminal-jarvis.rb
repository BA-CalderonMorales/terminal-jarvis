# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# Based on Federico Terzi's approach: https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/

class TerminalJarvis < Formula
  desc "A unified command center for AI coding tools"
  homepage "https://github.com/BA-CalderonMorales/terminal-jarvis"
  
  if OS.mac?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.46/terminal-jarvis-mac.tar.gz"
    sha256 "54541393e9d396082cea5cdd1ee06a2f65f4053fc373c91a91e8214b48bab94d"
  elsif OS.linux?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.46/terminal-jarvis-linux.tar.gz" 
    sha256 "54541393e9d396082cea5cdd1ee06a2f65f4053fc373c91a91e8214b48bab94d"
  end
  
  version "0.0.46"

  def install
    bin.install "terminal-jarvis"
  end

  test do
    system "#{bin}/terminal-jarvis", "--version"
  end
end
