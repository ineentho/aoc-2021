enum BitResult {
    ZERO,
    ONE,
    EQUAL,
}

pub fn run(stdin: String) -> String {
    let lines: Vec<Vec<char>> = stdin
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let oxygen_generator_rating = {
        let line = filter_lines(&lines, |res| match res {
            BitResult::ZERO => '0',
            BitResult::EQUAL => '1',
            BitResult::ONE => '1',
        });

        char_vec_to_number(line)
    };

    let co2_scrubber_rating = {
        let line = filter_lines(&lines, |res| match res {
            BitResult::ONE => '0',
            BitResult::EQUAL => '0',
            BitResult::ZERO => '1',
        });

        char_vec_to_number(line)
    };

    (oxygen_generator_rating * co2_scrubber_rating).to_string()
}

fn char_vec_to_number(char_array: Vec<char>) -> u32 {
    let mut res: u32 = 0;

    let len = char_array.len();

    for i in 0..len {
        if char_array[i] == '1' {
            let exp: u32 = (len - i - 1).try_into().unwrap();
            res += 2u32.pow(exp);
        }
    }

    res
}

fn filter_lines<F: Fn(&BitResult) -> char>(
    original_lines: &Vec<Vec<char>>,
    char_to_keep: F,
) -> Vec<char> {
    let mut lines: Vec<Vec<char>> = original_lines.clone();
    let bits_per_line = lines[0].len();

    for i in 0..bits_per_line {
        let comparison_res = most_common_bit_at_column(&lines, i);

        lines = lines
            .iter()
            .filter(|line| char_to_keep(&comparison_res) == line[i])
            .map(|line| line.clone())
            .collect();

        if lines.len() == 1 {
            break;
        }
    }

    lines[0].clone()
}

fn most_common_bit_at_column(lines: &Vec<Vec<char>>, column: usize) -> BitResult {
    let ones = lines
        .iter()
        .map(|line| line[column])
        .map(|char| if char == '1' { 1 } else { 0 })
        .fold(0, |acc, num| acc + num);

    if ones * 2 > lines.len() {
        BitResult::ONE
    } else if ones * 2 < lines.len() {
        BitResult::ZERO
    } else {
        BitResult::EQUAL
    }
}

#[cfg(test)]
mod tests {
    use crate::util::testing_utils::read_test_resource;

    #[test]
    fn run() {
        assert_eq!("3765399", super::run(read_test_resource("day03.txt")));
    }
}
