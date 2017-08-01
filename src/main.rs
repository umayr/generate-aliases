use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

struct Alias {
    name: String,
    cmd: String,
}

impl Alias {
    fn new(n: String, c: String) -> Alias {
        Alias { name: n, cmd: c }
    }

    fn parse(s: String) -> Vec<Alias> {
        let mut list: Vec<Alias> = Vec::new();
        for line in s.split("\n") {
            // see if this line starts with alias
            if line.starts_with("alias") {
                let v = line.split('=').collect::<Vec<_>>();
                list.push(Alias::new(v[0][5..].to_string().trim().into(), v[1].trim().into()));
            }
        }
        return list;
    }

    fn function(&self) -> String {
        format!("function {}\n\t{}\nend", self.name, self.cmd).into()
    }

    fn write(&self) -> Result<(), ApplicationError> {
        let home_dir = match env::home_dir() {
            Some(path) => path,
            None => return Err(ApplicationError::NoHomeDirectory),
        };
        let raw = format!("{}/.config/fish/functions/{}.fish",
                          home_dir.to_str().unwrap(),
                          self.name.to_string());
        let path = Path::new(&raw);

        let mut file = match File::create(&path) {
            Err(_) => return Err(ApplicationError::CreatingFile),
            Ok(file) => file,
        };

        match file.write_all(self.function().as_bytes()) {
            Err(_) => return Err(ApplicationError::WritingFile),
            Ok(_) => true,
        };

        Ok(())
    }
}
enum ApplicationError {
    NoHomeDirectory,
    CreatingFile,
    OpeningFile,
    ReadingFile,
    WritingFile,
}

fn run() -> Result<(), ApplicationError> {
    let home_dir = match env::home_dir() {
        Some(path) => path,
        _ => return Err(ApplicationError::NoHomeDirectory),
    };

    let args = env::args().collect::<Vec<_>>();
    let raw_path = if args.len() > 2 {
        args[2].to_string()
    } else {
        format!("{}/.alias", home_dir.to_str().unwrap())
    };

    let file_path = Path::new(&raw_path);
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => return Err(ApplicationError::OpeningFile),
    };

    let mut contents = String::new();
    let aliases = match file.read_to_string(&mut contents) {
        Ok(_) => Alias::parse(contents),
        Err(_) => return Err(ApplicationError::ReadingFile),
    };
    for a in aliases {
        match a.write() {
            Err(e) => return Err(e),
            Ok(_) => continue,
        }
    }

    Ok(())
}
fn main() {
    exit(match run() {
             Ok(_) => {
        println!("created aliases");
        0
    }
             Err(e) => {
        match e {
            ApplicationError::NoHomeDirectory => println!("couldn't find home directory"),
            ApplicationError::OpeningFile => println!("couldn't open file"),
            ApplicationError::CreatingFile => println!("couldn't create file"),
            ApplicationError::WritingFile => println!("couldn't write file"),
            ApplicationError::ReadingFile => println!("couldn't read file"),
        }
        1
    }
         });
}
