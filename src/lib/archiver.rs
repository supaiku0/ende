use std::path::PathBuf;
use std::env;
use std::process;
use std::fs;

static TEMP_ARCHIVE: &'static str = "ende.tar.gz";

pub fn get_paths(data: &String) -> Vec<PathBuf> {

    let split: Vec<String> = data
        .split('\n')
        .filter_map(|s| if !s.is_empty() { return Some(s) } else { return None })
        .map(|s| s.to_owned())
        .collect();

    let mut paths: Vec<PathBuf> = vec![];
    for item in split {
        let mut line = item.clone();

        if line.starts_with("file://") {
            line = line.replace("file://", "");
        }

        let path = PathBuf::from(line);
        if !path.exists() {
            println!("{:?} does not exist!?", path);
            continue;
        }

        if !path.is_absolute() {
            println!("{:?} needs to be absolute!?", path);
            continue;
        }

        paths.push(path);
    }

    paths
}

pub fn create_archive(paths: &Vec<PathBuf>) -> PathBuf {

    let mut temp = temp_dir();
    if temp.exists() {
        // REMOVE tmp file
    }

    paths.iter().for_each(|path| {
        process::Command::new("cp")
            .arg(path.to_str().unwrap())
            .arg(&temp.to_str().unwrap())
            .output().expect("Failed to copy to temp dir.");
    });

    let mut command = process::Command::new("tar");
    command.arg("-C");
    command.arg(temp.to_str().unwrap());
    command.arg("-zcf");

    let temp_clone = temp.clone();
    let temp_prefix = temp_clone.to_str().unwrap();
    let stripped_paths: Vec<&str> = paths.iter()
        .map(|path| path.strip_prefix(temp_prefix).unwrap().to_str().unwrap())
        .collect();

    temp.push(TEMP_ARCHIVE);
    command.arg(temp.to_str().unwrap());
    stripped_paths.iter().for_each(|path| {
        command.arg(path);
    });

    let output = command.output().expect("Failed to create archive.");
    assert!(output.status.success());
    temp
}

fn temp_dir() -> PathBuf {
    let mut temp = env::temp_dir();
    temp.push("ende");

    if !temp.exists() {
        fs::DirBuilder::new().create(&temp).expect("Failed to create temp dir.");
    }

    temp
}

#[test]
fn test_create_archive() {
    let mut temp = temp_dir();

    let temp_file = [temp.to_str().unwrap(), "/foo.txt"].join("");
    fs::File::create(&temp_file);

    let paths = vec![PathBuf::from(&temp_file)];

    create_archive(&paths);

    temp.push(TEMP_ARCHIVE);
    assert!(temp.exists());
}

#[test]
fn test_create_mixed_archive() {
    let mut temp = temp_dir();

    let temp_file = [temp.to_str().unwrap(), "/foo.txt"].join("");
    fs::File::create(&temp_file);

    let temp_dir = [temp.to_str().unwrap(), "/bar"].join("");
    fs::DirBuilder::new().create(&temp_dir);

    let paths = vec![
        PathBuf::from(&temp_file),
        PathBuf::from(&temp_dir)
    ];

    create_archive(&paths);

    temp.push(TEMP_ARCHIVE);
    assert!(temp.exists());
}
