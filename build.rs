use std::process::Command;

fn qmake_query(var: &str) -> Result<String, std::io::Error> {
    let qmake = std::env::var("QMAKE").unwrap_or_else(|_| "qmake".to_string());
    let output = Command::new(qmake).args(&["-query", var]).output()?;
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "qmake returned with error:\n{}\n{}",
                std::str::from_utf8(&output.stderr).unwrap_or_default(),
                std::str::from_utf8(&output.stdout).unwrap_or_default()
            ),
        ));
    }
    Ok(std::str::from_utf8(&output.stdout)
        .expect("UTF-8 conversion failed")
        .trim()
        .to_string())
}

fn main() {
    let mut cfg = cpp_build::Config::new();

    let qt_include_path = qmake_query("QT_INSTALL_HEADERS").expect("QMAKE");
    let qt_include_path = qt_include_path.trim();

    let contains_cpp = ["sailfishapp.rs"];

    for f in &contains_cpp {
        println!("cargo:rerun-if-changed=src/{}", f);
    }

    cfg.include(&qt_include_path)
        .include("/usr/include/auroraapp/")
        .include(format!("{}/QtCore", qt_include_path))
        // -W deprecated-copy triggers some warnings in old Jolla's Qt distribution.
        // It is annoying to look at while developing, and we cannot do anything about it
        // ourselves.
        .flag("-Wno-deprecated-copy")
        .build("src/lib.rs");

    let libs = ["auroraapp", "EGL"];
    for lib in &libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
}
