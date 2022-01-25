class Cwk < Formula
  version '0.2.2'
  desc "Workflow tooling"
  homepage "https://github.com/dennis-kuypers/cdrust"

  if OS.mac?
    case Hardware::CPU::type
    when :intel
      url "https://github.com/dennis-kuypers/cdrust/releases/download/cwk-#{version}/cwk-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "6c329b9082b8cc24b9f5e9ca5483c4f7270fc15a44e749005772314f8e8d2c5b"
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