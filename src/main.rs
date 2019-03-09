use neovim_lib::{Neovim, NeovimApi, Session};
use neovim_lib::{Handler, Value};
use neovim_lib::neovim_api::Buffer;
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

fn attach_to_nvim(nvim: &mut Neovim) -> Buffer {
    //https://github.com/boxofrox/neovim-scorched-earth/blob/master/src/main.rs
    nvim.command("echom \"rust client connected to neovim\"").unwrap();
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

fn send_events_to_buffer(nvim: &mut Neovim, buffer: &Buffer) {
    buffer
        .set_lines(
            nvim,
            0,
            1,
            true,
            vec![String::from("foo"), String::from("hey rust")],
            )
        .unwrap();
}

#[derive(Debug)]
struct BufLineEvent {
    tick: u64,
    start: u64,
    end: u64,
    data: Vec<String>,
}

fn parse_nvim_buf_lines_event(v: &Vec<Value>) -> Result<BufLineEvent, &'static str> {
    match v.as_slice() {
        [_, ticker, line_start, line_end, lines, _] => {
            match (ticker.as_u64(),
                   line_start.as_u64(),
                   line_end.as_u64(),
                   lines.as_array()) {
                (Some(t),Some(s), Some(e), Some(data)) => {
                   let data_parsed : Vec<String> =
                        data.iter().map(|s| String::from(s.as_str().unwrap())).collect();
                    Ok(BufLineEvent {
                        tick: t,
                        start: s,
                        end: e,
                        data: data_parsed.clone(),
                    })
                }
                _ => return Err("ticker did not parse"),
            }
        },
        _ => return Err("bar"),
    }
}

fn notify_blocking_example() {
    let mut session = Session::new_tcp("127.0.0.1:6666").unwrap();
    let _writer = session.start_event_loop_channel();
    let mut nvim = Neovim::new(session);
    let read_user_buf = attach_to_nvim(&mut nvim);

    let mut session2 = Session::new_tcp("127.0.0.1:6667").unwrap();
    let write_user = session2.start_event_loop_channel();
    let mut nvim2 = Neovim::new(session2);
    attach_to_nvim(&mut nvim2);

    println!("Starting blocking event loop");
    loop {
        match write_user.recv() {
            Ok((event_type, v)) => {
                match parse_nvim_buf_lines_event(&v) {
                    Ok(b) => { println!("Got event: {:?}", b); }
                    Err(s) => { println!("Error: {:?}", s); }
                }
                send_events_to_buffer(&mut nvim, &read_user_buf);
            }
            Err(e) => println!("Got an error!: {:?}", e),
        }
    }
}

fn main() {
    notify_blocking_example()
}
