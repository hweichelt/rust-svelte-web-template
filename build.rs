use std::path::Path;

/// The frontend build output is embedded into release binaries with
/// `include_dir!`, which does not register the embedded files with cargo.
/// Tell cargo to rebuild when they change, and fail early with a clear
/// message when a release build would embed nothing.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let build_dir = Path::new("frontend/build");
    if build_dir.is_dir() {
        println!("cargo:rerun-if-changed=frontend/build");
    }

    let release = std::env::var("PROFILE").as_deref() == Ok("release");
    if release && !build_dir.join("index.html").is_file() {
        panic!(
            "frontend/build/index.html not found. Release builds embed the web app, \
             so run `npm run build` in frontend/ first (see README)."
        );
    }
}
