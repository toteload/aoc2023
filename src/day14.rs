use std::collections::HashSet;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Bitmap {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Bitmap {
    fn parse(input: &str) -> Bitmap {
        let lines = input.lines().collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();

        let data = lines
            .into_iter()
            .flat_map(|line| line.as_bytes().iter().copied())
            .collect();

        let res = Bitmap {
            data,
            width,
            height,
        };

        res.rotate_clockwise()
    }

    fn rotate_clockwise(&self) -> Bitmap {
        let mut data = Vec::new();

        for i in 0..self.width {
            for j in (0..self.height).rev() {
                data.push(self.data[j * self.width + i]);
            }
        }

        Bitmap {
            data,
            width: self.height,
            height: self.width,
        }
    }

    fn tilt(&self) -> Bitmap {
        let mut data = self
            .data
            .iter()
            .map(|&c| if c == b'O' { b'.' } else { c })
            .collect::<Vec<_>>();

        for (k, line) in self.data.as_slice().chunks_exact(self.width).enumerate() {
            let mut rock_count = 0;

            for (i, c) in line.iter().enumerate() {
                if *c == b'#' {
                    for j in 0..rock_count {
                        data[k * self.width + i - j - 1] = b'O';
                    }

                    rock_count = 0;
                }

                if *c == b'O' {
                    rock_count += 1;
                }
            }

            for j in 0..rock_count {
                data[k * self.width + line.len() - j - 1] = b'O';
            }
        }

        Bitmap {
            data,
            width: self.width,
            height: self.height,
        }
    }

    fn total_load(&self) -> u32 {
        let mut load = 0;

        for line in self.data.as_slice().chunks_exact(self.width) {
            for (i, c) in line.iter().enumerate() {
                if *c == b'O' {
                    load += i + 1;
                }
            }
        }

        load as u32
    }
}

pub fn part1(input: &str) -> u32 {
    Bitmap::parse(input).tilt().total_load()
}

pub fn part2(input: &str) -> u32 {
    let mut platform = Bitmap::parse(input);
    let mut history = Vec::new();

    let start = loop {
        for _ in 0..4 {
            platform = platform.tilt().rotate_clockwise();
        }

        if let Some(idx) = history.iter().position(|x| x == &platform) {
            break idx;
        }

        history.push(platform.clone());
    };

    let period = history.len() - start;
    let offset = (999_999_999 - start) % period;
    history[start + offset].total_load()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(include_str!("../input/day_14.txt")), 108857);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(include_str!("../input/day_14.txt")), 95273);
    }
}
