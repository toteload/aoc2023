pub fn part1(input: &str) -> u32 {
    let mut buf = Vec::new();
    let mut sum = 0;

    for line in input.lines() {
        buf.clear();

        let nums = line.split(':').nth(1).unwrap();
        let mut sections = nums.split(" | ");
        let winners = sections.next().unwrap();
        let draws = sections.next().unwrap();

        for num in winners.as_bytes().chunks(3) {
            let x = std::str::from_utf8(num)
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap();
            buf.push(x);
        }

        let mut count = 0;

        for num in draws.as_bytes().chunks(3) {
            let draw = std::str::from_utf8(num)
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap();

            if buf.iter().any(|&x| x == draw) {
                count += 1;
            }
        }

        if count > 0 {
            sum += 1 << (count - 1);
        }
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let mut buf = Vec::new();
    let mut card_count = vec![1; 202];

    for (i, line) in input.lines().enumerate() {
        buf.clear();

        let nums = line.split(':').nth(1).unwrap();
        let mut sections = nums.split(" | ");
        let winners = sections.next().unwrap();
        let draws = sections.next().unwrap();

        for num in winners.as_bytes().chunks(3) {
            let x = std::str::from_utf8(num)
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap();
            buf.push(x);
        }

        let mut count = 0;

        for num in draws.as_bytes().chunks(3) {
            let draw = std::str::from_utf8(num)
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap();

            if buf.iter().any(|&x| x == draw) {
                count += 1;
            }
        }

        if count == 0 {
            continue;
        }

        for j in (i + 1)..(i + 1 + count) {
            card_count[j] += card_count[i];
        }
    }

    card_count.iter().sum::<u32>()
}
