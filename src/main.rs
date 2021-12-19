#[macro_use]
extern crate clap;

use clap::{App, Arg};
use core_challenge::Challenge;
use std::io::{self, Error, ErrorKind, Read};

mod core_challenge;
mod template;
mod util;

mod challenge01;
mod challenge02;
mod challenge03;
mod challenge04;
mod challenge05;
mod challenge06;
mod challenge07;
mod challenge08;

fn exec(day: u32, part: &str, stdin: String) -> Result<String, Error> {
    let challenge: Box<dyn Challenge> = match day {
        0 => Box::new(template::ChallengeTemplate {}),
        1 => Box::new(challenge01::Challenge01 {}),
        2 => Box::new(challenge02::Challenge02 {}),
        3 => Box::new(challenge03::Challenge03 {}),
        4 => Box::new(challenge04::Challenge04 {}),
        5 => Box::new(challenge05::Challenge05 {}),
        6 => Box::new(challenge06::Challenge06 {}),
        7 => Box::new(challenge07::Challenge07 {}),
        8 => Box::new(challenge08::Challenge08 {}),
        _ => return Err(Error::new(ErrorKind::Other, "Invalid day")),
    };

    match part {
        "a" => Ok(challenge.run_part_a(stdin)),
        "b" => Ok(challenge.run_part_b(stdin)),
        _ => return Err(Error::new(ErrorKind::Other, "Invalid part")),
    }
}

fn main() -> std::io::Result<()> {
    let matches = App::new("Henriks AOC 2020 solutions")
        .arg(Arg::with_name("day").required(true).index(1))
        .arg(Arg::with_name("part").required(true).index(2))
        .get_matches();

    let day = value_t!(matches.value_of("day"), u32).unwrap();
    let part = matches.value_of("part").unwrap();

    let mut stdin = String::new();
    io::stdin().read_to_string(&mut stdin)?;
    stdin = stdin.trim().to_string();

    match exec(day, &part, stdin) {
        Ok(result) => {
            println!("{}", &result);
            Ok(())
        }
        Err(error) => Err(error),
    }
}
