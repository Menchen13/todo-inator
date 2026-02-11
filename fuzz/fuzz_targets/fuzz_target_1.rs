#![no_main]

use libfuzzer_sys::fuzz_target;
use std::str::FromStr;

use todo_inator::TodoItem;

fuzz_target!(|data: &str| {
    // fuzzed code goes here
    // attempt at round trip fuzzing harnes
    // random string -Parse-> Struct -Display-> string compare strings


    if let Ok(item) = TodoItem::from_str(data) {
        let formatted = item.to_string();

        let item_round_trip =
            TodoItem::from_str(&formatted).expect("Should be able to parse generated output");

        assert_eq!(item, item_round_trip, "Round trip failed!\nRandom Data: '{}'\nOriginal: {:?}\nFormatted: '{}'\nReparsed: {:?}", data,item, formatted, item_round_trip);
    }
});
