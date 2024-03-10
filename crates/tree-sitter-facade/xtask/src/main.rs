//! Cargo xtask definitions for the project.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

type Fallible<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> Fallible<()> {
    let help = r#"
xtask

USAGE:
    xtask [SUBCOMMAND]

FLAGS:
    -h, --help          Prints help information

SUBCOMMANDS:
    build
    check
    clippy
    doc
    format
    help                Prints this message or the help of the subcommand(s)
    init
    install
    test
"#
    .trim();

    let mut args: Vec<_> = std::env::args_os().collect();
    // remove "xtask" argument
    args.remove(0);

    let cargo_args = if let Some(dash_dash) = args.iter().position(|arg| arg == "--") {
        let c = args.drain(dash_dash + 1 ..).collect();
        args.pop();
        c
    } else {
        Vec::new()
    };

    let mut args = pico_args::Arguments::from_vec(args);

    let result = match args.subcommand()?.as_deref() {
        None => {
            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
            }
            Ok(())
        },
        Some("build") => subcommand::cargo::build(&mut args, &cargo_args),
        Some("check") => subcommand::cargo::check(&mut args, &cargo_args),
        Some("clippy") => subcommand::cargo::clippy(&mut args, &cargo_args),
        Some("doc") => subcommand::cargo::doc(&mut args, &cargo_args),
        Some("format") => subcommand::cargo::format(&mut args, &cargo_args),
        Some("init") => subcommand::cargo::init(&mut args),
        Some("install") => subcommand::cargo::install(&mut args, &cargo_args),
        Some("test") => subcommand::cargo::test(&mut args, &cargo_args),
        Some("udeps") => subcommand::cargo::udeps(&mut args, &cargo_args),
        Some("help") => {
            println!("{}\n", help);
            Ok(())
        },
        Some(subcommand) => Err(format!("unknown subcommand: {}", subcommand).into()),
    };
    crate::util::handle_result(result);

    let result = crate::util::handle_unused(&args);
    crate::util::handle_result(result);

    Ok(())
}

mod metadata {
    use std::path::{Path, PathBuf};

    pub fn cargo() -> crate::Fallible<String> {
        // NOTE: we use the cargo wrapper rather than the binary reported through the "CARGO" environment
        // variable because we need to be able to invoke cargo with different toolchains (e.g., +nightly)
        Ok(String::from("cargo"))
    }

    pub fn project_root() -> PathBuf {
        Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .unwrap()
            .to_path_buf()
    }
}

mod subcommand {
    pub mod cargo {
        use crate::metadata;
        use std::process::Command;

        // Run `cargo build` with custom options.
        pub fn build(args: &mut pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-build

USAGE:
    xtask build

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["build", "--package", "tree-sitter-facade"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo check` with custom options.
        pub fn check(args: &mut pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-check

USAGE:
    xtask check

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&["check", "--all-targets"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "tree-sitter-facade"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo clippy` with custom options.
        pub fn clippy(args: &mut pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-clippy

USAGE:
    xtask clippy

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "clippy", "--all-targets"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "tree-sitter-facade"]);
            cmd.args(cargo_args);
            cmd.args(&["--", "-D", "warnings"]);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo doc` with custom options.
        pub fn doc(args: &mut pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-doc

USAGE:
    xtask doc

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "doc"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo format` with custom options.
        pub fn format(args: &mut pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-format

USAGE:
    xtask format

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "fmt", "--all"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo init` with custom options.
        pub fn init(args: &mut pico_args::Arguments) -> crate::Fallible<()> {
            let help = r#"
xtask-init

USAGE:
    xtask init

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            crate::util::handle_unused(args)?;

            let status = {
                let mut cmd = Command::new("npm");
                cmd.args(&["ci"]);
                cmd.status()?
            };

            if status.success() {
                let assets_dir = format!("{}/../assets", env!("CARGO_MANIFEST_DIR"));
                let assets_dir = std::path::Path::new(&assets_dir);
                let mut cmd = Command::new("npx");
                cmd.current_dir(assets_dir);
                cmd.args(&["tree-sitter", "build-wasm", "../node_modules/tree-sitter-javascript"]);
                cmd.status()?;
            }

            Ok(())
        }

        // Run `cargo install` with custom options.
        pub fn install(args: &mut pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-install

USAGE:
    xtask install

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["install", "--path", "crates/cli"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo test` with custom options.
        pub fn test(args: &mut pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-test

USAGE:
    xtask test

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&["test", "--examples", "--lib", "--tests"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "tree-sitter-facade"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo udeps` with custom options.
        pub fn udeps(args: &mut pico_args::Arguments, cargo_args: &[std::ffi::OsString]) -> crate::Fallible<()> {
            let help = r#"
xtask-udep

USAGE:
    xtask udep

FLAGS:
    -h, --help          Prints help information
    -- '...'            Extra arguments to pass to the cargo command
"#
            .trim();

            if args.contains(["-h", "--help"]) {
                println!("{}\n", help);
                return Ok(());
            }

            crate::util::handle_unused(args)?;

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "udeps", "--all-targets"]);
            cmd.args(&["--package", "xtask"]);
            cmd.args(&["--package", "tree-sitter-facade"]);
            cmd.args(cargo_args);
            cmd.status()?;

            Ok(())
        }
    }
}

mod util {
    pub(super) fn handle_result<T>(result: crate::Fallible<T>) {
        if let Err(err) = result {
            println!("Error :: {}", err);
            std::process::exit(1);
        }
    }

    pub(super) fn handle_unused(args: &pico_args::Arguments) -> crate::Fallible<()> {
        use std::borrow::Borrow;
        let unused = args.clone().finish();
        if !unused.is_empty() {
            let mut message = String::new();
            for str in unused {
                message.push(' ');
                message.push_str(str.to_string_lossy().borrow());
            }
            Err(format!("unrecognized arguments '{}'", message).into())
        } else {
            Ok(())
        }
    }
}
