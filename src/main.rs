use colorz::*;
use std::{
  env, fs,
  fs::OpenOptions,
  io,
  io::{BufReader, BufWriter, Read, Write},
  process,
};

const HELP: &str = "LsTodo v0.1.3
Usage: lstodo [COMMAND] [ARGUMENTS]
Commands:
  [h]elp                      show this help message
  [l]ist                      list all tasks
  [a]dd [TASK]                add new task(s)
  [d]one [INDEX]              mark task(s) as done
  [u]ndo [INDEX]              mark task(s) as undone
  [r]emove [INDEX]            remove task(s)
  [s]ort                      sort completed and uncompleted tasks
  [n]ote [d/i/e/u/h] [INDEX]  highlight important task
  [c]hange [INDEX] [TASK]     change the content of a task
  [m]ove [INDEX] [INDEX]      switch the position of two tasks
  reset                       remove all tasks";
const OPEN_ERR: &str = "Unable to open the todo file!";
const INST_ERR: &str = "Unable to create new instance!";
const SAVE_ERR: &str = "Unable to save the todo file!";
const WRITE_ERR: &str = "Unable to write data!";
const STDOUT_ERR: &str = "Failed to write to stdout!";

pub struct LsTodo {
  lstodo: Vec<String>,
  lstodo_path: String,
  lstodo_count: usize,
  lstodo_indent: usize,
}

impl LsTodo {
  pub fn new() -> Result<Self, String> {
    let lstodo_path: String = env::var("LSTODO_PATH").unwrap_or_else(|_| {
      let home_dir = env::var("HOME").unwrap();

      format!("{}/.config/lstodo", &home_dir)
    });

    let file = OpenOptions::new()
      .write(true)
      .read(true)
      .create(true)
      .open(&lstodo_path)
      .expect(&OPEN_ERR);

    let mut buf_read = BufReader::new(&file);
    let mut content = String::new();

    buf_read.read_to_string(&mut content).unwrap();

    let lstodo = content.lines().map(str::to_string).collect();
    let lstodo_count = content.lines().count();
    let lstodo_indent = lstodo_count.to_string().len();

    Ok(Self {
      lstodo,
      lstodo_path,
      lstodo_count,
      lstodo_indent,
    })
  }

  pub fn list(&self) {
    let stdout = io::stdout();

    let mut writer = BufWriter::new(stdout);

    macro_rules! listfmt {
      ($pos: expr, $line: expr) => {
        format!(
          "{:>width$} {}\n",
          $pos.bold(),
          $line,
          width = &self.lstodo_indent
        )
      };
    }

    for (p, l) in self.lstodo.iter().enumerate() {
      if l.len() > 4 {
        let p = (p + 1).to_string();
        let s = &l[..4];
        let l = &l[4..];

        let data = match s {
          "[d] " => listfmt!(p, l.dimmed().strikethrough()),
          "[i] " => listfmt!(p, l.yellow()),
          "[e] " => listfmt!(p, l.red()),
          "[ ] " => listfmt!(p, l),
          _ => listfmt!(p, "Not a valid todo!".red()),
        };

        writer.write_all(data.as_bytes()).expect(&STDOUT_ERR)
      }
    }
  }

  pub fn add(&self, args: &[String]) {
    let file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(&self.lstodo_path)
      .expect(&OPEN_ERR);

    let mut buffer = BufWriter::new(file);

    for a in args {
      if a.trim().is_empty() {
        continue;
      }

      let line = format!("[ ] {a}\n");

      buffer.write_all(line.as_bytes()).expect(&WRITE_ERR)
    }
  }

  pub fn remove(&self, args: &[String]) {
    let file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(&self.lstodo_path)
      .expect(&OPEN_ERR);

    let mut buffer = BufWriter::new(file);

    for (p, l) in self.lstodo.iter().enumerate() {
      if args.contains(&(p + 1).to_string()) {
        continue;
      }

      let l = format!("{l}\n");

      buffer.write_all(l.as_bytes()).expect(&WRITE_ERR)
    }
  }

  pub fn done(&self, args: &[String]) {
    let file = OpenOptions::new().write(true).open(&self.lstodo_path).expect(&OPEN_ERR);

    let mut buffer = BufWriter::new(file);

    for (p, l) in self.lstodo.iter().enumerate() {
      if args.contains(&(p + 1).to_string()) {
        if &l[..4] == "[d] " {
          let l = format!("[ ] {}\n", &l[4..]);

          buffer.write_all(l.as_bytes()).expect(&WRITE_ERR)
        } else {
          let l = format!("[d] {}\n", &l[4..]);

          buffer.write_all(l.as_bytes()).expect(&WRITE_ERR)
        }
      } else if &l[..4] == "[ ] " || &l[..4] == "[d] " {
        let l = format!("{l}\n");

        buffer.write_all(l.as_bytes()).expect(&WRITE_ERR)
      }
    }
  }

  pub fn reset(&self) {
    match fs::remove_file(&self.lstodo_path) {
      Ok(_) => {}
      Err(e) => eprintln!("Error while clearing file: {e}"),
    };
  }

  pub fn undo(&self, args: &[String]) {
    let file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(&self.lstodo_path)
      .expect(&OPEN_ERR);

    let mut buffer = BufWriter::new(file);

    for (p, l) in self.lstodo.iter().enumerate() {
      if args.contains(&(p + 1).to_string()) {
        if &l[..4] != "[ ] " {
          let l = format!("[ ] {}\n", &l[4..]);

          buffer.write_all(l.as_bytes()).expect(&WRITE_ERR)
        }
      } else {
        let l = format!("{l}\n");

        buffer.write_all(l.as_bytes()).expect(&WRITE_ERR)
      }
    }
  }

  pub fn sort(&self) {
    let new_todo: String;

    let mut todo = String::new();
    let mut done = String::new();
    let mut impo = String::new();
    let mut emer = String::new();

    for l in self.lstodo.iter() {
      if &l[..4] == "[ ] " {
        let l = format!("{l}\n");

        todo.push_str(&l)
      } else if &l[..4] == "[d] " {
        let l = format!("{l}\n");

        done.push_str(&l)
      } else if &l[..4] == "[i] " {
        let l = format!("{l}\n");

        impo.push_str(&l)
      } else if &l[..4] == "[e] " {
        let l = format!("{l}\n");

        emer.push_str(&l)
      }
    }

    new_todo = format!("{}{}{}{}", &emer, &impo, &todo, &done);

    let mut file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(&self.lstodo_path)
      .expect(&OPEN_ERR);

    file.write_all(new_todo.as_bytes()).expect(&SAVE_ERR)
  }

  pub fn note(&self, args: &[String]) {
    if args[0] == "h" {
      note_help()
    }

    let file = OpenOptions::new().write(true).open(&self.lstodo_path).expect(&OPEN_ERR);

    let mut buffer = BufWriter::new(file);

    for (p, l) in self.lstodo.iter().enumerate() {
      if args.contains(&(p + 1).to_string()) {
        let l = match args[0].as_str() {
          "d" => format!("[d] {}\n", &l[4..]),
          "i" => format!("[i] {}\n", &l[4..]),
          "e" => format!("[e] {}\n", &l[4..]),
          "u" => format!("[ ] {}\n", &l[4..]),
          _ => {
            eprintln!("Invalid note! Use h to see help!");
            process::exit(1)
          }
        };

        buffer.write_all(l.as_bytes()).expect(&WRITE_ERR)
      } else {
        let l = format!("{l}\n");

        buffer.write_all(l.as_bytes()).expect(&WRITE_ERR)
      }
    }
  }

  pub fn change(&self, args: &[String]) {
    let file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(&self.lstodo_path)
      .expect(&OPEN_ERR);

    let mut buffer = BufWriter::new(file);

    for (p, l) in self.lstodo.iter().enumerate() {
      if &(p + 1).to_string() == &args[0] {
        let l = format!("{}{}\n", &l[..4], &args[1]);

        buffer.write_all(l.as_bytes()).expect(&WRITE_ERR);
      } else {
        let l = format!("{l}\n");

        buffer.write_all(l.as_bytes()).expect(&WRITE_ERR);
      }
    }
  }

  pub fn mover(&self, args: &[String]) {
    let file = OpenOptions::new().write(true).open(&self.lstodo_path).expect(&OPEN_ERR);
    let index: Vec<usize> = args.iter().map(|arg| arg.parse::<usize>().unwrap()).collect();

    let todo_st = self.lstodo[index[0] - 1].clone();
    let todo_nd = self.lstodo[index[1] - 1].clone();

    let mut buffer = BufWriter::new(file);

    for (p, l) in self.lstodo.iter().enumerate() {
      let l = match p + 1 {
        i if i == index[0] => format!("{todo_nd}\n"),
        i if i == index[1] => format!("{todo_st}\n"),
        _ => format!("{l}\n"),
      };

      buffer.write_all(l.as_bytes()).expect(&WRITE_ERR);
    }
  }

  // TODO: probably need a rewrite to be more concise
  pub fn args<'l>(&self, args: &'l [String], cnt: usize) -> Self {
    if args.len() != cnt {
      eprintln!("This command needs {cnt} arguments!");
      process::exit(1)
    }

    if args.iter().any(|i| i.parse::<usize>().unwrap() > self.lstodo_count) {
      eprintln!("There are only {} todos!", &self.lstodo_count.yellow());
      process::exit(1)
    }

    Self {
      lstodo: self.lstodo.clone(),
      lstodo_path: self.lstodo_path.clone(),
      lstodo_count: self.lstodo_count,
      lstodo_indent: self.lstodo_indent,
    }
  }
}

#[rustfmt::skip]
fn note_help() {
  println!(
"lstodo note [d/i/e/u/h] [INDEX] highlight important task
Notes:
  h print this help
  d mark the task as {}
  i mark the task as {}
  e mark the task as {}
  u mark the task as undone",
    "done".dimmed().strikethrough(),
    "important".yellow(),
    "emergency".red()
  )
}

fn help() {
  println!("{}", &HELP)
}

fn main() {
  let lstodo = LsTodo::new().expect(&INST_ERR);
  let args: Vec<String> = env::args().collect();

  if args.len() > 1 {
    let command = &args[1];

    match &command[..] {
      "reset" => lstodo.reset(),
      "add" | "a" => lstodo.args(&args[2..], 1).add(&args[2..]),
      "note" | "n" => lstodo.args(&args[2..], 2).note(&args[2..]),
      "done" | "d" => lstodo.args(&args[2..], 1).done(&args[2..]),
      "undo" | "u" => lstodo.args(&args[2..], 1).undo(&args[2..]),
      "move" | "m" => lstodo.args(&args[2..], 2).mover(&args[2..]),
      "remove" | "r" => lstodo.args(&args[2..], 1).remove(&args[2..]),
      "change" | "c" => lstodo.args(&args[2..], 2).change(&args[2..]),
      "list" | "l" => lstodo.list(),
      "sort" | "s" => lstodo.sort(),
      "help" | "h" | "-h" | _ => help(),
    }
  } else {
    lstodo.list()
  }
}
