mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    // Part 1
    let p1_out = match day_4::part1("./src/Inputs/D4.txt") {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    println!("Part 1 output: {p1_out}!");
    // Part 2
    let p2_out = match day_3::part2("./src/Inputs/D3.txt") {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    println!("Part 2 output: {p2_out}!");
}
