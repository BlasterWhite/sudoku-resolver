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
        Self {
            number,
            is_editable,
            is_valid,
        }
    }

    pub fn get_number(&self) -> &u8 {
        return &self.number;
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

    pub fn set_editable(&mut self, is_editable: bool) {
        self.is_editable = is_editable;
    }

    pub fn to_colored_string(&self) -> ColoredString {
        let mut displayed_string = self.number.to_string();
        if self.number == 0 {
            displayed_string = String::from(".");
        }
        if !self.is_editable {
            return displayed_string.blue();
        } else if !self.is_valid {
            return displayed_string.red();
        } else {
            return displayed_string.green();
        }
    }
}

impl Boxe {
    pub fn new() -> Self {
        Self {
            cells: [
                [
                    Cell::new(0, true, true),
                    Cell::new(0, true, true),
                    Cell::new(0, true, true),
                ],
                [
                    Cell::new(0, true, true),
                    Cell::new(0, true, true),
                    Cell::new(0, true, true),
                ],
                [
                    Cell::new(0, true, true),
                    Cell::new(0, true, true),
                    Cell::new(0, true, true),
                ],
            ],
            is_valid: true,
        }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> &Cell {
        return &self.cells[row % 3][column % 3];
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

    pub fn get_cell_mut(&mut self, row: usize, column: usize) -> &mut Cell {
        &mut self.cells[row % 3][column % 3]
    }

    pub fn check_region(&self) -> bool {
        let mut is_valid = true;
        let mut region_numbers: Vec<u8> = vec![];

        for row in 0..3 {
            for column in 0..3 {
                let num = self.get_cell(row, column).get_number();
                if region_numbers.contains(num) {
                    is_valid = false;
                }
                if *num != 0 {
                    // Avoid 0 witch is empty value
                    region_numbers.push(*num);
                }
            }
        }

        return is_valid;
    }
}

impl Sudoku {
    pub fn new() -> Self {
        Self {
            boxes: [
                [Boxe::new(), Boxe::new(), Boxe::new()],
                [Boxe::new(), Boxe::new(), Boxe::new()],
                [Boxe::new(), Boxe::new(), Boxe::new()],
            ],
            is_valid: true,
        }
    }

    pub fn get_boxe(&self, row: usize, column: usize) -> &Boxe {
        return &self.boxes[row % 3][column % 3];
    }

    pub fn get_boxe_mut(&mut self, row: usize, column: usize) -> &mut Boxe {
        &mut self.boxes[row % 3][column % 3]
    }

    pub fn is_valid(&self) -> bool {
        return self.is_valid;
    }

    pub fn get_number(&self, row: usize, column: usize) -> &u8 {
        let boxe_row = row / 3;
        let boxe_column = column / 3;

        return self
            .get_boxe(boxe_row, boxe_column)
            .get_cell(row, column)
            .get_number();
    }

    pub fn set_number(&mut self, row: usize, column: usize, number: u8) {
        if self.is_editable(row, column) {
            // if editable
            let boxe_row = row / 3;
            let boxe_column = column / 3;

            self.get_boxe_mut(boxe_row, boxe_column)
                .get_cell_mut(row, column)
                .set_number(number);
        }
    }

    pub fn display(&self) {
        for row in 0..9 {
            for column in 0..9 {
                let boxe_row = row / 3;
                let boxe_column = column / 3;
                print!(
                    "  {}",
                    self.get_boxe(boxe_row, boxe_column)
                        .get_cell(row % 3, column % 3)
                        .to_colored_string()
                );

                if column % 3 == 2 {
                    print!("  ");
                }
            }
            print!("\n\n");
            if row % 3 == 2 {
                print!("\n");
            }
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
            let num = self
                .get_boxe(boxe_row, boxe_column)
                .get_cell(row, column)
                .get_number();
            if row_number.contains(num) {
                is_valid = false;
            }
            if *num != 0 {
                // Avoid 0 witch is empty value
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
            let num = self
                .get_boxe(boxe_row, boxe_column)
                .get_cell(row, column)
                .get_number();
            if column_number.contains(num) {
                is_valid = false;
            }
            if *num != 0 {
                // Avoid 0 witch is empty value
                column_number.push(*num);
            }
        }
        return is_valid;
    }

    pub fn check(&self) -> bool {
        for row in 0..9 {
            if !self.check_row(row) {
                // if row not valid return false
                return false;
            }
        }

        for column in 0..9 {
            if !self.check_column(column) {
                // if column not valid return false
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

    pub fn is_editable(&self, row: usize, column: usize) -> bool {
        let boxe_row = row / 3;
        let boxe_column = column / 3;
        return self
            .get_boxe(boxe_row, boxe_column)
            .get_cell(row, column)
            .is_editable();
    }

    pub fn set_editable(&mut self, row: usize, column: usize, editable: bool) {
        let boxe_row = row / 3;
        let boxe_column = column / 3;
        self.get_boxe_mut(boxe_row, boxe_column)
            .get_cell_mut(row, column)
            .set_editable(editable);
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

    if sudoku.is_editable(row, column) {
        for num in 1..=9 {
            // Test des valeur de 0 à 9
            sudoku.set_number(row, column, num);
            if sudoku.check() {
                // Si ma grille est valide
                if backtracking_solver(sudoku, next_row, next_column, iter) {
                    break; // Solution trouvé on quitte la boucle
                }
            }

            if num == 9 {
                // Si on est 9 et qu'on a pas trouvé de résultat
                sudoku.set_number(row, column, 0);
                return false;
            }
        }
    } else {
        return backtracking_solver(sudoku, next_row, next_column, iter);
    }

    return sudoku.check();
}

fn main() {
    let mut sudoku = Sudoku::new();
    let mut iteration: u16 = 0;
    let locked_num: Vec<[usize; 3]> = vec![[0, 1, 1], [1, 7, 1], [8, 8, 1], [4, 1, 7]];

    for num_obj in locked_num {
        sudoku.set_number(num_obj[0], num_obj[1], num_obj[2].try_into().unwrap());
        sudoku.set_editable(num_obj[0], num_obj[1], false);
    }

    println!("Grille de base:\n");
    sudoku.display();

    backtracking_solver(&mut sudoku, 0, 0, &mut iteration);

    if sudoku.check() {
        println!("Solution trouvé en {} itérations:\n", iteration);
        sudoku.display();
    } else {
        println!("Aucune solutions trouvée");
    }
}
