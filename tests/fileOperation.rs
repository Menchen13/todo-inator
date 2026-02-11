use chrono::NaiveDate;
use std::fs;
use std::io::{Seek, SeekFrom, Write};
use todo_inator::{TodoItem, load_file, save_file};

fn date(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).expect("Invalid date in tests!")
}

#[test]
fn test_read_from_file() {
    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    writeln!(tmpfile, "x (F) test").unwrap();
    writeln!(tmpfile, "  ").unwrap();
    writeln!(
        tmpfile,
        "2024-03-20 2024-01-01 dude this better work @unit +test"
    )
    .unwrap();
    writeln!(tmpfile, "2023-12-30 (F)").unwrap();

    let todos: Vec<TodoItem> = load_file(tmpfile.path()).unwrap();
    assert_eq!(todos.len(), 3);
    assert_eq!(todos[0].completed, true);
    assert_eq!(todos[0].priority, Some('F'));
    assert_eq!(todos[0].description, "test");
    assert_eq!(todos[1].completion_date, Some(date(2024, 03, 20)));
    assert_eq!(todos[1].creation_date, Some(date(2024, 01, 01)));
    assert_eq!(todos[1].description, "dude this better work @unit +test");
    assert_eq!(todos[1].projects, vec!["test"]);
    assert_eq!(todos[1].contexts, vec!("unit"));
    assert_eq!(todos[2].creation_date, Some(date(2023, 12, 30)));
    assert_eq!(todos[2].description, "(F)");
}

#[test]
fn test_write_to_file() {
    let item1: TodoItem = TodoItem {
        completed: (true),
        priority: (Some('G')),
        completion_date: (None),
        creation_date: (Some(date(2025, 02, 11))),
        contexts: (vec!["Noel".to_string()]),
        projects: (Vec::new()),
        description: ("unit test sein vadder @Noel".to_string()),
    };

    let item2: TodoItem = TodoItem {
        completed: (false),
        priority: (Some('A')),
        completion_date: (Some(date(2026, 10, 20))),
        creation_date: (Some(date(2025, 02, 11))),
        contexts: (vec!["Nik".to_string()]),
        projects: (vec!("Yoko".to_string())),
        description: ("unit test seine mudder @Nik +Yoko".to_string()),
    };

    let item3: TodoItem = TodoItem {
        completed: (false),
        priority: (None),
        completion_date: (None),
        creation_date: (None),
        contexts: (Vec::new()),
        projects: (Vec::new()),
        description: ("Final Line".to_string()),
    };

    let todos: Vec<TodoItem> = vec!(item1, item2, item3);

    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    
    save_file(&tmpfile, &todos).unwrap();

    let known_truth = "x (G) 2025-02-11 unit test sein vadder @Noel\n(A) 2026-10-20 2025-02-11 unit test seine mudder @Nik +Yoko\nFinal Line\n";

    let contents =  fs::read_to_string(tmpfile).unwrap();

    assert_eq!(contents, known_truth);

}
