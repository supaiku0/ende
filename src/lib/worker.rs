use std::path::PathBuf;
use std::thread;
use std::sync::mpsc::{Sender};
use archiver;

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

    pub fn process(&self, paths: Vec<PathBuf>, passphrase: String) {
        assert!(!paths.is_empty());
        assert!(!passphrase.is_empty());

        println!("Got passphrase: {:?}", passphrase);

        let worker = ThreadWorker {
            mode: self.mode.clone(),
            sender: self.sender.clone()
        };

        thread::spawn(move || {
            worker.process(paths, passphrase)
        });
    }

}

impl ThreadWorker {

    pub fn process(&self, paths: Vec<PathBuf>, passphrase: String) {

        let target_path;
        if paths.len() > 1 {
            target_path = archiver::create_archive(&paths);
        } else {
            target_path = paths.first().unwrap().clone();
        }

        self.process_file(&target_path, &passphrase);
        self.send("DONE".to_owned());
    }

    fn process_file(&self, path: &PathBuf, passphrase: &String) {
        let formatted = format!("Processing file {:?}", path);
        println!("{:?}", formatted);




        self.send(formatted);
    }

    fn send(&self, data: String) {
        self.sender.send(data).unwrap();
    }
}
