use std::io::Cursor;
use wordcount::{count, CountOption};

#[macro_use]
mod utils;

#[test]
fn char_count_test() {
    let input = Cursor::new(b"abadracadabra");

    let freq = count(input, CountOption::Char);
    assert_map!(freq,
                {
                    "a" => 6,
                    "b" => 2,
                    "c" => 1,
                    "d" => 2,
                    "r" => 2
                }
    );
}

#[test]
fn char_count_utf8_test() {
    let input = Cursor::new(
        r#"
天地玄黄
宇宙洪荒
日月盈昃
辰宿列張
"#,
    );

    let freq = count(input, CountOption::Char);
    println!("{:?}", freq);
    assert_eq!(freq.len(), 16);
    for (_, count) in freq {
        assert_eq!(count, 1);
    }
}
