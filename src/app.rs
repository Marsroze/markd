use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

pub struct App {
    dirs: VecDeque<String>,
    path: String,
    backed_up: bool,
    backup_path: String,
}

impl App {
    pub fn new() -> Self {
        let key = if cfg!(windows) { "TEMP" } else { "TEMPDIR" };
        let temp_path = match env::var(key) {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Error: Failed to access the tempdir!");
                std::process::exit(1);
            }
        };

        let mut path = PathBuf::from(&temp_path);
        path.push(".hitlist");
        let path = format!("{}", path.display());

        let mut backup_path = PathBuf::from(temp_path);
        backup_path.push(".hitlist.bak");

        let mut backed_up = false;
        if backup_path.exists() {
            backed_up = true;
        }

        let backup_path = format!("{}", backup_path.display());

        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&path)
            .expect("Error: Failed to open the file!");

        let mut reader = BufReader::new(file);

        let mut contents = String::new();

        reader
            .read_to_string(&mut contents)
            .expect("Error: Failed to read to string!");

        let dirs = contents.lines().map(|line| format!("{}\n", line)).collect();

        Self {
            dirs,
            path,
            backed_up,
            backup_path,
        }
    }

    pub fn mark(&self) {
        let current = std::env::current_dir()
            .expect("Error: Failed to get path of the current directory!");

        let data = format!("{}\n", current.display());

        if self.dirs.contains(&data) {
            std::process::exit(1);
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .expect("Error: Failed to open the file in append mode!");

        file.write(data.as_bytes())
            .expect("Error: Failed to write to the file!");
    }

    pub fn unmark(&self, index: usize) {
        let mut lines = self.dirs.clone();
        if index > lines.len() || index == 0 {
            eprintln!("Not valid index!");
            std::process::exit(1);
        }

        let file = File::create(&self.path)
            .expect("Error: Failed to open the file in create mode!");

        let mut writer = BufWriter::new(file);

        let mut counter = 0;
        while let Some(line) = lines.pop_front() {
            counter += 1;
            if index == counter {
                continue;
            }
            writer
                .write(line.as_bytes())
                .expect("Error: Failed to write to file!");
        }
    }

    pub fn check(&self) {
        if self.dirs.len() == 0 {
            eprintln!("Nothing to check!");
            std::process::exit(1);
        }

        let mut paths: VecDeque<&Path> = self
            .dirs
            .iter()
            .map(|line| Path::new(line.trim()))
            .collect();

        let handle = io::stdout().lock();
        let mut writer = BufWriter::new(handle);

        while let Some(path) = paths.pop_front() {
            let data = if path.exists() {
                format!("[✓] {}\n", path.display())
            } else {
                format!("[✘] {}\n", path.display())
            };
            writer
                .write_all(data.as_bytes())
                .expect("Error: Failed to write the contents of the file!");
        }
    }

    pub fn clear(&self) {
        if self.dirs.len() != 0 {
            fs::copy(&self.path, &self.backup_path)
                .expect("Error: Failed to backup the data!");
            fs::remove_file(&self.path)
                .expect("Error: Failed to clear the file!");
        }
    }

    pub fn list(&self) {
        if self.dirs.len() == 0 {
            eprintln!("Nothing to show!");
            std::process::exit(1);
        }

        let handle = io::stdout().lock();
        let mut writer = BufWriter::new(handle);

        for (index, line) in self.dirs.iter().enumerate() {
            let data = format!("[{}] {}", index + 1, line);
            writer
                .write(data.as_bytes())
                .expect("Error: Failed to print contents!");
        }
    }

    pub fn restore(&self) {
        if self.backed_up {
            fs::copy(&self.backup_path, &self.path)
                .expect("Error: Failed to load the backup data!");
        } else {
            eprintln!("No backup found!");
        }
    }
}
