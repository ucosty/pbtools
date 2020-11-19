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

use std::str;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;

fn get_atom(conn: &impl Connection, name: &str) -> u32 {
    return conn.intern_atom(false, name.as_bytes()).unwrap().reply().unwrap().atom;
}

fn get_clipboard_value(conn: &impl Connection, window_id: u32, selection: u32, prop_id: u32, format_name: &str) -> String {
    let format = get_atom(conn, format_name);    
    conn.convert_selection(window_id, selection, format, prop_id, Time::CurrentTime).unwrap();
    conn.flush().unwrap();

    loop {
        let event = conn.wait_for_event().unwrap();
        match event {
            Event::SelectionNotify(_) => {
                break;
            }
            Event::Error(err) => println!("Got an unexpected error: {:?}", err),
            _ => {}
        }
    }

    let reply = conn
        .get_property(false, window_id, prop_id, format, 0, std::u32::MAX)
        .unwrap();

    let reply = reply.reply().unwrap();
    
    return String::from_utf8(reply.value).unwrap()
}

fn main() {
    let (conn, screen_num) = x11rb::connect(None)
        .expect("Failed to connect to the X11 server");

    let selection = get_atom(&conn, "CLIPBOARD");
    let prop_id = get_atom(&conn, "PBCOPY");
    
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
    
    let mut response = get_clipboard_value(&conn, window_id, selection, prop_id, "UTF8_STRING");
    if response.is_empty() {
        response = get_clipboard_value(&conn, window_id, selection, prop_id, "STRING");
    }

    println!("{}", response);
}
