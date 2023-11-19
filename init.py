import os

for i in range(1, 26):
    open(f"input/{i:02}.txt", "w")

    with open(f"src/day{i:02}.rs", "w") as f:
        f.write("pub fn run(_input: &str) { todo!() }")

for i in range(1,26):
    print(f"{i} => day{i:02}::run(&input),")
