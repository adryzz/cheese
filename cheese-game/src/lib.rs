use std::{ops::Index, fmt::Display};

mod utils;
pub mod format_utils;
pub mod cells;
pub mod game;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    owner: Player,
    piece: PieceType,
}

impl Piece {
    pub const fn new(owner: Player, piece: PieceType) -> Self {
        Self { owner, piece }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Board([Row; 8]);

pub const EMPTY_BOARD: Board = Board([EMPTY_ROW; 8]);

impl Index<usize> for Board {
    type Output = Row;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<Cell> for Board {
    type Output = Option<Piece>;

    fn index(&self, index: Cell) -> &Self::Output {
        let (row, col) = index.to_row_col();
        &self[row][col]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Row([Option<Piece>; 8]);

pub const EMPTY_ROW: Row = Row([None; 8]);

impl Index<usize> for Row {
    type Output = Option<Piece>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Board {
    pub fn new() -> Self {
        utils::init_board()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cell(usize);

impl Cell {
    pub fn to_row_col(&self) -> (usize, usize) {
        (self.0 / 8, self.0 % 8)
    }

    pub fn from_row_col(row: usize, col: usize) -> Self {
        Self((row * 8) + col)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (row, col) = self.to_row_col();
        let rowc = match row {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!()
        };

        write!(f, "{}{}", rowc, col + 1)
    }
}