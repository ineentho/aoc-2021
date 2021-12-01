pub fn run(stdin: String) -> String {
    stdin
}

#[cfg(test)]
mod tests {
    use crate::util::testing_utils::read_test_resource;

    #[test]
    fn run() {
        assert_eq!("", super::run(read_test_resource("template.txt")));
    }
}
