struct State {
    depth: i64,
    horizontal: i64,
}

enum Instruction {
    Down(i64),
    Forward(i64),
    Up(i64),
}

pub fn run(stdin: String) -> String {
    let initial_state = State {
        depth: 0,
        horizontal: 0,
    };

    let end_position = stdin
        .split("\n")
        .filter_map(parse_instruction)
        .fold(initial_state, move_submarine);

    (end_position.depth * end_position.horizontal).to_string()
}

fn move_submarine(state: State, instruction: Instruction) -> State {
    match instruction {
        Instruction::Down(amount) => State {
            depth: state.depth + amount,
            ..state
        },
        Instruction::Up(amount) => State {
            depth: state.depth - amount,
            ..state
        },
        Instruction::Forward(amount) => State {
            horizontal: state.horizontal + amount,
            ..state
        },
    }
}

fn parse_instruction(str: &str) -> Option<Instruction> {
    let mut parts = str.trim().split(" ");
    let direction = parts.next();
    let amount = parts
        .next()
        .map_or(0, |amount| amount.parse::<i64>().unwrap_or(0));

    match direction {
        Some("down") => Some(Instruction::Down(amount)),
        Some("up") => Some(Instruction::Up(amount)),
        Some("forward") => Some(Instruction::Forward(amount)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::util::testing_utils::read_test_resource;

    #[test]
    fn run() {
        assert_eq!("1840243", super::run(read_test_resource("day02.txt")));
    }
}
