class Cvpn < Formula
  version '0.2.0'
  desc "Control VPN from the command line"
  homepage "https://github.com/dennis-kuypers/cdrust/cli"

  if OS.mac?
    case Hardware::CPU::type
    when :intel
      url "https://github.com/dennis-kuypers/cdrust/releases/download/cvpn-#{version}/cvpn-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "42c2e1373b6d209ce9a30998871a75bf10c738ed9b0e545c1fd5e232c6c857cd"
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