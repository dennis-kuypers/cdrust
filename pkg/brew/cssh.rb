class Cssh < Formula
  version '0.1.0'
  desc "SSH into ec2 instances"
  homepage "https://github.com/dennis-kuypers/cdrust/cli"

  if OS.mac?
      url "https://github.com/dennis-kuypers/cdrust/releases/download/#{version}/cssh-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "585c18350cb8d4392461edd6c921e6edd5a97cbfc03b567d7bd440423e118082"
  end

  def install
    bin.install "cssh"

    # man1.install "doc/cssh.1"
    # bash_completion.install "complete/cssh.bash"
    # zsh_completion.install "complete/_cssh"
  end
end