fn main() {
    // This is so sad ;-(
    // https://github.com/rust-lang/cargo/issues/2599

    println!("cargo:rerun-if-changed=client/public/channel.html");
    println!("cargo:rerun-if-changed=client/public/favicon.ico");
    println!("cargo:rerun-if-changed=client/public/login.html");

    println!("cargo:rerun-if-changed=client/src/assets/anonymous.png");

    println!("cargo:rerun-if-changed=client/src/components/Channel.vue");
    println!("cargo:rerun-if-changed=client/src/components/ChannelCreateDialog.vue");
    println!("cargo:rerun-if-changed=client/src/components/ChannelDeleteDialog.vue");
    println!("cargo:rerun-if-changed=client/src/components/Group.vue");
    println!("cargo:rerun-if-changed=client/src/components/GroupCreateDialog.vue");
    println!("cargo:rerun-if-changed=client/src/components/InviteDialog.vue");
    println!("cargo:rerun-if-changed=client/src/components/Message.vue");
    println!("cargo:rerun-if-changed=client/src/components/MessageList.vue");
    println!("cargo:rerun-if-changed=client/src/components/MessageSender.vue");
    println!("cargo:rerun-if-changed=client/src/components/ModalDialog.vue");
    println!("cargo:rerun-if-changed=client/src/components/ProfileNav.vue");
    println!("cargo:rerun-if-changed=client/src/components/StatusMessage.vue");

    println!("cargo:rerun-if-changed=client/src/pages/channel/App.vue");
    println!("cargo:rerun-if-changed=client/src/pages/channel/main.js");

    println!("cargo:rerun-if-changed=client/vue.config.js");

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
