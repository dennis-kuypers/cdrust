class Cssh < Formula
  version '0.3.0'
  desc "SSH into ec2 instances"
  homepage "https://github.com/dennis-kuypers/cdrust/cli"

  if OS.mac?
    case Hardware::CPU::type
    when :intel
      url "https://github.com/dennis-kuypers/cdrust/releases/download/cssh-#{version}/cssh-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "5654fb471c1ceb803b9f8e4ab8ee1822d24401c280964f616c1eff21cdafed91"
    end
  end

  def install
    bin.install "cssh"

    # TODO: man pages
    # man1.install "doc/cssh.1"
    # TODO: completions...
    # bash_completion.install "complete/cssh.bash"
    # zsh_completion.install "complete/_cssh"
  end
end