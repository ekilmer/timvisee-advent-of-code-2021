extern crate rustc_version;
use rustc_version::{version, version_meta, Channel};

fn main() {
    // Assert we haven't travelled back in time
    assert!(version().unwrap().major >= 1);

    // Sanity checking for unstable features based on channel
    match version_meta().unwrap().channel {
        Channel::Nightly => (),
        channel => {
            if cfg!(feature = "unstable-features") {
                println!("cargo:warning=You can only use 'unstable-features' feature with the 'Nightly' channel. Your rustc compiler channel is '{channel:?}'. Try using 'cargo +nightly ...'.");
                // Should we fail in build.rs or just let the compiler fail?
            }
        }
    }
}
