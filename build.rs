use std::process::{self, Command};

fn main() {
    let host = std::env::var_os("HOST").unwrap();
    let rustc = std::env::var_os("RUSTC").unwrap();
    let target = std::env::var_os("TARGET").unwrap();

    if host != target {
        eprintln!("Coopted LLVM cannot be cross-compiled.");
        process::exit(1);
    }

    let output = run(
        Command::new(rustc).args(&["--color", "always", "--print", "sysroot"]),
        Command::output,
    );

    if !output.status.success() {
        eprintln!("{}", output.status);
        process::exit(1);
    }

    let sysroot = match String::from_utf8(output.stdout) {
        Ok(mut utf8) => {
            let len = utf8.trim_end().len();
            utf8.truncate(len);
            utf8
        }
        Err(e) => {
            eprintln!("could not convert sysroot to UTF-8: {}", e);
            process::exit(1);
        }
    };

    println!("cargo:rerun-if-changed=");
    println!("cargo:rustc-link-lib=dylib=LLVM");
    println!("cargo:rustc-link-search=native={}/lib", sysroot);
}

fn run<T>(command: &mut Command, f: impl FnOnce(&mut Command) -> Result<T, std::io::Error>) -> T {
    println!("+ {:?}", command);
    match f(command) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}
