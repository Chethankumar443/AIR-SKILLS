class Air < Formula
  desc "AI Project Bootstrap & Skill Manager"
  homepage "https://github.com/Chethankumar443/AIR-SKILLS"
  version "1.0.0"
  license "MIT"

  if OS.mac? && Hardware::CPU.intel?
    url "https://github.com/Chethankumar443/AIR-SKILLS/releases/download/v1.0.0/air-x86_64-apple-darwin.tar.gz"
    sha256 "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  elsif OS.mac? && Hardware::CPU.arm?
    url "https://github.com/Chethankumar443/AIR-SKILLS/releases/download/v1.0.0/air-aarch64-apple-darwin.tar.gz"
    sha256 "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  elsif OS.linux?
    url "https://github.com/Chethankumar443/AIR-SKILLS/releases/download/v1.0.0/air-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  end

  def install
    bin.install "air"
  end

  test do
    assert_match "AIR.SKILLS CLI v1.0.0", shell_output("#{bin}/air version")
  end
end
