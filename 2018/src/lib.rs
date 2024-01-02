use std::io::{BufRead, BufReader, Lines};
use std::fs::File;
use std::time::Instant;

#[inline]
pub fn time<F, Out>(label: &str, f: F) -> Out
where
    F: FnOnce() -> Out,
{
    let before = Instant::now();
    let res = f();
    println!("{}: {:?}", label, before.elapsed());
    res
}

pub struct DataReader {
    lines: Lines<BufReader<File>>,
}

impl DataReader {
    pub fn open(day: usize) -> Self {
        let f = match File::open(format!("input/{}.txt", day)) {
            Ok(f) => f,
            Err(_) => panic!("Input for day {} is missing.", day),
        };
        let f = BufReader::new(f);
        DataReader {
            lines: f.lines(),
        }
    }
    pub fn text_io(self) -> TextIOLines {
        TextIOLines {
            lines: self.lines,
        }
    }
}

impl Iterator for DataReader {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.lines.next().map(|res| res.expect("IO error while reading string."))
    }
}

pub type TextIOLine = std::vec::IntoIter<u8>;
pub struct TextIOLines {
    lines: Lines<BufReader<File>>,
}
impl Iterator for TextIOLines {
    type Item = TextIOLine;
    fn next(&mut self) -> Option<TextIOLine> {
        self.lines.next().map(|res| res.expect("IO error while reading string."))
            .map(|string| string.into_bytes().into_iter())
    }
}
