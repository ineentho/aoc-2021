use crate::core_challenge::Challenge;

pub mod a;
pub mod b;

pub struct Challenge07 {}

impl Challenge for Challenge07 {
    fn run_part_a(&self, stdin: String) -> String {
        a::run(stdin)
    }

    fn run_part_b(&self, stdin: String) -> String {
        b::run(stdin)
    }
}
