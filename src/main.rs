use crate::app::App;
use crate::sudoku::Sudoku;

pub mod app;
pub mod event;
pub mod ui;
pub mod sudoku;
use crate::sudoku::backtracking_solver;


fn main() {
    let mut sudoku = Sudoku::new();
    let mut iteration: u16 = 0;
    let locked_num: Vec<[usize; 3]> = vec![[0, 1, 1], [1, 7, 1], [8, 8, 1], [4, 1, 7]];

    let terminal: ratatui::prelude::Terminal<ratatui::prelude::CrosstermBackend<_>> = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    let _ = result;

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
