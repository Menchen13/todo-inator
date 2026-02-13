use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::str::FromStr;
use std::{fmt, fs};
use std::collections::HashSet;

use chrono::NaiveDate;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct TodoItem {
    pub completed: bool,
    pub priority: Option<char>,
    pub completion_date: Option<NaiveDate>,
    pub creation_date: Option<NaiveDate>,
    pub contexts: HashSet<String>,
    pub projects: HashSet<String>,
    pub description: String,
}

#[derive(Default)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
}

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Invalid priority format: expected (A-Z), found {0}")]
    InvalidPriority(String),

    #[error("Invalid date format: {0}")]
    InvalidDate(#[from] chrono::ParseError), // Automatically converts chrono errors!

    #[error("Line is empty")]
    EmptyLine,

    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Unknown parsing error")]
    Unknown,
}

impl FromStr for TodoItem {
    type Err = TodoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace().peekable();

        let mut completed = false;
        if let Some(token) = tokens.peek()
            && *token == "x"
        {
            completed = true;
            tokens.next();
        }

        let mut priority = None;
        if let Some(token) = tokens.peek()
            && token.starts_with('(')
            && token.ends_with(')')
            && token.len() == 3
        {
            let char_code = token.chars().nth(1).unwrap();
            if char_code.is_alphabetic() && char_code.is_uppercase() {
                priority = Some(char_code);
                tokens.next();
            }
        }

        let mut date1 = None;
        let mut date2 = None;

        if let Some(token) = tokens.peek()
            && let Ok(d) = NaiveDate::parse_from_str(token, "%Y-%m-%d")
        {
            date1 = Some(d);
            tokens.next();
        }

        if let Some(token) = tokens.peek()
            && let Ok(d) = NaiveDate::parse_from_str(token, "%Y-%m-%d")
        {
            date2 = Some(d);
            tokens.next();
        }

        // assign the dates with this sick match statement, Cant do that in C
        let (completion_date, creation_date) = match (date1, date2) {
            (Some(d1), Some(d2)) => (Some(d1), Some(d2)),
            (Some(d1), None) => (None, Some(d1)),
            _ => (None, None),
        };

        let remaining: Vec<&str> = tokens.collect();
        let description = remaining.join(" ");

        let mut projects = HashSet::new();
        let mut contexts = HashSet::new();

        for word in &remaining {
            if word.starts_with('+') && word.len() > 1 {
                projects.insert(word[1..].to_string()); //something here
            } else if word.starts_with('@') && word.len() > 1 {
                contexts.insert(word[1..].to_string());
            }
        }

        Ok(TodoItem {
            completed,
            priority,
            completion_date,
            creation_date,
            projects,
            contexts,
            description,
        })
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.completed {
            write![f, "x "]?;
        }

        if let Some(prio) = self.priority {
            write![f, "({}) ", prio]?;
        }

        match (self.creation_date, self.completion_date) {
            (Some(creat), Some(comp)) => {
                write!(f, "{} {} ", comp, creat)?;
            }
            (Some(creat), None) => {
                write!(f, "{} ", creat)?;
            }
            (None, None) => {}
            _ => {
                return Err(std::fmt::Error);
            }
        }

        write!(f, "{}", self.description.trim())?;

        Ok(())
    }
}

impl TodoList {
    pub fn add_item(&mut self, input: &str) -> Result<(), TodoError> {
        let item = input.parse::<TodoItem>()?;
        self.items.push(item);
        Ok(())
    }
    pub fn sort_by_priority(&mut self) {
        // Rust's sort_by is perfect here
        // We put None (no priority) at the bottom
        self.items.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    // The template looking thing is definies a generic type P with the condition that the type P can
    // be turned to a &Path cheaply with the .as_ref() API. It basically checks if the type has the
    // Trait AsRef<Path> implemented.
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, TodoError> {
        // File::open is kinda like FILE* in C and the BufReader around it makes reading more efficient
        // (should almost always be used from what i can tell)
        let reader = BufReader::new(File::open(path)?);
        let items = Self::load_from_reader(reader)?;
        Ok(Self { items })
    }

    fn load_from_reader<R: BufRead>(reader: R) -> Result<Vec<TodoItem>, TodoError> {
        let mut tasks = Vec::<TodoItem>::new();

        for line_result in reader.lines() {
            // this is to handle IO errors like file corruption or deleting during read
            let line = line_result?;

            if line.trim().is_empty() {
                continue;
            }

            // uses the FromStr Trait defined for TodoItem with the parse API.
            let task = line.parse::<TodoItem>()?;

            tasks.push(task);
        }

        Ok(tasks)
    }

    #[cfg(fuzzing)]
    pub fn load_from_reader_fuzz<R: BufRead>(reader: R) -> Result<Vec<TodoItem>, TodoError> {
        Self::load_from_reader(reader)
    }

    pub fn save_file<P: AsRef<Path>>(path: P, todos: &TodoList) -> io::Result<()> {
        // use a tmp file to achive atomic write
        let tmp_path = path.as_ref().with_extension("tmp");
        let writer = BufWriter::new(File::create(&tmp_path)?);
        Self::save_to_writer(writer, todos)?;

        fs::rename(tmp_path, path)?;
        Ok(())
    }

    // &[TodoItem] is a Slice of TodoItem. This can be a Vector or Array or smt. Keeps the function
    // more generic
    fn save_to_writer<W: Write>(mut writer: W, todos: &TodoList) -> io::Result<()> {
        for item in todos.items.iter() {
            writeln!(writer, "{}", item)?;
        }
        Ok(())
    }

    #[cfg(fuzzing)]
    pub fn save_to_writer_fuzz<W: Write>(writer: W, todos: &[TodoItem]) -> io::Result<()> {
        Self::save_to_writer(writer, todos)
    }
}
