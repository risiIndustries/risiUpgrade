use std::{
    env,
    io::{stderr, stdout, Write},
    process::Command,
};

use glib_build_tools::compile_resources;

// APP CONFIGURATION
const BASE_ID: &'static str = "io.risi.upgrade";
const DATADIR: &'static str = "/usr/share/risiUpgrade";

fn execute_cmd(cmd: &mut Command) {
    let cmd = cmd.output().expect("Failed to execute command");

    if !cmd.status.success() {
        stdout().write_all(&cmd.stdout).unwrap();
        stderr().write_all(&cmd.stderr).unwrap();
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rerun-if-changed=data");

    compile_resources(
        "data",
        &format!("data/{}.gresources.xml", BASE_ID),
        "resources.gresource",
    );

    println!(
        "cargo:rustc-env=RESOURCES_FILE={}/resources.gresource",
        out_dir
    );

    println!("cargo:rustc-env=APP_ID={}", BASE_ID);
    println!("cargo:rustc-env=DATA_DIR={}", DATADIR);

    if profile == "release" {
        println!("cargo:rustc-env=APP_PROFILE=release");

        println!("cargo:rustc-env=RUST_LOG=info");
    } else if profile == "debug" {
        println!("cargo:rustc-env=APP_PROFILE=development");

        println!("cargo:rustc-env=RUST_LOG=trace");
    }

    execute_cmd(
        Command::new("sudo")
            .arg("cp")
            .arg(&format!("data/{}.gschema.xml", BASE_ID))
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
            .arg(format!("data/{}.desktop", BASE_ID))
            .arg("/usr/share/applications"),
    );

    // This has to be done manually because cargo as root doesn't work well
    execute_cmd(Command::new("sudo").arg("mkdir").arg("-p").arg(DATADIR));

    execute_cmd(
        Command::new("sudo")
            .arg("cp")
            .arg("data/upgrade.sh")
            .arg(DATADIR),
    );
}

