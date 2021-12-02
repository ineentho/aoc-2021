use crate::core_challenge::Challenge;

pub mod a;
pub mod b;

pub struct Challenge02 {}

impl Challenge for Challenge02 {
    fn run_part_a(&self, stdin: String) -> String {
        a::run(stdin)
    }

    fn run_part_b(&self, stdin: String) -> String {
        b::run(stdin)
    }
}
