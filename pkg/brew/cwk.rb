class Cwk < Formula
  version '0.3.0'
  desc "Workflow tooling"
  homepage "https://github.com/dennis-kuypers/cdrust"

  if OS.mac?
    case Hardware::CPU::type
    when :intel
      url "https://github.com/dennis-kuypers/cdrust/releases/download/cwk-#{version}/cwk-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "af3e0c31d663d2d9b6a28b863162b0e47e77ff865a8cc344b312f937014f4d7f"
    end
  end

  def install
    bin.install "cwk"

    # TODO: man pages
    # man1.install "doc/cwk.1"
    # TODO: completions...
    # bash_completion.install "complete/cwk.bash"
    # zsh_completion.install "complete/_cwk"
  end
end
