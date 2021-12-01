fn sliding_measurement(vec: &Vec<i32>, index: usize, number: usize) -> i32 {
    let range = index..(index + number);
    range.fold(0, |acc, index| acc + vec[index])
}

pub fn run(stdin: String) -> String {
    let measurements: Vec<i32> = stdin
        .split("\n")
        .map(|str| str.trim())
        .filter(|str| str.len() > 0)
        .map(|str| str.parse::<i32>().unwrap_or(0))
        .collect();

    let mut increases = 0;

    let num_measurements = 3;

    for i in 0..(measurements.len() - num_measurements) {
        let previous = sliding_measurement(&measurements, i, num_measurements);
        let current = sliding_measurement(&measurements, i + 1, num_measurements);

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
        assert_eq!("1065", super::run(read_test_resource("day01.txt")));
    }
}
