use neovim_lib::{Neovim, NeovimApi, Session};
use neovim_lib::{Handler, Value};
use std::sync::mpsc;

#[allow(dead_code)]
fn quit_example() {
    let mut session = Session::new_tcp("127.0.0.1:6666").unwrap();
    session.start_event_loop();
    let mut nvim = Neovim::new(session);
    match nvim.quit_no_save() {
        Ok(()) => println!("quit success!"),
        Err(msg) => println!("error! {:?}", msg)
    }
}

fn attach_to_nvim(nvim: &mut Neovim) -> (neovim_lib::neovim_api::Buffer) {
    //https://github.com/boxofrox/neovim-scorched-earth/blob/master/src/main.rs
    nvim.command("echom \"rust client connected to neovim\"").unwrap();
    nvim.subscribe("cursor-moved-i").expect("error: cannot subscribe to event: change-cursor-i");
    nvim.subscribe("insert-enter").expect("error: cannot subscribe to event: insert-enter");
    nvim.subscribe("insert-leave").expect("error: cannot subscribe to event: insert-leave");
    nvim.subscribe("quit").expect("error: cannot subscribe to event: quit");


    let buffer = nvim.get_current_buf().unwrap();
    buffer
        .set_lines(
            nvim,
            0,
            1,
            true,
            vec![String::from("foo"), String::from("hey rust")],
        )
        .unwrap();

    buffer.attach(nvim, true, Vec::new()).unwrap();
    return buffer;
}

fn notify_blocking_example() {
    let mut session = Session::new_tcp("127.0.0.1:6666").unwrap();
    let writer = session.start_event_loop_channel();
    let mut nvim = Neovim::new(session);
    attach_to_nvim(&mut nvim);

    let mut session2 = Session::new_tcp("127.0.0.1:6667").unwrap();
    let writer = session2.start_event_loop_channel();
    let mut nvim2 = Neovim::new(session2);
    attach_to_nvim(&mut nvim);
    let read_buf = attach_to_nvim(&mut nvim2);

    println!("Starting blocking event loop");
    loop {
        match writer.recv() {
            Ok(s) => {
                println!("We did it! Got {:?} Setting linesin read buf", s);
                read_buf
                    .set_lines(
                        &mut nvim,
                        0,
                        1,
                        true,
                        vec![String::from("foo"), String::from("hey rust")],
                    )
                    .unwrap();
            }
            Err(e) => println!("Got an error!: {:?}", e),
        }
    }
    println!("broke from recv");
}

fn main() {
    notify_blocking_example()
}
