type Id = u16;

enum Module {
    FlipFlop(i8),
    Conjuction(Vec<(Id, i8)>),
}

struct State {
    broadcaster: &[Id],
    modules: Vec<(Id, Module, &[Id])>,
}

fn parse_line(line: &str) -> (Id, Module, Vec<Id>) {
    todo!()
}

pub fn part1(input: &str) -> u64 {
    todo!()
}

pub fn part2(input: &str) -> u64 {
    todo!()
}
