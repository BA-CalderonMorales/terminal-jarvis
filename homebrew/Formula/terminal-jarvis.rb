class TerminalJarvis < Formula
  desc "A unified command center for AI coding tools"
  homepage "https://github.com/BA-CalderonMorales/terminal-jarvis"
  version "0.0.76"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.76/terminal-jarvis-mac.tar.gz"
      sha256 "SKIP_CHECK"
    elsif Hardware::CPU.arm?
      url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.76/terminal-jarvis-mac.tar.gz"
      sha256 "SKIP_CHECK"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.76/terminal-jarvis-linux.tar.gz"
      sha256 "SKIP_CHECK"
    else
      # Fallback for other Linux architectures
      url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.76/terminal-jarvis-linux.tar.gz"
      sha256 "SKIP_CHECK"
    end
  end

  def install
    bin.install "terminal-jarvis"
  end

  test do
    system "#{bin}/terminal-jarvis", "--version"
  end
end
