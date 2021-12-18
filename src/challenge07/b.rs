pub fn run(stdin: String) -> String {
    let crabs: Vec<i32> = stdin
        .split(",")
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();

    let min = *crabs.iter().min().unwrap_or(&0);
    let max = *crabs.iter().max().unwrap_or(&0);

    let distance = (min..(max + 1))
        .map(|pos| {
            let mut distance = 0;
            for crab in crabs.iter() {
                distance += fuel((crab - pos).abs());
            }
            distance
        })
        .min()
        .unwrap_or(0);

    distance.to_string()
}

fn fuel(distance: i32) -> i32 {
    distance * (distance + 1) / 2
}

#[cfg(test)]
mod tests {
    use crate::util::testing_utils::read_test_resource;

    #[test]
    fn run() {
        assert_eq!("95476244", super::run(read_test_resource("day07.txt")));
    }
}
