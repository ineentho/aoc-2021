pub fn run(stdin: String) -> String {
    let lines: Vec<Vec<char>> = stdin
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut gamma = 0;
    let mut epsilon = 0;

    let bits_per_line = lines[0].len();

    for i in 0usize..bits_per_line {
        let ones = lines
            .iter()
            .map(|line| line[i])
            .map(|char| if char == '1' { 1 } else { 0 })
            .fold(0, |acc, num| acc + num);

        let exp: u32 = (bits_per_line - i - 1).try_into().unwrap();

        if ones > lines.len() / 2 {
            gamma += 2i32.pow(exp);
        } else {
            epsilon += 2i32.pow(exp);
        }
    }

    (gamma * epsilon).to_string()
}

#[cfg(test)]
mod tests {
    use crate::util::testing_utils::read_test_resource;

    #[test]
    fn run() {
        assert_eq!("3549854", super::run(read_test_resource("day03.txt")));
    }
}
