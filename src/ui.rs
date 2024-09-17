use std::path::PathBuf;

use ncurses::*;

use crate::pos::Pos;

#[derive(Debug)]
pub struct MagWindow {
    pub path: PathBuf,
    pub win: WINDOW,
    pub dimensions: Pos<i32>,
    pub idx: usize,
    pub coord: Pos<i32>,
    pub my_pos: Pos<i32>,
}
