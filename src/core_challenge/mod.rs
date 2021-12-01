pub trait Challenge {
    fn run_part_a(&self, stdin: String) -> String;
    fn run_part_b(&self, stdin: String) -> String;
}
