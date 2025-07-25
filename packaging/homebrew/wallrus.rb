class Wallrus < Formula
  desc "Cross-platform wallpaper manager with native Wayland protocol support and multi-DE compatibility"
  homepage "https://github.com/pi22by7/wallrus"
  url "https://github.com/pi22by7/wallrus/archive/v0.3.0.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "wallrus", shell_output("#{bin}/wallrus --version")
  end
end