class Omnimm < Formula
  desc "Git for AI Agent Knowledge - local-first agent memory and code intelligence"
  homepage "https://github.com/foysalkhan288-rgb/omnimem"
  version "0.1.0"
  license "Apache-2.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/foysalkhan288-rgb/omnimem/releases/download/v#{version}/omnimem-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_SHA256"
    else
      url "https://github.com/foysalkhan288-rgb/omnimem/releases/download/v#{version}/omnimem-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_SHA256"
    end
  end

  on_linux do
    url "https://github.com/foysalkhan288-rgb/omnimem/releases/download/v#{version}/omnimem-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "REPLACE_WITH_SHA256"
  end

  def install
    bin.install "omnimem"
  end

  test do
    system "#{bin}/omnimem", "--version"
  end
end