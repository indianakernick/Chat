fn main() {
    println!("cargo:rerun-if-changed=client/src");
    println!("cargo:rerun-if-changed=client/public");
    let status = std::process::Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir("client")
        .status()
        .unwrap();
    assert!(status.success());
}
