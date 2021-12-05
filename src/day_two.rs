pub mod puzzle_one {
    use std::fs;
    use std::path::Path;


    pub fn run(path: &Path) {
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let lines = input.lines();


        let mut commands: Vec<Command> = Vec::new();
        for line in lines{
            let mut split = line.split(" ");
            let word = String::from(split.next().unwrap());
            let value = split.next().unwrap().trim().parse().expect("Line was not a number");
            commands.push(built_command(word, value));
        }

        let mut depth:i32 = 0;
        let mut horizontal:i32 = 0;

        for command in commands  {
            let word = command.word.as_str();
            match word {
                "forward" => { horizontal += command.value }
                "down" => { depth += command.value }
                "up" => { depth -= command.value }
                _ => {}
            }
        }

        let result = depth * horizontal;

        println!("Result: {}", result);

        struct Command {
            word:String,
            value:i32
        }
        fn built_command(word:String, value:i32) -> Command{
            Command {
                word,
                value
            }
        }
    }
}

pub mod puzzle_two {
    use std::fs;
    use std::path::Path;

    pub fn run(path: &Path) {
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let lines = input.lines();


        let mut commands: Vec<Command> = Vec::new();
        for line in lines{
            let mut split = line.split(" ");
            let word = String::from(split.next().unwrap());
            let value = split.next().unwrap().trim().parse().expect("Line was not a number");
            commands.push(built_command(word, value));
        }

        let mut depth:i32 = 0;
        let mut aim:i32 = 0;
        let mut horizontal:i32 = 0;

        for command in commands  {
            let word = command.word.as_str();
            match word {
                "forward" => {
                    horizontal += command.value;
                    depth += aim * command.value}
                "down" => { aim += command.value }
                "up" => { aim -= command.value }
                _ => {}
            }
        }

        let result = depth * horizontal;

        println!("Result: {}", result);

        struct Command {
            word:String,
            value:i32
        }
        fn built_command(word:String, value:i32) -> Command{
            Command {
                word,
                value
            }
        }
    }
}