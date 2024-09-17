use std::{
    env,
    path::{Path, PathBuf},
    sync::mpsc,
    thread::{sleep, spawn},
    time::Duration,
};

use crate::{pos::Pos, ui::MagWindow};
use ncurses::*;

const W_RIGHT: f32 = 0.2;
const W_MIDDLE: f32 = 0.4;
const W_LEFT: f32 = 0.4;
const START_TOP: i32 = 1;

#[derive(Debug)]
pub struct State {
    pub child_win: MagWindow,
    pub mid_win: MagWindow,
    pub parent_win: MagWindow,
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
        let h = getmaxy(stdscr()) - 3;

        let w_left = (w as f32 * W_RIGHT) as i32;
        let w_middle = (w as f32 * W_MIDDLE) as i32;
        let w_right = (w as f32 * W_LEFT) as i32;

        let dim = Pos::new(h, w);

        let parent_win = MagWindow::new(
            &path,
            Pos::new(1, START_TOP),
            Pos::new(w_right, h - START_TOP),
        )
        .fetch_return();

        let child_win = MagWindow::new(
            &path,
            Pos::new(1, START_TOP),
            Pos::new(w_left, h - START_TOP),
        )
        .fetch_return();

        let mid_win = MagWindow::new(
            &path,
            Pos::new(1, START_TOP),
            Pos::new(w_middle, h - START_TOP),
        )
        .fetch_return();

        Ok(Self {
            parent_win,
            child_win,
            mid_win,
            path,
            dim,
        })
    }

    pub fn update(&mut self) -> std::io::Result<&mut Self> {
        let (tx, rx) = mpsc::channel();
        let mut count = 0;

        spawn(move || loop {
            sleep(Duration::from_secs(1));
            tx.send(count).unwrap();
            count += 1;
        });

        nodelay(stdscr(), true);

        let mut ch = getch();
        self.display();
        while ch != 113 {
            // Si no hay teclas presionadas, `getch()` devolverá ERR (-1)
            if let Ok(value) = rx.try_recv() {
                if value > 100 {
                    return Ok(self);
                }
            }

            match ch {
                _ => {}
            }

            ch = getch();
            sleep(Duration::from_millis(100));
        }

        Ok(self)
    }

    fn display(&mut self) {
        self.resize();
        //box_(self.child_win.win, 0, 0);
        //box_(self.parent_win.win, 0, 0);
        //box_(self.mid_win.win, 0, 0);
        // clear();
        refresh();
        mvwprintw(stdscr(), 0, 1, &self.parent_win.path.to_string_lossy());
        self.mid_win.display();
        self.parent_win.display();
        self.child_win.display();
        refresh();
    }

    fn resize(&mut self) {
        let w = getmaxx(stdscr());
        let h = getmaxy(stdscr());

        if w != self.dim.x || h != self.dim.y {
            let w_right = (w as f32 * W_RIGHT) as i32;
            let w_middle = (w as f32 * W_MIDDLE) as i32;
            let w_left = (w as f32 * W_LEFT) as i32;

            self.parent_win
                .change_dim(Pos::new(1, START_TOP), Pos::new(w_right, h - START_TOP));
            self.mid_win.change_dim(
                Pos::new(1 + w_right, START_TOP),
                Pos::new(w_middle, h - START_TOP),
            );
            self.child_win.change_dim(
                Pos::new(1 + w_right + w_middle, START_TOP),
                Pos::new(w_left, h - START_TOP),
            );
            clear();
            refresh();
            wrefresh(self.mid_win.win);
        }
    }

    pub fn exit(&mut self) {
        delwin(self.child_win.win);
        delwin(self.mid_win.win);
        delwin(self.parent_win.win);
        endwin();
    }
}
