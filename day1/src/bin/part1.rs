fn main() {
    let res: i32 = aoc::parser::lines_from_args_as::<i32>(1).sum();
    println!("result: {}", res);
}
