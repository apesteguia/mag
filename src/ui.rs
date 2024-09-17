use std::{
    path::{Path, PathBuf},
    usize,
};

use ncurses::*;

use crate::{
    filesys::{MagEntry, MagFolder},
    pos::Pos,
};

#[derive(Debug)]
pub struct MagWindow {
    pub path: PathBuf,
    pub win: WINDOW,
    pub dimensions: Pos<i32>,
    pub idx: usize,
    pub coord: Pos<i32>,
    pub my_pos: Pos<i32>,
    pub dir: MagFolder,
}

impl MagWindow {
    pub fn new<P: AsRef<Path>>(path: P, coord: Pos<i32>, dimensions: Pos<i32>) -> Self {
        let path = path.as_ref().to_owned();
        let idx: usize = 0;
        let my_pos = Pos::new(0, 0);

        let win = newwin(dimensions.y, dimensions.x, coord.y, coord.x);
        let dir = MagFolder::new(&path);

        Self {
            path,
            idx,
            coord,
            dimensions,
            dir,
            my_pos,
            win,
        }
    }

    //Debug display
    pub fn display(&self) {
        let mut c = 0;
        let v = self.dir.get_entries().unwrap();
        for i in v {
            match i {
                MagEntry::File(f) => mvwprintw(self.win, c, 2, f.data.path.to_str().unwrap()),
                MagEntry::Dir(f) => mvwprintw(self.win, c, 2, f.data.path.to_str().unwrap()),
            };
            c += 1;
        }
        wrefresh(self.win);
    }

    pub fn change_dim(&mut self, coord: Pos<i32>, dim: Pos<i32>) {
        self.dimensions = dim;
        self.coord = coord;
        self.win = newwin(dim.y, dim.x, coord.y, coord.x);
    }
}
