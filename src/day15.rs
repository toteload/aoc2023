fn xmas_hash(s: &[u8]) -> u8 {
    let mut acc = 0u32;

    for c in s {
        acc += *c as u32;
        acc *= 17;
        acc %= 256;
    }

    acc as u8
}

pub fn part1(input: &str) -> u32 {
    input
        .trim()
        .split(',')
        .map(|s| xmas_hash(s.as_bytes()) as u32)
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    let mut boxes = std::iter::repeat(Vec::new()).take(256).collect::<Vec<_>>();

    for op in input.trim().split(',').map(|s| s.as_bytes()) {
        let (label, lens): (&[u8], u8) = {
            if *op.last().unwrap() == b'-' {
                (&op[..op.len() - 1], 0)
            } else {
                (&op[..op.len() - 2], op[op.len() - 1] - b'0')
            }
        };

        let idx = xmas_hash(label) as usize;
        let j = boxes[idx].iter().position(|(id, _)| id == &label);

        if lens == 0 {
            if let Some(j) = j {
                boxes[idx].remove(j);
            }
        } else if let Some(j) = j {
            boxes[idx][j] = (label, lens);
        } else {
            boxes[idx].push((label, lens));
        }
    }

    let mut answer = 0;

    for (i, b) in boxes.iter().enumerate() {
        for (j, (_, lens)) in b.iter().enumerate() {
            answer += (i + 1) as u32 * (j + 1) as u32 * *lens as u32;
        }
    }

    answer
}
