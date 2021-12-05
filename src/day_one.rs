pub mod puzzle_one {
    use std::fs;
    use std::path::Path;


    pub fn run(path: &Path) {
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let lines = input.lines();

        let mut acc = 0;
        let mut before = i32::MAX;

        for line in lines {
            let num: i32 = line.trim().parse().expect("Line was not a number");
            if num > before {acc+=1};
            before = num;
        }
        println!("The result is: {}", acc)
    }
}

pub mod puzzle_two {
    use std::fs;
    use std::path::Path;

    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let lines = input.lines();

        let mut numbers: Vec<i32> = Vec::new();
        for line in lines {
             numbers.push(line.trim().parse().expect("Line was not a number"));
        }
        let mut sliding_averages: Vec<i32> = Vec::new();
        for i in 0..numbers.len()-2 {
            sliding_averages.push(numbers[i] + numbers[i+1] + numbers[i+2])
        }

        let mut acc = 0;
        let mut before = i32::MAX;

        for num in sliding_averages {
            if num > before {acc+=1};
            before = num;
        }
        println!("The result is: {}", acc)
    }
}