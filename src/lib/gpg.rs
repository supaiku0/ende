use std::path::PathBuf;
use std::process;

// gpg \
// --symmetric \
// --batch \
// --cipher-algo=AES256 \
// --passphrase 1234 \
// --output test.bin \
// test.txt
pub fn encrypt(path: &PathBuf, passphrase: &String) {
    let result = process::Command::new("gpg")
        .arg("--symmetric")
        .arg("--cipher-algo=AES256")
        .arg("--batch")
        .arg("--passphrase")
        .arg(passphrase)
        .arg("--output")
        .arg(path.with_extension("").to_str().unwrap())
        .arg(path.to_str().unwrap())
        .output();

    println!("{:?}", result);
    match result {
        Ok(output) => {

            // TODO: bubble error/success to UI
            if !output.status.success() {
                let err = String::from_utf8(output.stderr).unwrap();
                println!("{:?}", err);
            } else {
                println!("{:?}", String::from_utf8(output.stdout));
            }

        },
        Err(e) => println!("{:?}", e)
    }
}

// gpg \
// --decrypt \
// --batch \
// --passphrase 1234 \
// --output test.txt \
// test.bin
pub fn decrypt(path: &PathBuf, passphrase: &String) {
    let result = process::Command::new("gpg")
        .arg("--decrypt")
        .arg("--batch")
        .arg("--passphrase")
        .arg(passphrase)
        .arg("--output")
        .arg(path.with_extension("out").to_str().unwrap())
        .arg(path.to_str().unwrap())
        .output();

    println!("{:?}", result);
    match result {
        Ok(output) => {

            // TODO: bubble error/success to UI
            if !output.status.success() {
                let err = String::from_utf8(output.stderr).unwrap();
                println!("{:?}", err);
            } else {
                println!("{:?}", String::from_utf8(output.stdout));
            }

        },
        Err(e) => println!("{:?}", e)
    }
}
