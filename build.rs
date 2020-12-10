fn main() {
    println!("cargo:rerun-if-changed=test.js");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=package-lock.json");
    std::process::Command::new("npm").arg("install").output().expect("could not install npm dependencies");
}
