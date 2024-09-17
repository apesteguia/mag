use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{pos::Pos, ui::MagWindow};
use ncurses::*;

const W_RIGHT: f32 = 0.2;
const W_MIDDLE: f32 = 0.4;
const W_LEFT: f32 = 0.4;
const START_TOP: i32 = 1;

#[derive(Debug)]
pub struct State {
    pub left_win: MagWindow,
    pub mid_win: MagWindow,
    pub right_win: MagWindow,
    pub path: PathBuf,
    pub dim: Pos<i32>,
}

impl State {
    pub fn new<P: AsRef<Path>>(p: P) -> std::io::Result<Self> {
        let untested_path = p.as_ref().to_owned();
        let path = match untested_path.is_dir() {
            true => untested_path,
            false => env::current_dir()?,
        };

        initscr();
        noecho();
        raw();
        cbreak();
        refresh();
        start_color();
        init_pair(1, COLOR_WHITE, COLOR_BLACK);
        init_pair(2, COLOR_WHITE, COLOR_BLUE);
        init_pair(3, COLOR_BLUE, COLOR_BLACK);
        init_pair(4, COLOR_BLACK, COLOR_WHITE);
        init_pair(5, COLOR_RED, COLOR_WHITE);

        let w = getmaxx(stdscr());
        let mut h = getmaxy(stdscr());
        h -= 3;

        let w_right = (w as f32 * W_RIGHT) as i32;
        let w_middle = (w as f32 * W_MIDDLE) as i32;
        let w_left = (w as f32 * W_LEFT) as i32;

        let dim = Pos::new(h, w);

        let left_win = MagWindow::new(
            &path,
            Pos::new(1, START_TOP),
            Pos::new(w_middle, h - START_TOP),
        );

        let right_win = MagWindow::new(
            &path,
            Pos::new(1, START_TOP),
            Pos::new(w_middle, h - START_TOP),
        );
        let mid_win = MagWindow::new(
            &path,
            Pos::new(1, START_TOP),
            Pos::new(w_middle, h - START_TOP),
        );

        Ok(Self {
            left_win,
            right_win,
            mid_win,
            path,
            dim,
        })
    }

    pub fn update(&mut self) -> std::io::Result<&mut Self> {
        self.display();
        let mut ch = getch();
        while ch != 113 {
            match ch {
                _ => {}
            }
            //self.display();
            ch = getch();
        }

        Ok(self)
    }

    fn display(&mut self) {
        self.resize();
        clear();
        refresh();
        mvwprintw(stdscr(), 0, 1, &self.right_win.path.to_string_lossy());
        //self.mid_win.display();
    }

    fn resize(&mut self) {
        let w = getmaxx(stdscr());
        let h = getmaxy(stdscr());

        if w != self.dim.x || h != self.dim.y {
            let w_right = (w as f32 * W_RIGHT) as i32;
            let w_middle = (w as f32 * W_MIDDLE) as i32;
            let w_left = (w as f32 * W_LEFT) as i32;

            self.right_win
                .change_dim(Pos::new(1, START_TOP), Pos::new(w_right, h - START_TOP));
            self.mid_win.change_dim(
                Pos::new(1 + w_right, START_TOP),
                Pos::new(w_middle, h - START_TOP),
            );
            self.left_win.change_dim(
                Pos::new(1 + w_right + w_middle, START_TOP),
                Pos::new(w_left, h - START_TOP),
            );
            clear();
            refresh();
        }
    }

    pub fn exit(&mut self) {
        delwin(self.left_win.win);
        delwin(self.mid_win.win);
        delwin(self.right_win.win);
        endwin();
    }
}
