class Cwk < Formula
  version '0.2.0'
  desc "Workflow tooling"
  homepage "https://github.com/dennis-kuypers/cdrust"

  if OS.mac?
    case Hardware::CPU::type
    when :intel
      url "https://github.com/dennis-kuypers/cdrust/releases/download/cwk-#{version}/cwk-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "6b1c0481bcd32ac49a53578be9c347b485c22b0924f6107c2c79a80a3481d888"
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