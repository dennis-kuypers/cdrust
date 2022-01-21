class Cssh < Formula
  version '0.2.0'
  desc "Control VPN from the command line"
  homepage "https://github.com/dennis-kuypers/cdrust/cli"

  if OS.mac?
    case Hardware::CPU::type
    when :intel
      url "https://github.com/dennis-kuypers/cdrust/releases/download/cvpn-#{version}/cvpn-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "55f5bde8a5568676d3a64722ba18e9954500418d641680b84bbebe763d1fc994"
    end
  end

  def install
    bin.install "cvpn"

    # TODO: man pages
    # man1.install "doc/cvpn.1"
    # TODO: completions...
    # bash_completion.install "complete/cvpn.bash"
    # zsh_completion.install "complete/_cvpn"
  end
end