use state::State;

pub mod filesys;
pub mod pos;
pub mod state;
pub mod ui;

fn main() {
    let _ = State::new("/home/mikel/").unwrap().update().unwrap().exit();
}
