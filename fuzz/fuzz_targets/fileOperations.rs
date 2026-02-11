#![no_main]

use libfuzzer_sys::fuzz_target;
use std::io::Cursor;

// Assuming you made load_from_reader_fuzz and save_to_writer_fuzz available
use todo_inator::{load_from_reader_fuzz, save_to_writer_fuzz}; 

fuzz_target!(|data: &str| {
    // 1. Attempt to load the random bytes
    let cursor = Cursor::new(data);
    
    // If it's valid UTF-8 and parses correctly...
    if let Ok(original_list) = load_from_reader_fuzz(cursor) {
        
        // 2. Save it back to a new byte buffer
        let mut out_buffer = Vec::new();
        // Assume save_to_writer_fuzz takes a Vec<TodoItem> and a Write trait object
        save_to_writer_fuzz(&mut out_buffer, &original_list).unwrap(); 
        
        // 3. Load it a second time
        let cursor2 = Cursor::new(&out_buffer);
        let reparsed_list = load_from_reader_fuzz(cursor2).expect("Must reparse valid output");
        
        // 4. The structural data must remain identical!
        assert_eq!(original_list, reparsed_list);
    }
});
