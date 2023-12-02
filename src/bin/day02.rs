const INPUT: &str = include_str!("../../input/day_02.txt");

fn part_1(input: &str) {
    let mut sum = 0;

    for (idx, line) in input.lines().enumerate() {
        let draws = line.rsplit(':').next().unwrap().split(';');

        'game: {
            for draw in draws {
                for grab in draw.split(',') {
                    let grab = &grab[1..];

                    let mut it = grab.split(' ');
                    let amount = it.next().unwrap();
                    let color = it.next().unwrap();

                    if color == "red" {
                        let x = amount.parse::<u8>().unwrap();
                        if x > 12 {
                            break 'game;
                        }
                    }

                    if color == "green" {
                        let x = amount.parse::<u8>().unwrap();
                        if x > 13 {
                            break 'game;
                        }
                    }

                    if color == "blue" {
                        let x = amount.parse::<u8>().unwrap();
                        if x > 14 {
                            break 'game;
                        }
                    }
                }
            }

            sum += idx + 1;
        }
    }

    assert_eq!(sum, 2169);
}

fn part_2(input: &str) {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let draws = line.rsplit(':').next().unwrap().split(';');

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for draw in draws {
            for grab in draw.split(',') {
                let grab = &grab[1..];

                let mut it = grab.split(' ');
                let amount = it.next().unwrap();
                let color = it.next().unwrap();

                if color == "red" {
                    let x = amount.parse::<u32>().unwrap();
                    red = red.max(x);
                }

                if color == "green" {
                    let x = amount.parse::<u32>().unwrap();
                    green = green.max(x);
                }

                if color == "blue" {
                    let x = amount.parse::<u32>().unwrap();
                    blue = blue.max(x);
                }
            }
        }


        sum += red * green * blue;
    }

    assert_eq!(sum, 60948);
}

fn main() {
    part_1(INPUT);
    part_2(INPUT);
}
