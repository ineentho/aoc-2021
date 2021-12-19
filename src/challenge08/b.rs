use std::collections::HashSet;

struct KnownEntryNumbers {
    four: HashSet<char>,
    one: HashSet<char>,
}

#[derive(Debug)]
struct Entry {
    output: Vec<i32>,
}

impl From<&str> for Entry {
    fn from(str: &str) -> Self {
        let mut parts = str.split("|");
        let input: Vec<&str> = parts.next().unwrap_or("").trim().split(" ").collect();
        let output: Vec<&str> = parts.next().unwrap_or("").trim().split(" ").collect();

        let mut all = Vec::new();
        all.append(&mut input.clone());
        all.append(&mut output.clone());

        let known = find_known_numbers(&all);

        Entry {
            output: output
                .iter()
                .filter_map(|s| parse_number(&known, s))
                .collect(),
        }
    }
}

impl From<Entry> for i32 {
    fn from(entry: Entry) -> Self {
        let len = entry.output.len();

        let mut num: i32 = 0;
        for i in 0..len {
            num += entry.output[i] * 10i32.pow((len - i - 1).try_into().unwrap());
        }

        num
    }
}

fn find_known_numbers(strs: &Vec<&str>) -> KnownEntryNumbers {
    let mut one: HashSet<char> = HashSet::new();
    let mut four: HashSet<char> = HashSet::new();

    for str in strs {
        match str.len() {
            2 => {
                if one.len() == 0 {
                    str.chars().for_each(|c| {
                        one.insert(c);
                    })
                }
            }
            4 => {
                if four.len() == 0 {
                    str.chars().for_each(|c| {
                        four.insert(c);
                    })
                }
            }
            _ => {}
        }
    }

    KnownEntryNumbers { four, one }
}

fn parse_number(known: &KnownEntryNumbers, str: &str) -> Option<i32> {
    let count_in =
        |set: &HashSet<char>| -> usize { str.chars().filter(|c| set.contains(c)).count() };

    match str.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        5 => match count_in(&known.four) {
            2 => Some(2),
            3 => match count_in(&known.one) {
                2 => Some(3),
                1 => Some(5),
                _ => None,
            },
            _ => None,
        },
        6 => match count_in(&known.one) {
            1 => Some(6),
            2 => match count_in(&known.four) {
                3 => Some(0),
                4 => Some(9),
                _ => None,
            },
            _ => None,
        },
        7 => Some(8),
        _ => None,
    }
}

pub fn run(stdin: String) -> String {
    let sum = stdin
        .split("\n")
        .map(Entry::from)
        .map(i32::from)
        .fold(0, |a, b| a + b);

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::util::testing_utils::read_test_resource;

    #[test]
    fn run() {
        assert_eq!("1012272", super::run(read_test_resource("day08.txt")));
    }
}
