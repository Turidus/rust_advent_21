mod day_one;

/*#[macro_use] extern crate lazy_static;*/
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = String::from("Usage: rust_advent[.exe] day:int puzzle:int fileName:String. Example: rust_advent.exe 1 2 1_2");
    println!("{:?}", args);

    let day: u8 = args.get(1).expect(&*usage).parse().expect(&*usage);

    if day < 1 || day > 24 {panic!("{}", usage)}

    let puzzle: u8 = args.get(2).expect(&*usage).parse().expect(&*usage);

    if puzzle < 1  || puzzle > 2 {panic!("{}", usage)}

    let f_name = args.get(3).expect(&*usage);

    let base = env::current_dir().unwrap();
    let path= match base.parent(){
        None => {base.join("inputFiles").join(f_name)}
        Some(parent) => {parent.join("inputFiles").join(f_name)}
    };

    match day {
        1 => {
            if puzzle == 1 {
                day_one::puzzle_one::run(&*path)
            }
            else {
                day_one::puzzle_two::run(&*path)
            }
        }
        _ => {panic!("{}", usage)}
    }
}