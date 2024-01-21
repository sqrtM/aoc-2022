use std::collections::HashSet;
use std::fs;

struct Signal<'a>(&'a str);

struct Marker(Vec<char>);

pub fn solve() {
    find_packet(4);
    find_packet(14)
}

impl Marker {
    fn is_marker(chars: &[char], len: usize) -> bool {
        assert_eq!(chars.len(), len);
        chars.iter().collect::<HashSet<_>>().len() == chars.len()
    }
}

impl Signal<'_> {
    fn find_marker(&self, len: usize) -> Option<usize> {
        self.0
            .chars()
            .collect::<Vec<char>>()
            .windows(len)
            .position(|chars| Marker::is_marker(chars, len))
    }
}

fn find_packet(len: usize) {
    if let Ok(contents) = fs::read_to_string("src/day_six/input.txt") {
        if let Some(marker) = Signal(&contents).find_marker(len) {
            println!("{:?}", marker + len);
        } else {
            panic!("Could not find signal!")
        }
    }
}
