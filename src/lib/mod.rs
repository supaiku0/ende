use std::process;

pub mod worker;
mod path_helper;

static REQUIRED_EXECUTABLES: &'static [&'static str] = &["gpg", "tar"];

pub fn check_required_binaries() -> Result<(), Vec<&'static str>> {
    let mut missing: Vec<&'static str> = vec![];

    REQUIRED_EXECUTABLES.iter().for_each(|p| {
        if !check_binary(p) {
            missing.push(p);
        }
    });

    if !missing.is_empty() {
        println!("Required executables not found {:?}", missing);
        return Err(missing);
    }

    Ok(())
}

fn check_binary(name: &'static str) -> bool {
    let output = process::Command::new("which")
        .arg(name)
        .output().expect("Failed to execute which.");

    //println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    output.status.success()
}


#[test]
fn test_required_binaries() {
    assert!(check_required_binaries().is_ok());
}
