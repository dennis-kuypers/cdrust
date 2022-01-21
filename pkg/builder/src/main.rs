//! # builder
//!
//! Quick and dirty helper to pack and release stuff
//!
//! Release steps include:
//! - set version in `Cargo.toml` files
//! - build `--release`
//! - tar the binaries
//! - upload to GH releases
//! - update homebrew formulae (edit and push)
//!
//! Currently this tool assumes that binaries are built with the correct version and just tars them.
//!
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;

static USAGE: &str = "USAGE: builder <binary_name> <version>\n`builder cssh 0.2.0`";

fn main() {
    let mut args = std::env::args();
    let binary = args.nth(1).expect(USAGE);
    let version = args.next().expect(USAGE);

    let release_path = std::path::Path::new("../../target/release");
    let target_path = std::path::Path::new("target/deploy");

    // would fail if already exists
    let _ = std::fs::DirBuilder::new().create(target_path);

    let tgz_name = &format!("{binary}-{version}-x86_64-apple-darwin.tar.gz");
    let target_file = target_path.join(tgz_name);
    let binary_file = release_path.join(&binary);

    let tar_gz = File::create(&target_file).expect("Can not create target file");
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_file(
        format!("bin/{binary}"),
        &mut File::open(binary_file).expect("Can not find binary"),
    )
    .expect("failed to add file to tar");
    eprintln!("Successfully packed {}", target_file.display());
}
