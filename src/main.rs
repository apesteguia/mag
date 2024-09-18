use state::State;

pub mod filesys;
pub mod pos;
pub mod state;
pub mod ui;

fn main() {
    /*
        let _ = State::new("/home/mikel/Escritorio/ruby/rb/rb/")
            .unwrap()
            .update()
            .unwrap()
            .exit();
    */
    let _ = State::new("/home/mikel/Escritorio/")
        .expect("ERROR CREATING STATE IN MAIN")
        .update()
        .expect("ERROR IN UPDATE")
        .exit();
}
