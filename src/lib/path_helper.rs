use std::path::PathBuf;

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

        paths.push(path);
    }

    paths
}
