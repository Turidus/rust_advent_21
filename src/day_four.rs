pub mod puzzle_one {
    use std::fs;
    use std::path::Path;
    use regex::Regex;

    pub fn run(path: &Path) {

        let group_reg = Regex::new(r"[^\S\r\n]*\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[\r\n]+[^\S\r\n]*\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[\r\n]+[^\S\r\n]*\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[\r\n]+[^\S\r\n]*\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[\r\n]+[^\S\r\n]*\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+").unwrap();
        let line_reg = Regex::new(r"[^\S\r\n]*\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+").unwrap();
        let number_reg = Regex::new(r"\d+").unwrap();


        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let mut lines = input.lines();

        let first_line_split = lines.next().unwrap().split(",");
        let mut random_numbers: Vec<i32> = Vec::new();
        for n in first_line_split{
            random_numbers.push(n.parse().unwrap());
        }

        let mut boards: Vec<Board> = Vec::new();
        let parsed_input = group_reg.find_iter(input.as_str());
        for group in parsed_input {
            let mut board = Board::new();

            let line_caps = line_reg.find_iter(group.as_str());
            for line in line_caps {
                let number_caps = number_reg.find_iter(line.as_str());
                let numbers: Vec<i32> = number_caps.filter_map(|digits| digits.as_str().parse().ok()).collect();
                board.add_row(numbers);
            }

            boards.push(board);
        }

        for number in random_numbers {
            for board in &mut boards {
                board.mark_field(number);
                if board.check() {
                    println!("Board: {:?} \nNumber: {}", board, number);
                    println!("Result is {}", board.sum_unmarked_fields() * number);
                    return;
                }
            }
        }
    }

    #[derive(Debug)]
    struct Field {
        value: i32,
        marked: bool
    }
    impl Field {
        fn new(value: i32) -> Field{
            Field {
                value,
                marked: false
            }
        }
    }

    #[derive(Debug)]
    struct Row {
        row: Vec<Field>,
        marked_fields: usize
    }
    impl Row {
        fn new(numbers: Vec<i32>) -> Row {
            let mut row = Row {
                row: Vec::new(),
                marked_fields: 0
            };

            for n in numbers {
                row.row.push(Field::new(n))
            }

            return row
        }

        fn mark_field(&mut self, value: i32) -> bool {
            for mut field in &mut self.row {
                if field.value == value{
                    if !field.marked {
                        field.marked = true;
                        self.marked_fields += 1;
                        return true;
                    }
                    return false
                }
            }
            return false
        }

        fn fully_marked(&self) -> bool {
            return self.marked_fields == self.row.len();
        }
    }

    #[derive(Debug)]
    struct Board {
        board: Vec<Row>,
        marked_fields: usize
    }
    impl Board {
        fn new() -> Board {
            Board {
                board: Vec::new(),
                marked_fields: 0
            }
        }

        fn add_row(&mut self, numbers: Vec<i32>){
            self.board.push(Row::new(numbers));
        }

        fn mark_field(&mut self, value: i32) -> bool {
            for row in &mut self.board {
                if row.mark_field(value) {
                    self.marked_fields += 1;
                    return  true}
            }
            return false
        }

        fn check(&self) -> bool {
            if self.marked_fields < self.board.len() { //Can not be true if there are less than x marked fields in total
                return false
            }

            for row in &self.board { //Check all rows
                if row.fully_marked() {return true}
            }

            for i in 0..self.board.len() { //Check all columns in a square(!) board
                let mut success = true;
                for row in &self.board {
                    success = success && row.row[i].marked
                }
                if success {return true}
            }

            return false
        }

        fn sum_unmarked_fields(&self) -> i32 {
            let mut acc = 0;
            for row in &self.board{
                for field in &row.row {
                    if !field.marked {acc += field.value}
                }
            }
            return acc;
        }
    }
}

pub mod puzzle_two {
    use std::fs;
    use std::path::Path;
    use regex::Regex;

    pub fn run(path: &Path) {

        let group_reg = Regex::new(r"[^\S\r\n]*\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[\r\n]+[^\S\r\n]*\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[\r\n]+[^\S\r\n]*\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[\r\n]+[^\S\r\n]*\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[\r\n]+[^\S\r\n]*\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+").unwrap();
        let line_reg = Regex::new(r"[^\S\r\n]*\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+[^\S\r\n]+\d+").unwrap();
        let number_reg = Regex::new(r"\d+").unwrap();


        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let mut lines = input.lines();

        let first_line_split = lines.next().unwrap().split(",");
        let mut random_numbers: Vec<i32> = Vec::new();
        for n in first_line_split{
            random_numbers.push(n.parse().unwrap());
        }

        let mut boards: Vec<Board> = Vec::new();
        let parsed_input = group_reg.find_iter(input.as_str());
        for group in parsed_input {
            let mut board = Board::new();

            let line_caps = line_reg.find_iter(group.as_str());
            for line in line_caps {
                let number_caps = number_reg.find_iter(line.as_str());
                let numbers: Vec<i32> = number_caps.filter_map(|digits| digits.as_str().parse().ok()).collect();
                board.add_row(numbers);
            }

            boards.push(board);
        }

        for number in random_numbers {
            let mut new_boards: Vec<Board> = Vec::new();
            for board in &mut boards {
                board.mark_field(number);
                if !board.check() {
                    new_boards.push(board.clone())
                }
            }

            if boards.len() == 1 && new_boards.len() == 0 {
                println!("Result: {}", boards[0].sum_unmarked_fields() * number);
                return;
            }
            boards = new_boards;
        }
    }

    #[derive(Debug, Clone)]
    struct Field {
        value: i32,
        marked: bool
    }
    impl Field {
        fn new(value: i32) -> Field{
            Field {
                value,
                marked: false
            }
        }
    }

    #[derive(Debug, Clone)]
    struct Row {
        row: Vec<Field>,
        marked_fields: usize
    }
    impl Row {
        fn new(numbers: Vec<i32>) -> Row {
            let mut row = Row {
                row: Vec::new(),
                marked_fields: 0
            };

            for n in numbers {
                row.row.push(Field::new(n))
            }

            return row
        }

        fn mark_field(&mut self, value: i32) -> bool {
            for mut field in &mut self.row {
                if field.value == value{
                    if !field.marked {
                        field.marked = true;
                        self.marked_fields += 1;
                        return true;
                    }
                    return false
                }
            }
            return false
        }

        fn fully_marked(&self) -> bool {
            return self.marked_fields == self.row.len();
        }
    }

    #[derive(Debug, Clone)]
    struct Board {
        board: Vec<Row>,
        marked_fields: usize
    }
    impl Board {
        fn new() -> Board {
            Board {
                board: Vec::new(),
                marked_fields: 0
            }
        }

        fn add_row(&mut self, numbers: Vec<i32>){
            self.board.push(Row::new(numbers));
        }

        fn mark_field(&mut self, value: i32) -> bool {
            for row in &mut self.board {
                if row.mark_field(value) {
                    self.marked_fields += 1;
                    return  true}
            }
            return false
        }

        fn check(&self) -> bool {
            if self.marked_fields < self.board.len() { //Can not be true if there are less than x marked fields in total
                return false
            }

            for row in &self.board { //Check all rows
                if row.fully_marked() {return true}
            }

            for i in 0..self.board.len() { //Check all columns in a square(!) board
                let mut success = true;
                for row in &self.board {
                    success = success && row.row[i].marked
                }
                if success {return true}
            }

            return false
        }

        fn sum_unmarked_fields(&self) -> i32 {
            let mut acc = 0;
            for row in &self.board{
                for field in &row.row {
                    if !field.marked {acc += field.value}
                }
            }
            return acc;
        }
    }
}