use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

pub struct App {
    path: PathBuf,
}

impl App {
    pub fn new() -> Self {
        let temp_path = if cfg!(windows) {
            env::var("TEMP").unwrap()
        } else {
            env::var("TMPDIR").unwrap()
        };

        let mut path = PathBuf::from(temp_path);
        path.push(".hitlist");
        Self { path }
    }

    pub fn mark(&self) {
        let filepath =
            self.path.to_str().expect("Error: filepath doesn't exists!");

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filepath)
            .expect("Unable to open file");

        let data = std::env::current_dir()
            .expect("Error: Failed to get path of the current directory!");
        let path = format!("{}\n", data.display());
        file.write(path.as_bytes()).unwrap();
    }

    pub fn unmark(&self, index: usize) {
        let filepath =
            self.path.to_str().expect("Error: filepath doesn't exists!");

        let mut lines = Vec::new();
        {
            let file = match File::open(filepath) {
                Ok(file) => file,
                Err(_) => {
                    eprintln!("List is empty!");
                    std::process::exit(1);
                }
            };

            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader
                .read_to_string(&mut contents)
                .expect("Error: Failed to read the contents of the file!");

            for line in contents.lines() {
                let data = format!("{}\n", line);
                lines.push(data);
            }
        }

        if index > lines.len() || index == 0 {
            eprintln!("Not valid index!");
            std::process::exit(1);
        }

        lines.reverse();

        let file = File::create(filepath)
            .expect("Error: Failed to open the file in create mode!");

        let mut writer = BufWriter::new(file);

        let mut counter = 0;
        while let Some(line) = lines.pop() {
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
        let filepath =
            self.path.to_str().expect("Error: filepath doesn't exists!");

        let file = match File::open(filepath) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("The hitlist is empty!");
                std::process::exit(1);
            }
        };

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader
            .read_to_string(&mut contents)
            .expect("Error: Failed to read the contents of the file!");

        if contents.is_empty() {
            eprintln!("The hitlist is empty!");
            std::process::exit(1);
        }

        let mut paths = Vec::new();
        for line in contents.lines() {
            let path = Path::new(line);
            paths.push(path);
        }

        let handle = io::stdout().lock();
        let mut writer = BufWriter::new(handle);

        paths.reverse();

        while let Some(path) = paths.pop() {
            let data = if path.exists() {
                format!("[✓] {}\n", path.display())
            } else {
                format!("[✘] {}\n", path.display())
            };
            writer
                .write(data.as_bytes())
                .expect("Error: Failed to write the contents of the file!");
        }
    }

    pub fn clear(&self) {
        let filepath =
            self.path.to_str().expect("Error: Filepath doesn't exists!");

        File::create(filepath).expect("Error: Failed to clear the file!");
    }

    pub fn list(&self) {
        let filepath =
            self.path.to_str().expect("Error: Filepath doesn't exists!");

        let file = match File::open(filepath) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Nothing to show!");
                std::process::exit(1);
            }
        };

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader
            .read_to_string(&mut contents)
            .expect("Error: Failed to read the contents of the file!");

        if contents.is_empty() {
            eprintln!("Nothing to show!");
            std::process::exit(1);
        }

        let handle = io::stdout().lock();
        let mut writer = BufWriter::new(handle);

        let mut index: usize = 1;

        for line in contents.lines() {
            let data = format!("[{}] {}\n", index, line);
            writer
                .write(data.as_bytes())
                .expect("Error: Failed to print contents!");
            index += 1;
        }
    }
}
