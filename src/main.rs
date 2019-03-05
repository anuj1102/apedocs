use neovim_lib::{Neovim, NeovimApi, Session};

fn main() {
    let mut session = Session::new_tcp("127.0.0.1:6666").unwrap();
    // let mut session = Session::new_child().unwrap();
    session.start_event_loop();
    let mut nvim = Neovim::new(session);
    match nvim.quit_no_save() {
        Ok(()) => println!("quit success!"),
        Err(msg) => println!("error! {:?}", msg)
    }
}
