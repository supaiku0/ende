use std::path::PathBuf;
use std::thread;
use std::sync::mpsc::{Sender};
use gpg;
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

        match self.mode {
            WorkerMode::Encryption => {
                let target_path;
                if paths.len() > 1 {
                    target_path = archiver::create_archive(&paths);
                } else {
                    target_path = paths.first().unwrap().clone();
                }
                self.encrypt_file(&target_path, &passphrase);
            },

            WorkerMode::Decryption => {
                self.decrypt_files(&paths, &passphrase);
            }
        }

        self.send("DONE".to_owned());
    }

    fn encrypt_file(&self, path: &PathBuf, passphrase: &String) {
        let formatted = format!("Encrypting file {:?}", path);
        println!("{:?}", formatted);
        gpg::encrypt(path, passphrase);
        self.send(formatted);
    }

    fn decrypt_files(&self, paths: &Vec<PathBuf>, passphrase: &String) {
        paths.into_iter().for_each(|path| {
            let formatted = format!("Decrypting file {:?}", path);
            println!("{:?}", formatted);
            gpg::decrypt(&path, &passphrase);
            self.send(formatted);
        });
    }

    fn send(&self, data: String) {
        self.sender.send(data).unwrap();
    }
}
