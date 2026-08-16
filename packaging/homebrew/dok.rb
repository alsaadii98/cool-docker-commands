# Homebrew formula for dok.
#
# Lives in the tap repo at alsaadii98/homebrew-tap as Formula/dok.rb.
# The release workflow bumps `url` and `sha256` on every tagged release.
class Dok < Formula
  desc "Docker output, made readable - what eza is to ls"
  homepage "https://github.com/alsaadii98/cool-docker-commands"
  url "https://github.com/alsaadii98/cool-docker-commands/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACED_ON_RELEASE"
  license "MIT"
  head "https://github.com/alsaadii98/cool-docker-commands.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "dok", shell_output("#{bin}/dok --version")
    # Without a daemon dok must fail loudly rather than hang or print garbage.
    output = shell_output("DOCKER_HOST=unix:///nonexistent #{bin}/dok ps 2>&1", 1)
    assert_match "docker", output.downcase
  end
end
