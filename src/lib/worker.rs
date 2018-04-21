use std::path::PathBuf;
use std::thread;
use std::sync::mpsc::{Sender};

#[derive(Clone)]
pub enum WorkerMode {
    Encryption,
    Decryption
}

pub struct Worker {

    mode: WorkerMode,
    sender: Sender<String>
}

struct ThreadWorker {
    mode: WorkerMode,
    sender: Sender<String>
}

impl Worker {

    pub fn new(mode: WorkerMode, sender: Sender<String>) -> Worker {
        Worker {
            mode,
            sender: sender
        }
    }

    pub fn process(&self, data: &String) {

        let worker = ThreadWorker {
            mode: self.mode.clone(),
            sender: self.sender.clone()
        };

        let payload = data.clone();
        thread::spawn(move || {
            worker.process(&payload)
        });
    }

}

impl ThreadWorker {

    pub fn process(&self, payload: &String) {
        let paths = self.get_paths(payload);

        match paths.len() {
            0 => return,
            1 => self.process_path(paths.first().unwrap()),
            _ => self.process_batch(&paths)
        }


        self.send("DONE".to_owned());
    }

    fn process_path(&self, path: &PathBuf) {
        let formatted = format!("Processing single path {:?}", path);
        println!("{:?}", formatted);
        self.send(formatted);
    }

    fn process_batch(&self, paths: &Vec<PathBuf>) {
        let formatted = format!("Processing paths {:?}", paths);
        println!("{:?}", formatted);
        self.send(formatted);
    }

    fn get_paths(&self, data: &String) -> Vec<PathBuf> {

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
            paths.push(path);
        }

        paths
    }

    fn send(&self, data: String) {
        self.sender.send(data).unwrap();
    }
}
