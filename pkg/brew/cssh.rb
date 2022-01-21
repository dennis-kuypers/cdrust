class Cssh < Formula
  version '0.2.0'
  desc "SSH into ec2 instances"
  homepage "https://github.com/dennis-kuypers/cdrust/cli"

  if OS.mac?
    case Hardware::CPU::type
    when :intel
      url "https://github.com/dennis-kuypers/cdrust/releases/download/cssh-#{version}/cssh-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "6cfe697f3ba3c8067b24183465b5aed82dca195f9bf3ee3e4335e15bb949b3b1"
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