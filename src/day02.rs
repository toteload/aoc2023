pub fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for (idx, line) in input.lines().enumerate() {
        let draws = line.rsplit(':').next().unwrap().split(';');

        'game: {
            for draw in draws {
                for grab in draw.split(',') {
                    // Skip the preceding space
                    let grab = &grab[1..];

                    let mut it = grab.split(' ');
                    let amount = it.next().unwrap().parse::<u8>().unwrap();
                    let color = it.next().unwrap();

                    match color {
                        "red" if amount > 12 => break 'game,
                        "green" if amount > 13 => break 'game,
                        "blue" if amount > 14 => break 'game,
                        _ => (),
                    }
                }
            }

            sum += idx as u32 + 1;
        }
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let draws = line.rsplit(':').next().unwrap().split(';');

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for draw in draws {
            for grab in draw.split(',') {
                // Skip the preceding space
                let grab = &grab[1..];

                let mut it = grab.split(' ');
                let amount = it.next().unwrap().parse::<u32>().unwrap();
                let color = it.next().unwrap();

                match color {
                    "red" => red = red.max(amount),
                    "green" => green = green.max(amount),
                    "blue" => blue = blue.max(amount),
                    _ => unreachable!(),
                }
            }
        }

        sum += red * green * blue;
    }

    sum
}
