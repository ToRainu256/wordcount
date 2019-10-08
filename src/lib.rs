//! wordcount provide countter for simple charactor word line
//! if you want more info pls look at ['count'](fn.count.html)'s document
#![warn(missing_docs)]
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// option for ['count'](fn.count.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    Char,
    Word,
    Line,
}

/// default option is ['Word'](enum.CountOption.html#variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// read each line as  UTF-8 string form input then cout frequency
/// count Target controled by Option
/// * ['CountOption::Char'](enum.CountOption.html#variant.Char): Unicode Charactor
/// * ['CountOption::Word'](enum.CountOption.html#variant.Word): Word which is matching by \w+
/// * ['CountOption::Line'](enum.CountOption.html#variant.Line): line which is ending by \n \r\n
///
/// # Examples
/// how to count word in input
/// ```
/// use std::io::Cursor;
/// use wordcount::{count, CountOption};
///
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
///
/// assert_eq!(freq["aa"], 1);
/// assert_eq!(freq["bb"], 2);
/// assert_eq!(freq["cc"], 1);
/// ```
/// # Panics
///
/// if input does'nt format by UTF-8 then panic

pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CountOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }
    freqs
}

#[test]
fn word_count_works() {
    use std::io::Cursor;
    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("bb".to_string(), 2);
    exp.insert("cc".to_string(), 1);

    assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
}

#[test]
fn word_count_works2() {
    use std::io::Cursor;
    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("cc".to_string(), 1);
    exp.insert("dd".to_string(), 1);

    assert_eq!(count(Cursor::new("aa cc dd"), CountOption::Word), exp);
}

use std::io;

//#[test]
//fn result_test() -> io::Result<()> {
//    use std::fs::{read_to_string, remove_file, write};
//    let message = read_to_string("test.txt")?;
//    remove_file("test.txt")?;
//    assert_eq!(message, "message");
//    Ok(())
//}

#[test]
#[should_panic]
fn word_count_do_not_contain_unknown_words() {
    use std::io::Cursor;

    count(
        Cursor::new([
            b'0', // a
            0xf0, 0x90, 0x80, // unknown char
            0xe3, 0x81, 0x82, // あ
        ]),
        CountOption::Word,
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    macro_rules! assert_map {
        ($expr: expr, {$($key: expr => $value:expr),*}) =>{
            $(assert_eq!($expr[$key], $value));*
        };
    }

    #[test]
    fn word_count_works3() {
        let freqs = count(Cursor::new("aa cc dd"), CountOption::Word);

        assert_eq!(freqs.len(), 3);
        assert_map!(freqs, {"aa" => 1, "cc" => 1, "dd" => 1});
    }
} /* test */
