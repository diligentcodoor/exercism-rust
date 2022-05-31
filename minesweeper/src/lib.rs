use std::fmt::{self, Display, Formatter};

enum Cell {
    Mine,
    Count(u8),
}

struct Row {
    cells: Vec<Cell>,
}

impl Row {
    fn new(cells: Vec<Cell>) -> Self {
        Row { cells }
    }

    fn update_slot(&mut self, i: usize) {
        if let Some(to_update) = self.cells.get_mut(i) {
            if let Cell::Count(count) = to_update {
                *count += 1;
            }
        }
    }
}

impl From<&str> for Row {
    fn from(string: &str) -> Self {
        let cells = string
            .chars()
            .map(|ch| match ch {
                '*' => Cell::Mine,
                ' ' => Cell::Count(0),
                _ => panic!("Invalid character: {}", ch),
            })
            .collect();
        Row::new(cells)
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for cell in &self.cells {
            match cell {
                Cell::Mine => write!(f, "*")?,
                Cell::Count(count) if *count == 0 => write!(f, " ")?,
                Cell::Count(count) => write!(f, "{}", count)?,
            }
        }
        Ok(())
    }
}

struct Board {
    rows: Vec<Row>,
}

impl Board {
    fn new(rows: Vec<Row>) -> Self {
        Board { rows }
    }

    fn into_vec_string(self) -> Vec<String> {
        self.rows.into_iter().map(|row| row.to_string()).collect()
    }

    fn annotate(&mut self) {
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].cells.len() {
                if let Cell::Mine = self.rows[i].cells[j] {
                    for a in vec![-1, 0, 1] {
                        for b in vec![-1, 0, 1] {
                            if let Some(next_row) = self.rows.get_mut((i as i8 + a) as usize) {
                                next_row.update_slot((j as i8 + b) as usize);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl From<&[&str]> for Board {
    fn from(strings: &[&str]) -> Self {
        let rows = strings.iter().map(|&r| Row::from(r)).collect();
        Board::new(rows)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            write!(f, "{}\n", row)?;
        }
        Ok(())
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut board = Board::from(minefield);
    println!("{}", board);
    board.annotate();
    board.into_vec_string()
}
