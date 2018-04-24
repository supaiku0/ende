use std::path::PathBuf;
use std::thread;
use std::sync::mpsc::{Sender};
use path_helper;

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
        let paths = path_helper::get_paths(payload);
        if paths.is_empty() {
            self.send("BRLRLRLR".to_owned());
            self.send("DONE".to_owned());
        }

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

    fn send(&self, data: String) {
        self.sender.send(data).unwrap();
    }
}