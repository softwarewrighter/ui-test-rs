use std::process::Command;

fn main() {
    // Get build host
    let build_host = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_HOST={}", build_host);

    // Get git commit hash
    let git_commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_COMMIT={}", git_commit);

    // Get build timestamp
    let build_time = chrono::Local::now().to_rfc3339();
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);

    // Rebuild if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");
}
