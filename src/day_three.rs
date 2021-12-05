pub mod puzzle_one {
    use std::fs;
    use std::path::Path;


    pub fn run(path: &Path) {
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let lines = input.lines();

        let len = input.lines().next().unwrap().len();
        println!("len {}", len);
        let mut counters: Vec<Counter> = Vec::new();
        for _i in 0..len {
            counters.push(built_counter())
        }

        for line in lines {
            for (i, c) in line.chars().enumerate(){
                match c {
                    '1' => counters[i].one += 1,
                    '0' => counters[i].zero += 1,
                    _ => panic!("A non bit digit found")
                }
            }
        }

        let mut gamma = 0;
        let mut epsilon = 0;
        for i in 0..len {
            gamma = gamma << 1;
            epsilon = epsilon << 1;
            let c = &counters[i];

            if c.one > c.zero {gamma += 1}
            else { epsilon += 1 }
        }

        println!("G: {}, E: {}", gamma, epsilon);
        println!("Result is: {}", epsilon * gamma);

        #[derive(Debug)]
        struct Counter {
            zero:i32,
            one:i32,
        }
        fn built_counter() -> Counter{
            Counter{
                zero: 0,
                one: 0
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

        let len = input.lines().next().unwrap().len();

        let oxi = find_oxi(0, input.lines().collect(), len).unwrap();
        let co2 = find_co2(0, input.lines().collect(), len).unwrap();
        println!("oxi: {} co2: {}", oxi, co2);
        let mut oxi = oxi.chars();
        let mut co2 = co2.chars();

        let mut oxinumber = 0;
        let mut co2number = 0;

        for _i in 0..len {
            oxinumber = oxinumber << 1;
            oxinumber += oxi.next().unwrap().to_string().parse::<i32>().unwrap();

            co2number = co2number << 1;
            co2number += co2.next().unwrap().to_string().parse::<i32>().unwrap();
        }

        println!("Result: {}", oxinumber * co2number);

        fn find_oxi(char_counter: usize, candidates: Vec<&str>, len: usize) -> Option<String> {

            if char_counter >= len {return  None}
            let counter = filled_counter(&candidates, char_counter);
            let mut result: Vec<&str> = Vec::new();
            for s in candidates {
                let c: char = s.chars().nth(char_counter).unwrap();

                if counter.zero > counter.one {
                    if c == '0' {result.push(s)}
                }
                else {
                    if c == '1' {result.push(s)}
                }
            }

            if result.len() == 0 {return None}
            else if result.len() == 1 {return Some(result[0].to_string())}
            else { return find_oxi(char_counter + 1, result, len) }
        }

        fn find_co2(char_counter: usize, candidates: Vec<&str>, len: usize) -> Option<String> {

            if char_counter >= len {return  None}
            let counter = filled_counter(&candidates, char_counter);
            let mut result: Vec<&str> = Vec::new();
            for s in candidates {
                let c: char = s.chars().nth(char_counter).unwrap();

                if counter.zero <= counter.one {
                    if c == '0' {result.push(s)}
                }
                else {
                    if c == '1' {result.push(s)}
                }
            }

            if result.len() == 0 {return None}
            else if result.len() == 1 {return Some(result[0].to_string())}
            else { return find_co2(char_counter + 1, result, len) }
        }

        fn filled_counter(input: &Vec<&str>, char_counter: usize) -> Counter {
            let mut counter = built_counter();
            for s in input{
                let mut chars = s.chars();
                match chars.nth(char_counter).unwrap() {
                    '1' => counter.one += 1,
                    '0' => counter.zero += 1,
                    _ => panic!("A non bit digit found")
                }
            }
            return counter
        }


        #[derive(Debug)]
        struct Counter {
            zero:i32,
            one:i32,
        }
        fn built_counter() -> Counter{
            Counter{
                zero: 0,
                one: 0
            }
        }
    }
}