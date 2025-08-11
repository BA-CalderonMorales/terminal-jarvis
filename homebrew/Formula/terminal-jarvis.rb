# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# Based on Federico Terzi's approach: https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/

class TerminalJarvis < Formula
  desc "A unified command center for AI coding tools"
  homepage "https://github.com/BA-CalderonMorales/terminal-jarvis"
  
  if OS.mac?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.53/terminal-jarvis-mac.tar.gz"
    sha256 "2357ffa2bf837eb97b8183daeabc3ac2d0420f8f5eaaa32fa200511b6fc8f7c7"
  elsif OS.linux?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.53/terminal-jarvis-linux.tar.gz" 
    sha256 "2357ffa2bf837eb97b8183daeabc3ac2d0420f8f5eaaa32fa200511b6fc8f7c7"
  end
  
  version "0.0.53"

  def install
    bin.install "terminal-jarvis"
  end

  test do
    system "#{bin}/terminal-jarvis", "--version"
  end
end
