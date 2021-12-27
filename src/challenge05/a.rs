use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn from(from: Point, to: Point) -> Self {
        Line { from, to }
    }

    fn iter(&self) -> LineIterator {
        LineIterator::from(self)
    }
}

struct LineIterator<'a> {
    line: &'a Line,
    current: Point,
    incr: Point,
    is_first: bool,
}

impl<'a> LineIterator<'a> {
    fn from(line: &'a Line) -> Self {
        LineIterator {
            line,

            incr: Point {
                x: normalize(line.to.x - line.from.x),
                y: normalize(line.to.y - line.from.y),
            },

            current: Point {
                x: line.from.x,
                y: line.from.y,
            },

            is_first: true,
        }
    }
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
        } else {
            if self.current.x == self.line.to.x && self.current.y == self.line.to.y {
                return None;
            }

            self.current.x += self.incr.x;
            self.current.y += self.incr.y;
        }

        Some(Point {
            x: self.current.x,
            y: self.current.y,
        })
    }
}

fn parse_coord(str: &str) -> Option<Point> {
    let mut numbers = str.split(",").filter_map(|s| s.parse::<i32>().ok());
    Some(Point {
        x: numbers.next()?,
        y: numbers.next()?,
    })
}

fn parse_line(str: &str) -> Option<Line> {
    let mut coords = str.split(" -> ").filter_map(parse_coord);

    Some(Line::from(coords.next()?, coords.next()?))
}

fn parse_input(input: &str) -> Vec<Line> {
    input.split("\n").filter_map(parse_line).collect()
}

pub fn run(stdin: String) -> String {
    let input = parse_input(&stdin);

    let mut field: HashMap<Point, i32> = HashMap::new();

    input
        .iter()
        .filter(|line| line.from.x == line.to.x || line.from.y == line.to.y)
        .flat_map(|line| line.iter())
        .for_each(|point| {
            *field.entry(point).or_insert(0) += 1;
        });

    let res = field.values().filter(|a| **a >= 2).count();

    res.to_string()
}

fn normalize(diff: i32) -> i32 {
    if diff == 0 {
        0
    } else if diff > 0 {
        1
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::util::testing_utils::read_test_resource;

    #[test]
    fn run() {
        assert_eq!("7142", super::run(read_test_resource("day05.txt")));
    }
}
