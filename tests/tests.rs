use std::fs::File;
use std::io::Read;
use std::process::Command;

#[test]
fn cargo_readme_up_to_date() {
    println!("Checking that `cargo readme > README.md` is up to date...");

    let expected = Command::new("cargo")
        .arg("readme")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("should run `cargo readme` OK")
        .stdout;
    let expected = String::from_utf8_lossy(&expected);

    let actual = {
        let mut file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))
            .expect("should open README.md file");
        let mut s = String::new();
        file.read_to_string(&mut s)
            .expect("should read contents of file to string");
        s
    };

    if actual != expected {
        panic!("Run `cargo readme > README.md` to update README.md");
    }
}

#[test]
fn snip_me() {
    let status = Command::new("cargo")
        .args(&["run", "--", "-o"])
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snipped.wasm"))
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/hello.wasm"))
        .arg("_ZN5hello7snip_me17hf15dbd799e7ad6aaE")
        .status()
        .unwrap();
    assert!(status.success());

    let expected = {
        let mut file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/expected.wasm"))
            .expect("should open README.md file");
        let mut e = Vec::new();
        file.read_to_end(&mut e)
            .expect("should read contents of file to vec");
        e
    };

    let actual = {
        let mut file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snipped.wasm"))
            .expect("should open README.md file");
        let mut a = Vec::new();
        file.read_to_end(&mut a)
            .expect("should read contents of file to vec");
        a
    };

    if actual != expected {
        panic!("snipping `snip_me` did not result in expected wasm file");
    }
}
