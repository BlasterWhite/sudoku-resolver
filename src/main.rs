use core::net;

use colored::{ColoredString, Colorize};

pub struct Cell {
    number: u8, // Number of the cell
    is_editable: bool, 
    is_valid: bool,
}

pub struct Boxe {
    cells: [[Cell; 3]; 3],
    is_valid: bool,
}

pub struct Sudoku {
    boxes: [[Boxe; 3]; 3],
    is_valid: bool,
}

impl Cell {
    pub fn new(number: u8, is_editable: bool, is_valid: bool) -> Self {
        Self { number, is_editable, is_valid }
    }

    pub fn get_number(&self) -> &u8 {
        return  &self.number;
    }

    pub fn is_editable(&self) -> bool {
        return self.is_editable;
    }

    pub fn is_valid(&self) -> bool {
        return self.is_valid;
    }

    pub fn set_number(&mut self, number: u8) {
        self.number = number;
    }

    pub fn set_validity(&mut self, is_valid: bool) {
        self.is_valid = is_valid;
    }

    pub fn to_colored_string(&self) -> ColoredString {
        if !self.is_editable {
            return self.number.to_string().blue();
        } else if !self.is_valid {
            return self.number.to_string().red();
        } else {
            return self.number.to_string().green();
        }
    }

}

impl Boxe {
    pub fn new() -> Self {
        Self { cells: [
            [Cell::new(0, true, true),Cell::new(0, true, true),Cell::new(0, true, true)],
            [Cell::new(0, true, true),Cell::new(0, true, true),Cell::new(0, true, true)],
            [Cell::new(0, true, true),Cell::new(0, true, true),Cell::new(0, true, true)],
        ], is_valid: true }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> &Cell {
        return &self.cells[row % 3][column %3];
    }

    pub fn is_valid(&self) -> bool {
        return self.is_valid;
    }

    pub fn reset_error(&mut self) {
        for row in 0..3 {
            for column in 0..3 {
                self.cells[row][column].set_validity(true);
            }
        }
    }

    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> &mut Cell {
        &mut self.cells[row % 3][col % 3]
    }

    pub fn check_region(&self) -> bool {
        let mut is_valid = true;
        let mut region_numbers : Vec<u8> = vec![];

        for row in 0..3 {
            for column in 0..3 {
                let num = self.get_cell(row, column).get_number();
                if region_numbers.contains(num) {
                    is_valid = false;
                }
                if *num != 0 { // Avoid 0 witch is empty value
                    region_numbers.push(*num);
                }
            }
        }

        return is_valid;
    }
}

impl Sudoku {
    pub fn new() -> Self {
        Self { boxes: [
            [Boxe::new(),Boxe::new(),Boxe::new()],
            [Boxe::new(),Boxe::new(),Boxe::new()],
            [Boxe::new(),Boxe::new(),Boxe::new()],
        ], is_valid: true }
    }

    pub fn get_boxe(&self, row: usize, column: usize) -> &Boxe {
        return &self.boxes[row % 3][column % 3];
    }

    pub fn get_boxe_mut(&mut self, row: usize, col: usize) -> &mut Boxe {
        &mut self.boxes[row][col]
    }

    pub fn is_valid(&self) -> bool {
        return self.is_valid;
    }

    pub fn get_number(&self, row: usize, column: usize) -> &u8 {
        let boxe_row = row / 3;
        let boxe_column = column / 3;

        return self.get_boxe(boxe_row, boxe_column).get_cell(row, column).get_number();
    }

    pub fn set_number(&mut self, row: usize, column: usize, number: u8) {
        let boxe_row = row / 3;
        let boxe_column = column / 3;

        self.get_boxe_mut(boxe_row, boxe_column).get_cell_mut(row, column).set_number(number);
    }

    pub fn display(&self) {
        for row in 0..9 {
            for column in 0..9 {
                let boxe_row = row / 3;
                let boxe_column = column / 3;
                print!("  {}", self.get_boxe(boxe_row, boxe_column).get_cell(row % 3, column %3).to_colored_string());

                if column % 3 == 2 { print!("  "); }
            }
            print!("\n\n");
            if row % 3 == 2 { print!("\n"); }
        }
    }

    pub fn reset_error(&mut self) {
        for row in 0..3 {
            for column in 0..3 {
                self.boxes[row][column].reset_error();
            }
        }
    }

    pub fn check_row(&self, row: usize) -> bool {
        let mut row_number: Vec<u8> = vec![];
        let mut is_valid = true;
        let boxe_row = row / 3;
        for column in 0..9 {
            let boxe_column = column / 3; // Index of the boxe column
            // println!("ask boxe {}:{}", boxe_row, boxe_column);
            let num = self.get_boxe(boxe_row, boxe_column).get_cell(row, column).get_number();
            if row_number.contains(num) {
                is_valid = false;
            }
            if *num != 0 { // Avoid 0 witch is empty value
                row_number.push(*num);
            }
        }

        // println!("row: {} -> {:?} | {}", row, row_number, is_valid);
        return is_valid;
    }

    pub fn check_column(&self, column: usize) -> bool {
        let mut column_number: Vec<u8> = vec![];
        let mut is_valid = true;
        let boxe_column = column / 3;
        for row in 0..9 {
            let boxe_row = row / 3; // Index of the boxe row
            let num = self.get_boxe(boxe_row, boxe_column).get_cell(row, column).get_number();
            if column_number.contains(num) {
                is_valid = false;
            }
            if *num != 0 { // Avoid 0 witch is empty value
                column_number.push(*num);
            }
        }
        return is_valid;
    }

    pub fn check(&self) -> bool {
        for row in 0..9 {
            if !self.check_row(row) { // if row not valid return false
                return false;
            }
        }

        for column in 0..9 {
            if !self.check_column(column) { // if column not valid return false
                return false;
            }
        }

        // check all region valididy, return false if one not valid
        for region_row in 0..3 {
            for region_column in 0..3 {
                if !self.boxes[region_row][region_column].check_region() {
                    return false;
                }
            }
        }


        return true;
    }
}


// Si ma grille n'est pas valide avec mon numéro j'essaie de 1 à 9.
    // Si aucunes solution je remet ma case à 0 et je retourne false (aucun chemin possible)

// Si ma grille est valide avec le numéro que je viens d'ajouter
    // Je fais un appel a moi meme pour trouver la case suivante.
        // Si la fonction retourne false j'increment la loop. si j'ai fais le tour je retourne false


fn backtracking_solver(sudoku: &mut Sudoku, row: usize, column: usize, iter: &mut u16) -> bool {
    *iter = *iter + 1;
    if row > 8 {
        return true;
    }

    let next_column = (column + 1) % 9;
    let mut next_row = row;
    if next_column == 0 {
        next_row = row + 1;
    }

    for num in 1..=9 { // Test des valeur de 0 à 9
        // println!("[{}:{}] test with {}", row, column, num);
        sudoku.set_number(row, column, num);
        if sudoku.check() { // Si ma grille est valide
            if backtracking_solver(sudoku, next_row, next_column, iter) {
                break; // Solution trouvé on quitte la boucle
            }
        } 

        if num == 9 { // Si on est 9 et qu'on a pas trouvé de résultat
            // println!("No solution founds in this branch");
            sudoku.set_number(row, column, 0);
            return false;
        }      
    }

    return sudoku.check();
}


fn main() {
    if false {

    // println!("Hello, world!");
    // let mut cell1 = Cell::new(1, true);
    // let boxe1 = Boxe::new();
    let mut sudoku: Sudoku = Sudoku::new();

    // println!("{}", cell1.get_number());
    // cell1.set_number(5);
    // println!("{}", cell1.get_number());

    sudoku.display();
    
    sudoku.reset_error();

    sudoku.set_number(7, 7, 7);

    println!("------------------------------------------"); 

    sudoku.display();

    println!("{}", sudoku.check_row(0));
    println!("{}", sudoku.check_column(0));
    println!("{}\n", sudoku.get_boxe(0, 0).check_region());
    
    for n in 0..9 {
        sudoku.set_number(0, n, (n+1).try_into().unwrap());
        sudoku.set_number(n, 0, (10-(n+1)).try_into().unwrap());
    }

    sudoku.set_number(0, 8, 1);

    sudoku.set_number(1, 1, 1);
    sudoku.set_number(1, 2, 4);
    sudoku.set_number(2, 1, 5);
    sudoku.set_number(2, 2, 6);
    sudoku.set_number(1, 3, 1);
    sudoku.set_number(1, 4, 2);
    sudoku.set_number(1, 5,3);
    

    sudoku.display();

    println!("{}", sudoku.check_row(0));
    println!("{}", sudoku.check_column(0));
    println!("{}", sudoku.get_boxe(0, 0).check_region());
    println!("{}", sudoku.check());

    // let mut valid_sudoku = Sudoku::new();
    // for r in 0..9 {
    //     for c in 0..9 {
    //         // Formula for a valid Sudoku pattern
    //         let val = ((r * 3 + r / 3 + c) % 9) + 1;
    //         valid_sudoku.set_number(r, c, val as u8);
    //     }
    // }

    // valid_sudoku.display();
    // println!("{}", valid_sudoku.check());
    }


    let mut sudoku = Sudoku::new();

    let mut iteration : u16 = 0;

    backtracking_solver(&mut sudoku, 0, 0, &mut iteration);
    
    sudoku.display();

    println!("{}", sudoku.check());
    
    print!("{} itérations", iteration);


}
