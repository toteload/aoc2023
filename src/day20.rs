use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn len(&self) -> u64 {
        (self.end - self.start) as u64
    }
}

enum Module {
    Broadcaster,
    FlipFlop(i8),
    Conjuction(Vec<(usize, i8)>),
}

struct State {
    groups: Vec<usize>,
    modules: Vec<(Module, Range)>,
}

impl State {
    fn push_button(&mut self) -> (u64, u64) {
        let mut los = 0;
        let mut his = 0;

        let mut signals = VecDeque::new();
        let (_, broadcast_range) = self.modules[0];
        signals.push_back((0, broadcast_range, -1));

        while let Some((source, range, pulse)) = signals.pop_front() {
            if pulse == 1 {
                his += range.len();
            } else {
                los += range.len();
            }

            for idx in &self.groups[range.start..range.end] {
                let (m, outputs) = &mut self.modules[*idx];
                match m {
                    Module::FlipFlop(x) if pulse == -1 => {
                        let y = *x * -1;
                        *x = y;

                        signals.push_back((*idx, *outputs, y));
                    },
                    Module::Conjuction(inputs) => {
                        let Some((_, x)) = inputs.iter_mut().find(|(i, _)| i == idx) else { unreachable!() };
                        *x = pulse;

                        let all_hi = inputs.iter().all(|&(_, x)| x == 1);

                        signals.push_back((*idx, *outputs, if all_hi { 1 } else { -1 }));
                    },
                    _ => unreachable!(),
                }
            }
        }

        (los, his)
    }
}

fn parse_id(id: &str) -> u32 {
    let bs = id.as_bytes();

    debug_assert!(bs.len() <= 4);

    let mut acc = 0u32;
    for x in bs {
        acc *= 256;
        acc += *x as u32;
    }

    acc
}

fn get_or_add_index(ids: &mut HashMap<u32, usize>, id: u32) -> usize {
    let n = ids.len();
    *ids.entry(id).or_insert(n)
}

fn parse_line(ids: &mut HashMap<u32, usize>, groups: &mut Vec<usize>, line: &str) -> (usize, Module, Range) {
    let kind = &line[0..1];

    if kind == "b" {
        let start = groups.len();

        let outputs = line.split(" -> ").nth(1).unwrap().split(", ").map(parse_id);

        for id in outputs {
            groups.push(get_or_add_index(ids, id));
        }

        let end = groups.len();

        return (0, Module::Broadcaster, Range { start, end });
    }

    let module = match kind {
        "%" => Module::FlipFlop(-1),
        "&" => Module::Conjuction(Vec::new()),
        _ => unreachable!(),
    };

    let mut parts = line.split(" -> ");

    let id_str = parts.next().unwrap();
    let id = get_or_add_index(ids, parse_id(&id_str[1..]));

    let start = ids.len();

    let outputs = parts.next().unwrap().split(", ").map(parse_id);

    for id in outputs {
        groups.push(get_or_add_index(ids, id));
    }

    let end = ids.len();

    (id, module, Range { start, end })
}

fn parse_circuit(input: &str) -> State {
    let mut ids = HashMap::new();

    let mut groups = Vec::new();
    let mut modules = Vec::new();

    ids.insert(0, 0);

    for line in input.lines() {
        let (idx, m, range) = parse_line(&mut ids, &mut groups, line);

        if modules.len() < idx {
            modules.reserve(modules.len() - idx + 1);
            unsafe {
                modules.set_len(idx+1);
            }
        }

        modules[idx] = (m, range);
    }

    for (conjuction_idx, (m, _)) in modules.iter_mut().enumerate() {
        if let Module::Conjuction(inputs) = m {
            for (idx, (_, range)) in modules.iter().enumerate() {
                for output_idx in &groups[range.start..range.end] {
                    if *output_idx == conjuction_idx {
                        inputs.push((conjuction_idx, -1));
                    }
                }
            }
        }
    }

    State {
        groups,
        modules,
    }
}

pub fn part1(input: &str) -> u64 {
    let mut state = parse_circuit(input);

    let mut lo_pulse_count = 0;
    let mut hi_pulse_count = 0;

    for _ in 0..1000 {
        let (lo, hi) = state.push_button();
        lo_pulse_count += lo;
        hi_pulse_count += hi;
    }

    lo_pulse_count * hi_pulse_count
}

pub fn part2(input: &str) -> u64 {
    todo!()
}
