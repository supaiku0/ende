use std::path::PathBuf;
use std::process;

// gpg --cipher-algo AES256 --symmetric filename
pub fn encrypt(path: &PathBuf) {
    process::Command::new("gpg")
        .arg("--cipher-algo AES256")
        .arg("--symmetric")
        .arg("xxx")
        .output();
}

// gpg --output filename.out --decrypt filename
pub fn decrypt(path: &PathBuf) {
    process::Command::new("gpg")
        .arg("--output xxxx")
        .arg("--decrypt xxx")
        .output();
}
