mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

fn main() {
    // Part 1
    let p1_out = day_8::part1("./src/Inputs/D8.txt", 1000);
    println!("Part 1 output: {p1_out}!");
    // Part 2
    let p2_out = day_8::part2("./src/Inputs/D8.txt");
    println!("Part 2 output: {p2_out}!");
}
