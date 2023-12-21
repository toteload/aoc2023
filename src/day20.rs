use std::collections::{HashMap, VecDeque};
use std::mem::MaybeUninit;

const PULSE_HI: i8 = 1;
const PULSE_LO: i8 = -1;

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

#[derive(Clone)]
enum ModuleKind {
    Broadcaster,
    Dummy,
    FlipFlop(i8),
    Conjunction(Vec<(usize, i8)>),
}

struct Module<'a> {
    kind: ModuleKind,
    id: &'a str,
    outputs: Range,
}

impl<'a> Module<'a> {
    fn dummy() -> Module<'a> {
        Module {
            kind: ModuleKind::Dummy,
            id: "dummy",
            outputs: Range { start: 0, end: 0 },
        }
    }
}

struct State<'a> {
    groups: Vec<usize>,
    modules: Vec<Module<'a>>,
}

impl State<'_> {
    fn push_button(&mut self) -> (u64, u64) {
        let mut los = 1;
        let mut his = 0;

        let mut signals = VecDeque::new();
        let Module { outputs: broadcast_range, .. } = self.modules[0];
        signals.push_back((0, broadcast_range, PULSE_LO));

        while let Some((source, range, pulse)) = signals.pop_front() {
            if pulse == PULSE_HI {
                his += range.len();
            } else {
                los += range.len();
            }

            for idx in &self.groups[range.start..range.end] {
                let Module { kind: m, outputs, .. }= &mut self.modules[*idx];
                match m {
                    ModuleKind::FlipFlop(x) if pulse == PULSE_LO => {
                        let y = *x * -1;
                        *x = y;

                        signals.push_back((*idx, *outputs, y));
                    },
                    ModuleKind::Conjunction(inputs) => {
                        let Some((_, x)) = inputs.iter_mut().find(|(i, _)| *i == source) else { unreachable!() };
                        *x = pulse;

                        let all_hi = inputs.iter().all(|&(_, x)| x == PULSE_HI);

                        signals.push_back((*idx, *outputs, if all_hi { PULSE_LO } else { PULSE_HI }));
                    },
                    _ => (),
                }
            }
        }

        (los, his)
    }

    fn push_button_part2(&mut self) -> bool {
        let mut signals = VecDeque::new();
        let Module { outputs: broadcast_range, .. } = self.modules[0];
        signals.push_back((0, broadcast_range, PULSE_LO));

        while let Some((source, range, pulse)) = signals.pop_front() {
            for idx in &self.groups[range.start..range.end] {
                let Module { kind: m, outputs, .. }= &mut self.modules[*idx];
                match m {
                    ModuleKind::Dummy if pulse == PULSE_LO => return true,
                    ModuleKind::FlipFlop(x) if pulse == PULSE_LO => {
                        let y = *x * -1;
                        *x = y;

                        signals.push_back((*idx, *outputs, y));
                    },
                    ModuleKind::Conjunction(inputs) => {
                        let Some((_, x)) = inputs.iter_mut().find(|(i, _)| *i == source) else { unreachable!() };
                        *x = pulse;

                        let all_hi = inputs.iter().all(|&(_, x)| x == PULSE_HI);

                        signals.push_back((*idx, *outputs, if all_hi { PULSE_LO } else { PULSE_HI }));
                    },
                    _ => (),
                }
            }
        }

        false
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

fn parse_module<'a>(ids: &mut HashMap<u32, usize>, groups: &mut Vec<usize>, line: &'a str) -> (usize, Module<'a>) {
    let kind = &line[0..1];

    if kind == "b" {
        let start = groups.len();

        let outputs = line.split(" -> ").nth(1).unwrap().split(", ").map(parse_id);

        for id in outputs {
            groups.push(get_or_add_index(ids, id));
        }

        let end = groups.len();

        return (0, Module {
            id: "broadcaster",
            kind: ModuleKind::Broadcaster, 
            outputs: Range { start, end },
        });
    }

    let kind = match kind {
        "%" => ModuleKind::FlipFlop(PULSE_LO),
        "&" => ModuleKind::Conjunction(Vec::new()),
        _ => unreachable!(),
    };

    let mut parts = line.split(" -> ");

    let id_str = &parts.next().unwrap()[1..];
    let idx = get_or_add_index(ids, parse_id(id_str));

    let start = groups.len();

    let outputs = parts.next().unwrap().split(", ").map(parse_id);

    for id in outputs {
        groups.push(get_or_add_index(ids, id));
    }

    let end = groups.len();

    (idx, Module {
        id: id_str,
        kind, 
        outputs: Range { start, end },
    })
}

fn parse_circuit(input: &str) -> State {
    let mut ids = HashMap::new();

    let mut groups = Vec::new();
    let mut modules: Vec<Module> = Vec::new();

    ids.insert(0, 0);

    for line in input.lines() {
        let (idx, m) = parse_module(&mut ids, &mut groups, line);

        if modules.len() <= idx {
            modules.resize_with(idx + 1, || Module::dummy());
        }

        modules[idx] = m;
    }

    let n = modules.len();

    for conjunction_idx in 0..n {
        match modules[conjunction_idx].kind {
            ModuleKind::Conjunction(_) => (),
            _ => continue,
        }

        for idx in 0..n {
            let outputs = {
                let Module { outputs: range, .. } = modules[idx];
                &groups[range.start..range.end]
            };

            if outputs.contains(&conjunction_idx) {
                let ModuleKind::Conjunction(inputs) = &mut modules[conjunction_idx].kind else { unreachable!() };
                inputs.push((idx, PULSE_LO));
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
    let mut state = parse_circuit(input);
    let mut answer = 1;

    while !state.push_button_part2() {
        answer += 1;

        if answer % 100_000 == 0 {
            println!("{answer}");
        }
    }

    answer
}
