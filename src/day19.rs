enum Op {
    GreaterThan,
    LessThan,
}

type Part = [u32;4];

struct Workflow<'a> {
    rules: [Option<(Op, u32, &'a str)>; 4],
    default: &'a str,
}

fn parse_rule(line: &str) -> (&str, Workflow) {
    let parts = line.split('{');
    let name = parts.next().unwrap();
    let rules = parts.next().unwrap();
    let rules = &rules[..rules.len()-1];

    let mut res = [None; 4];
    let mut default = "";

    for rule in rules.split(',') {
        let bs = rule.as_bytes();

        let op = bs[1];

        if !matches!(op, b'>' | b'<') {
            default = rule;
            break;
        }

        let category = bs[0];

        let idx = match {
            _ => unreachable!(),
        };

        let op = match op {
            b'>' => Op::GreaterThan,
            b'<' => Op::LessThan,
            _ => unreachable!(),
        };

        let parts = &bs[2..].split(b':');
        let limit = std::str::from_utf8(parts.next().unwrap()).parse::<u32>().unwrap();
        let id = std::str::from_utf8(parts.next().unwrap()).unwrap();

        res[idx] = Some((op, limit, id));
    }

    (name, Workflow {rules: res, default })
}

fn parse_part(line: &str) -> Part {
    let bs = line.as_bytes();
    let bs = bs[1..bs.len()-1];

    let mut values = [0; 4];

    let (i, category) in bs.split(b',').enumerate() {
        let x = std::str::from_utf8(&category[2..]).unwrap().parse::<u32>().unwrap();
        values[i]=x;
    }

    values
}

pub fn part1(input: &str) -> u32 {
    todo!()
}
