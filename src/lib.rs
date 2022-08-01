#[derive(Debug)]
pub struct SudokuGrid {
    grid: Vec<SudokuCell>,
}

impl SudokuGrid {
    pub fn from(serialized_format: &str) -> Self {
        if serialized_format.len() != 81 {
            panic!("Input invalid, Sudoku is always a 9x9 grid");
        }
        let mut grid: Vec<SudokuCell> = Vec::with_capacity(81);

        for c in serialized_format.chars() {
            if !c.is_ascii_digit() {
                panic!("non-numeric char in serialized string");
            }
            let cell = match c {
                c if !c.is_ascii_digit() => panic!("non-numeric char in serialized string"),
                c if c == '0' => SudokuCell::NullCell,
                c => SudokuCell::FixelCell(c.to_digit(10).unwrap() as u8),
            };
            grid.push(cell);
        }
        Self { grid }
    }

    pub fn serialize(&self) -> String {
        let mut s = String::new();
        for c in &self.grid {
            match &c {
                SudokuCell::FixelCell(n) => s.push_str(format!("{}", n).as_str()),
                SudokuCell::DynCell(o) => s.push_str(format!("{}", o.value).as_str()),
                _ => s.push('_'),
            }
        }
        s
    }

    //Briefly, a program would solve a puzzle by placing the digit "1" in the first cell and checking if it is allowed to be there.
    //If there are no violations (checking row, column, and box constraints) then the algorithm advances to the next cell and places a "1" in that cell.
    // When checking for violations, if it is discovered that the "1" is not allowed, the value is advanced to "2". If a cell is discovered where none of the 9 digits is allowed,
    //then the algorithm leaves that cell blank and moves back to the previous cell. The value in that cell is then incremented by one. This is repeated until the allowed value in the last (81st) cell is discovered.
    pub fn solve(&mut self) -> bool {
        self.solve_internal(0)
    }

    fn solve_internal(&mut self, index: usize) -> bool {
        if index > 80 {
            return true;
        }

        if let SudokuCell::FixelCell(_) = self.grid[index] {
            if self.solve_internal(index + 1) {
                return true;
            } else if let SudokuCell::DynCell(_) = &self.grid[index + 1] {
                //println!("Resetting after fix for {:?} @ {index}", self.grid[index]);
                self.grid[index + 1] = SudokuCell::NullCell;
                return false;
            } else {
                return false;
            }
        } else if let SudokuCell::NullCell = &self.grid[index] {
            self.grid[index] = SudokuCell::DynCell(Default::default());
        }

        loop {
            if self.is_valid_at_index(index) {
                if self.solve_internal(index + 1) {
                    return true;
                } else if let SudokuCell::DynCell(_) = &self.grid[index + 1] {
                    self.grid[index + 1] = SudokuCell::NullCell;
                }
            }
            if let SudokuCell::DynCell(o) = &mut self.grid[index] {
                if !o.set_next_value() {
                    //println!("Resetting for {:?} @ {index}", self.grid[index]);
                    return false;
                }
            }
        }
    }

    fn is_valid_at_index(&self, index: usize) -> bool {
        let i = index / 9;
        let row_valid = SudokuGrid::is_region_unique(&self.rows(i));
        if !row_valid {
            return false;
        }

        let i = index % 9;
        let coll_valid = SudokuGrid::is_region_unique(&self.colls(i));
        if !coll_valid {
            return false;
        }

        let mut remain = index;
        let mut divided = 0;
        while remain > 8 {
            divided += 1;
            remain -= 9
        }

        let row_count = divided / 3;
        let coll_count = remain / 3;

        let i = row_count + coll_count;

        let grid_valid = SudokuGrid::is_region_unique(&self.sub_grids(i));
        if !grid_valid {
            return false;
        }

        true
    }

    fn rows(&self, index: usize) -> [&SudokuCell; 9] {
        if index > 8 {
            panic!(" Can't have index bigger than 8");
        }
        let mut arr = [&SudokuCell::NullCell; 9];
        for (i, item) in arr.iter_mut().enumerate() {
            *item = &self.grid[(index * 9) + i];
        }
        arr
    }

    fn colls(&self, index: usize) -> [&SudokuCell; 9] {
        if index > 8 {
            panic!(" Can't have index bigger than 8");
        }
        let mut arr = [&SudokuCell::NullCell; 9];
        for (i, item) in arr.iter_mut().enumerate() {
            *item = &self.grid[(i * 9) + index];
        }
        arr
    }

    fn sub_grids(&self, index: usize) -> [&SudokuCell; 9] {
        if index > 8 {
            panic!(" Can't have index bigger than 8");
        }
        let mut arr = [&SudokuCell::NullCell; 9];

        let row_range = {
            if index < 3 {
                0..=2
            } else if index < 6 {
                3..=5
            } else {
                6..=8
            }
        };
        let el_range = match index {
            0 => 0..=2,
            1 => 3..=5,
            2 => 6..=8,
            3 => 0..=2,
            4 => 3..=5,
            5 => 6..=8,
            6 => 0..=2,
            7 => 3..=5,
            8 => 6..=8,
            _ => panic!("Out of range"),
        };
        for (i, row_nr) in row_range.enumerate() {
            for (j, el_nr) in el_range.clone().enumerate() {
                arr[i * 3 + j] = self.rows(row_nr)[el_nr];
            }
        }
        arr
    }

    fn is_region_unique(region: &[&SudokuCell]) -> bool {
        let mut arr = [false; 9];

        for &el in region.iter() {
            let n = match el {
                SudokuCell::FixelCell(n) => n,
                SudokuCell::DynCell(o) => &o.value,
                SudokuCell::NullCell => continue,
            };
            if arr[usize::from(n - 1)] {
                return false;
            }
            arr[usize::from(n - 1)] = !arr[usize::from(n - 1)];
        }
        true
    }
}

#[derive(Debug)]
pub enum SudokuCell {
    NullCell,
    FixelCell(u8),
    DynCell(DynCellOption),
}

#[derive(Debug)]
pub struct DynCellOption {
    value: u8,
}

impl DynCellOption {
    pub fn new() -> DynCellOption {
        DynCellOption { value: 1 }
    }

    pub fn set_next_value(&mut self) -> bool {
        self.value += 1;
        if self.value > 9 {
            return false;
        }
        true
    }
}

impl Default for DynCellOption {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sudoku_1() {
        let mut grid = SudokuGrid::from(
            "200006009070008500860950037100030090589400371006090425607040010010780964400603002",
        );

        let res = grid.solve();
        assert!(res);

        assert_eq!(
            grid.serialize(),
            "245376189973128546861954237124537698589462371736891425657249813312785964498613752"
        )
    }
}
