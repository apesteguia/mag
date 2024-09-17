use std::{
    path::{Path, PathBuf},
    usize,
};

use ncurses::*;

use crate::{
    filesys::{MagEntry, MagFile, MagFolder},
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
    pub dir: MagEntry,
}

impl MagWindow {
    pub fn new<P: AsRef<Path>>(path: P, coord: Pos<i32>, dimensions: Pos<i32>) -> Self {
        let path = path.as_ref().to_owned();
        let idx: usize = 0;
        let my_pos = Pos::new(0, 0);

        let win = newwin(dimensions.y, dimensions.x, coord.y, coord.x);
        let dir = MagEntry::Dir(MagFolder::new(&path));

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

    pub fn new_file<P: AsRef<Path>>(path: P, coord: Pos<i32>, dimensions: Pos<i32>) -> Self {
        let path = path.as_ref().to_owned();
        let idx: usize = 0;
        let my_pos = Pos::new(0, 0);

        let win = newwin(dimensions.y, dimensions.x, coord.y, coord.x);
        let dir = MagEntry::File(MagFile::new(&path));

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

    // TODO: VERY VERBOSE FUNCION
    pub fn display(&self) {
        match &self.dir {
            MagEntry::Dir(d) => {
                for (c, i) in d.items.iter().enumerate() {
                    if c as i32 > self.dimensions.y - 5 {
                        break;
                    }

                    if c == self.idx {
                        // Activar formato bold
                        wattron(self.win, COLOR_PAIR(5) | A_BOLD());
                        match i {
                            MagEntry::File(f) => {
                                mvwprintw(self.win, c as i32 + 1, 2, f.data.path.to_str().unwrap());
                            }
                            MagEntry::Dir(f) => {
                                mvwprintw(self.win, c as i32 + 1, 2, f.data.path.to_str().unwrap());
                            }
                        }
                        wattroff(self.win, COLOR_PAIR(5) | A_BOLD());
                    } else {
                        match i {
                            MagEntry::File(f) => {
                                mvwprintw(self.win, c as i32 + 1, 2, f.data.path.to_str().unwrap());
                            }
                            MagEntry::Dir(f) => {
                                wattron(self.win, COLOR_PAIR(4));
                                mvwprintw(self.win, c as i32 + 1, 2, f.data.path.to_str().unwrap());
                                wattroff(self.win, COLOR_PAIR(4));
                            }
                        }
                    }
                }
            }
            MagEntry::File(f) => {
                mvwprintw(self.win, 1, 1, "sdfasdf");
                if f.content.is_empty() {
                    mvwprintw(self.win, 1, 1, "Empty File");
                } else {
                    let v: Vec<&str> = f.content.split('\n').collect();
                    for (i, st) in v.iter().enumerate() {
                        mvwprintw(self.win, i as i32 + 2, 1, st);
                    }
                }
            }
        }
        wrefresh(self.win);
    }

    pub fn change_dir<P: AsRef<Path>>(&mut self, path: P, is_folder: bool) {
        let path = path.as_ref().to_owned();

        match is_folder {
            false => self.dir = MagEntry::File(MagFile::new_return(&path)),
            true => self.dir = MagEntry::Dir(MagFolder::new(&path).get_entries_return().unwrap()),
        }
    }

    pub fn fetch_return(self) -> Self {
        match self.dir {
            MagEntry::File(f) => {
                let mut updated_file = MagFile::new(&f.data.path);
                updated_file.file_contents();
                Self {
                    dir: MagEntry::File(updated_file),
                    ..self
                }
            }
            MagEntry::Dir(d) => {
                let mut updated_folder = d.clone();
                updated_folder.get_entries();

                Self {
                    dir: MagEntry::Dir(updated_folder),
                    ..self
                }
            }
        }
    }

    pub fn fetch(&mut self) {
        match &self.dir {
            MagEntry::File(f) => self.dir = MagEntry::File(MagFile::new(&f.data.path)),
            MagEntry::Dir(d) => {
                self.dir = MagEntry::Dir(MagFolder::new(&d.data.path).get_entries_return().unwrap())
            }
        }
    }

    pub fn change_dim(&mut self, coord: Pos<i32>, dim: Pos<i32>) {
        self.dimensions = dim;
        self.coord = coord;
        self.win = newwin(dim.y, dim.x, coord.y, coord.x);
    }
}
