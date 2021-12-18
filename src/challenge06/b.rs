pub fn run(stdin: String) -> String {
    let mut fishes: Vec<i64> = vec![0; 9];

    stdin
        .split(",")
        .filter_map(|n| n.parse::<usize>().ok())
        .for_each(|fish| {
            fishes[fish] += 1;
        });

    for _ in 0..256 {
        let producing_fishes = fishes[0];

        for i in 1..fishes.len() {
            fishes[i - 1] = fishes[i];
        }

        fishes[8] = producing_fishes;
        fishes[6] += producing_fishes;
    }

    fishes.iter().fold(0, |acc, n| acc + n).to_string()
}

#[cfg(test)]
mod tests {
    use crate::util::testing_utils::read_test_resource;

    #[test]
    fn run() {
        assert_eq!("1686252324092", super::run(read_test_resource("day06.txt")));
    }
}
