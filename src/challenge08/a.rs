#[derive(Debug)]
struct Entry {
    output: Vec<String>,
}

impl From<&str> for Entry {
    fn from(str: &str) -> Self {
        let parts = str.split("|");
        let output = parts.skip(1).next().unwrap_or("").trim();
        Entry {
            output: output.split(" ").map(|s| s.to_owned()).collect(),
        }
    }
}

fn parse_number(num: &str) -> Option<i8> {
    match num.len() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}

pub fn run(stdin: String) -> String {
    let entries: Vec<Entry> = stdin.split("\n").map(Entry::from).collect();

    let sum = entries
        .iter()
        .map(|entry| {
            entry
                .output
                .iter()
                .filter(|s| match parse_number(s) {
                    Some(_) => true,
                    None => false,
                })
                .count()
        })
        .fold(0, |acc, v| acc + v);

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::util::testing_utils::read_test_resource;

    #[test]
    fn run() {
        assert_eq!("514", super::run(read_test_resource("day08.txt")));
    }
}
