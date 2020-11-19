/*
 * This file is part of the pbtools distribution (https://github.com/ucosty/pbtools).
 * Copyright (c) 2020 Matthew Costa.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */
extern crate x11rb;

use std::io::{self, Read};
use std::str;

use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::protocol::xproto::*;
use x11rb::wrapper::ConnectionExt as _;

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer)?;
    }
    Ok(buffer)
}

fn get_atom(conn: &impl Connection, name: &str) -> u32 {
    return conn.intern_atom(false, name.as_bytes()).unwrap().reply().unwrap().atom;
}

fn set_clipboard_value(conn: &impl Connection, window_id: u32, selection: u32, value: &str) {
    let format = get_atom(conn, "UTF8_STRING");
    let string_format = get_atom(conn, "STRING");
    let targets = get_atom(conn, "TARGETS");

    conn.set_selection_owner(window_id, selection, Time::CurrentTime).unwrap();
    conn.flush().unwrap();

    let mut done = false;
    while !done {
        let event = conn.wait_for_event().unwrap();
        match event {
            Event::SelectionRequest(req) => {
                let evt = SelectionNotifyEvent {
                    response_type: 31,
                    property: req.property,
                    requestor: req.requestor,
                    selection: req.selection,
                    target: req.target,
                    sequence: req.sequence,
                    time: req.time,
                };

                if req.target == targets {
                    let formats: [u32; 3] = [targets, string_format, format];
                    conn.change_property32(PropMode::Replace, req.requestor, req.property, targets, &formats).unwrap();
                } else if req.target == format || req.target == string_format {
                    conn.change_property8(PropMode::Replace, req.requestor, req.property, format, value.as_bytes()).unwrap();
                    done = true;
                }

                conn.send_event(true, req.requestor, EventMask::NoEvent, &evt).unwrap();
                conn.flush().unwrap();
            }
            Event::Error(err) => println!("Got an unexpected error: {:?}", err),
            _ => {}
        }
    }
}

fn main() {
    let buffer: String = read_stdin().expect("Could not read stdin");
    let (conn, screen_num) = x11rb::connect(None)
        .expect("Failed to connect to the X11 server");

    let selection = get_atom(&conn, "CLIPBOARD");

    let window_id = conn.generate_id().unwrap();
    let screen = &conn.setup().roots[screen_num];
    let win_aux = CreateWindowAux::new();

    CreateWindowRequest {
        depth: screen.root_depth,
        wid: window_id,
        parent: screen.root,
        x: 0,
        y: 0,
        width: 1,
        height: 1,
        border_width: 0,
        class: WindowClass::InputOutput,
        visual: 0,
        value_list: std::borrow::Cow::Borrowed(&win_aux),
    }.send(&conn).unwrap();

    set_clipboard_value(&conn, window_id, selection, &buffer);
}
