fn main() {
    // This is so sad ;-(
    // https://github.com/rust-lang/cargo/issues/2599

    println!("cargo:rerun-if-changed=client/vue.config.js");

    println!("cargo:rerun-if-changed=client/public/channel.html");
    println!("cargo:rerun-if-changed=client/src/pages/channel/App.vue");
    println!("cargo:rerun-if-changed=client/src/pages/channel/main.js");

    println!("cargo:rerun-if-changed=client/public/login.html");
    println!("cargo:rerun-if-changed=client/src/pages/login/App.vue");
    println!("cargo:rerun-if-changed=client/src/pages/login/main.js");

    println!("cargo:rerun-if-changed=client/src/components/Login.vue");
    println!("cargo:rerun-if-changed=client/src/components/Message.vue");
    println!("cargo:rerun-if-changed=client/src/components/MessageList.vue");
    println!("cargo:rerun-if-changed=client/src/components/ProfileNav.vue");
    println!("cargo:rerun-if-changed=client/src/components/StatusMessage.vue");

    let build = match std::env::var("PROFILE").unwrap().as_str() {
        "debug" => "build-dev",
        "release" => "build-prod",
        _ => panic!()
    };

    let status = std::process::Command::new("npm")
        .arg("run")
        .arg(build)
        .current_dir("client")
        .status()
        .unwrap();
    assert!(status.success());
}
