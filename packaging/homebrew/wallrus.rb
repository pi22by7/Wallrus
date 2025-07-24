class Wallrus < Formula
  desc "A fast, cross-platform wallpaper management tool with Hyprland support"
  homepage "https://github.com/yourusername/wallrus"
  url "https://github.com/yourusername/wallrus/archive/v0.2.0.tar.gz"
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