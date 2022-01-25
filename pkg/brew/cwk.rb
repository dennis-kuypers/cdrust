class Cwk < Formula
  version '0.2.1'
  desc "Workflow tooling"
  homepage "https://github.com/dennis-kuypers/cdrust"

  if OS.mac?
    case Hardware::CPU::type
    when :intel
      url "https://github.com/dennis-kuypers/cdrust/releases/download/cwk-#{version}/cwk-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "34803626780ca981788f058fc2de67225a398313477010fd243cb54901808536"
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