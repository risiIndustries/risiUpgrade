use std::{
    env,
    io::{stderr, stdout, Write},
    process::Command,
};

use glib_build_tools::compile_resources;

fn execute_cmd(cmd: &mut Command) {
    let cmd = cmd.output().expect("Failed to execute command");

    if !cmd.status.success() {
        stdout().write_all(&cmd.stdout).unwrap();
        stderr().write_all(&cmd.stderr).unwrap();
    }
}

fn main() {
    let base_id = "io.risi.upgrade";
    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rerun-if-changed=data");

    compile_resources(
        "data",
        &format!("data/{}.gresources.xml", base_id),
        "resources.gresource",
    );

    println!(
        "cargo:rustc-env=RESOURCES_FILE={}/resources.gresource",
        out_dir
    );

    if profile == "release" {
        println!("cargo:rustc-env=APP_PROFILE=release");
        println!("cargo:rustc-env=APP_ID={}", base_id);

        println!("cargo:rustc-env=RUST_LOG=info");
    } else if profile == "debug" {
        println!("cargo:rustc-env=APP_PROFILE=development");
        println!("cargo:rustc-env=APP_ID={}", base_id);

        println!("cargo:rustc-env=RUST_LOG=trace");
    }

    execute_cmd(
        Command::new("sudo")
            .arg("cp")
            .arg(&format!("data/{}.gschema.xml", base_id))
            .arg("/usr/share/glib-2.0/schemas/"),
    );

    execute_cmd(
        Command::new("sudo")
            .arg("glib-compile-schemas")
            .arg("/usr/share/glib-2.0/schemas/"),
    );

    execute_cmd(
        Command::new("sudo")
            .arg("cp")
            .arg(format!("data/{}.desktop", base_id))
            .arg("/usr/share/applications"),
    );
}
