use std::{fmt::{self, Display}, ops::{Deref, RangeInclusive, DerefMut}};
use colored::Colorize;

const GRID_NB_COLUMNS: usize = 7;
const GRID_NB_ROWS: usize = 6;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Case{
    RED,
    YELLOW,
    EMPTY
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    RED,
    YELLOW
}

impl Player {
    pub fn opponent(self) -> Player {
        match self {
            Player::RED => Player::YELLOW,
            Player::YELLOW => Player::RED,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::RED => write!(f, "{}", "🔴 Red".red()),
            Player::YELLOW => write!(f, "{}", "🟡 Yellow".yellow()),
        }
    }
}

impl Into<Case> for Player {
    fn into(self) -> Case {
        match self {
            Player::RED => Case::RED,
            Player::YELLOW => Case::YELLOW
        }
    }
}

impl Into<Player> for Case {
    fn into(self) -> Player {
        match self {
            Case::RED => Player::RED,
            Case::YELLOW => Player::YELLOW,
            Case::EMPTY => panic!("Cannot convert empty case to player")
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub rows: usize,
    pub columns: usize
}

#[derive(Debug, Clone)]
struct GridColumn{
    cases: Vec<Case>,
    current_case: usize
}

impl Deref for GridColumn {
    type Target = Vec<Case>;

    fn deref(&self) -> &Self::Target {
        &self.cases
    }
}

impl DerefMut for GridColumn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cases
    }
}

#[derive(Debug, Clone)]
pub struct Grid{
    grid: Vec<GridColumn>,
    size: Size
}

impl Grid{
    fn new(size: Size) -> Grid{
        Grid{
            grid: vec![GridColumn {
                cases: vec![Case::EMPTY; size.rows],
                current_case: 1
            }; size.columns],
            size
        }
    }
    pub fn new_default() -> Grid{
        Grid::new(Size{
            rows: GRID_NB_ROWS,
            columns: GRID_NB_COLUMNS
        })
    }
    pub fn get_case(&self, column: usize, row: usize) -> Case{
        if column > self.get_size().columns || row > self.get_size().rows || column < 1 || row < 1 {
            return Case::EMPTY;
        }
        self.grid[column-1][row-1]
    }
    fn set_case(&mut self, column: usize, row: usize, case: Case){
        self.grid[column-1][row-1] = case;
    }
    pub fn get_size(&self) -> Size{
        self.size
    }
    fn get_column(&self, column: usize) -> &GridColumn{
        &self.grid[column-1]
    }
    fn get_column_mut(&mut self, column: usize) -> &mut GridColumn{
        &mut self.grid[column-1]
    }
    pub fn get_current_case(&self, column: usize) -> usize{
        self.get_column(column).current_case
    }
    pub fn columns_i(&self) -> RangeInclusive<usize>{
        1..=self.get_size().columns
    }
    pub fn rows_j(&self) -> RangeInclusive<usize>{
        1..=self.get_size().rows
    }
    pub fn is_column_full(&self, column: usize) -> bool{
        self.get_column(column).current_case > self.get_size().rows
    }
    pub fn is_full(&self) -> bool {
        self.grid.iter().all(|column| column.current_case > self.get_size().rows)
    }
    pub fn play(&self, column: usize, choix: Player) -> Result<Grid, &'static str>{
        if self.is_column_full(column){
            return Err("Column is full");
        }
        let mut grid = self.clone();

        grid.set_case(column, grid.get_current_case(column), choix.into());
        grid.get_column_mut(column).current_case += 1;
        Ok(grid)
    }
    pub fn find_4_aligned(&self) -> Option<Player>{
        //Find 4 same case aligned in a row, column or diagonal
        for column in self.columns_i(){
            for row in self.rows_j(){
                let case = self.get_case(column, row);
                if case == Case::EMPTY{
                    continue;
                }
                
                if case == self.get_case(column, row+1) &&
                    case == self.get_case(column, row+2) &&
                    case == self.get_case(column, row+3){
                    return Some(case.into());
                }

                if case == self.get_case(column+1, row) &&
                    case == self.get_case(column+2, row) &&
                    case == self.get_case(column+3, row){
                    return Some(case.into());
                }

                if case == self.get_case(column+1, row+1) &&
                    case == self.get_case(column+2, row+2) &&
                    case == self.get_case(column+3, row+3){
                    return Some(case.into());
                }

                if case == self.get_case(column-1, row+1) &&
                    case == self.get_case(column-2, row+2) &&
                    case == self.get_case(column-3, row+3){
                    return Some(case.into());
                }
            }
        }
        None
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        for row in self.rows_j().rev(){
            for column in self.columns_i(){
                match self.get_case(column, row){
                    Case::RED => write!(f, "|{}", "🔴 ".blue())?,
                    Case::YELLOW => write!(f, "|{}", "🟡 ".yellow())?,
                    Case::EMPTY => write!(f, "|{}", "⚪ ")?
                };
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}