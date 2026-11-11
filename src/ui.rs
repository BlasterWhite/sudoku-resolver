use ratatui::{
    buffer::Buffer,
    layout::{Rect, Constraint, Layout},
    widgets::{Block, Widget, BorderType, Table, Row},
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        
        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(area);
        let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(1); 2]).areas(right);


        
        let rows: [Row; 5] = [
            Row::new(["Eggplant", "1 medium", "25 kcal, 6g carbs, 1g protein"]),
            Row::new(["Tomato", "2 large", "44 kcal, 10g carbs, 2g protein"]),
            Row::new(["Zucchini", "1 medium", "33 kcal, 7g carbs, 2g protein"]),
            Row::new(["Bell Pepper", "1 medium", "24 kcal, 6g carbs, 1g protein"]),
            Row::new(["Garlic", "2 cloves", "9 kcal, 2g carbs, 0.4g protein"]),
            ];
            
        let table = Table::new(rows, [Constraint::Fill(1)]);


        
        Block::bordered().border_type(BorderType::Rounded).title("Sudoku").render(left, buf);
        Block::bordered().border_type(BorderType::Rounded).title("Data").render(top_right, buf);
        Block::bordered().border_type(BorderType::Rounded).title("Settings").render(bottom_right, buf);

    }
}