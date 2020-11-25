fn main() {
    println!("cargo:rerun-if-changed=client/src");
    println!("cargo:rerun-if-changed=client/public");

    let build = match std::env::var("PROFILE").unwrap().as_str() {
        "debug" => "build-dev",
        "release" => "build-prod",
        _ => ""
    };
    assert!(!build.is_empty());

    let status = std::process::Command::new("npm")
        .arg("run")
        .arg(build)
        .current_dir("client")
        .status()
        .unwrap();
    assert!(status.success());
}
