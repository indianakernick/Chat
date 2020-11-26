fn main() {
    // This is so sad ;-(
    // https://github.com/rust-lang/cargo/issues/2599
    // Having to update this script every time I add a component is probably not
    // worth the effort to be honest.

    println!("cargo:rerun-if-changed=client/public/index.html");
    println!("cargo:rerun-if-changed=client/src/App.vue");
    println!("cargo:rerun-if-changed=client/src/main.js");
    println!("cargo:rerun-if-changed=client/src/components/Message.vue");
    println!("cargo:rerun-if-changed=client/src/components/MessageList.vue");

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