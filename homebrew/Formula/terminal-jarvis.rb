# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# Based on Federico Terzi's approach: https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/

class TerminalJarvis < Formula
  desc "A unified command center for AI coding tools"
  homepage "https://github.com/BA-CalderonMorales/terminal-jarvis"
  
  if OS.mac?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.47/terminal-jarvis-mac.tar.gz"
    sha256 "a6a02c58b3262d1ef103f343f71efe92527816a8046db07656fcb958f8bf9cef"
  elsif OS.linux?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.47/terminal-jarvis-linux.tar.gz" 
    sha256 "a6a02c58b3262d1ef103f343f71efe92527816a8046db07656fcb958f8bf9cef"
  end
  
  version "0.0.47"

  def install
    bin.install "terminal-jarvis"
  end

  test do
    system "#{bin}/terminal-jarvis", "--version"
  end
end
