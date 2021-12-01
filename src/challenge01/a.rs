pub fn run(stdin: String) -> String {
    let measurements: Vec<i32> = stdin
        .split("\n")
        .map(|str| str.trim())
        .filter(|str| str.len() > 0)
        .map(|str| str.parse::<i32>().unwrap_or(0))
        .collect();

    let mut increases = 0;

    for i in 1..measurements.len() {
        let previous = measurements[i - 1];
        let current = measurements[i];

        if current > previous {
            increases += 1;
        }
    }

    increases.to_string()
}

#[cfg(test)]
mod tests {
    use crate::util::testing_utils::read_test_resource;

    #[test]
    fn run() {
        assert_eq!("1121", super::run(read_test_resource("day01.txt")));
    }
}
