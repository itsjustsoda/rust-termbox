#![desc = "A Rust wrapper for the termbox library"]
#![license = "MIT"]
#![crate_id = "termbox#0.1.0"]
#![crate_type = "lib" ]

#![feature(globs)]
#![feature(phase)]

#[phase(syntax, link)] extern crate log;
extern crate libc;

use std::task;

pub use libc::types::os::arch::c95::{c_int, c_uint};

/*
 *
 * A lightweight curses alternative wrapping the termbox library.
 *
 * # SYNOPSIS
 *
 * A hello world for the terminal:
 *
 *     use std;
 *     use termbox;
 *
 *     import tb = termbox;
 *
 *     fn main() {
 *         tb::init();
 *         tb::print(1, 1, tb::bold, tb::white, tb::black, "Hello, world!");
 *         tb::present();
 *         std::timer::sleep(std::uv_global_loop::get(), 1000);
 *         tb::shutdown();
 *     }
 *
 * # DESCRIPTION
 *
 * Output is double-buffered.
 *
 * TODO
 *
 * # EXAMPLES
 *
 * TODO
 *
 */

// Exported functions
// export init, shutdown
//      , width, height
//      , clear, present
//      , set_cursor
//      , print, print_ch
//      , poll_event, peek_event
//      , event;

// Exported types
// export color, style
//      , event;



/*
 * The event type matches struct tb_event from termbox.h
 */
pub struct raw_event {
    etype: u8,
    emod: u8,
    key: u16,
    ch: u32,
    w: i32,
    h: i32,
}



/*
 * Foreign functions from termbox.
 */
mod c {
    use libc::types::os::arch::c95::{ c_int, c_uint};

		#[link(name = "termbox")]
    extern {

        pub fn tb_init() -> c_int;
        pub fn tb_shutdown();

        pub fn tb_width() -> c_uint;
        pub fn tb_height() -> c_uint;

        pub fn tb_clear();
        pub fn tb_present();

        pub fn tb_set_cursor(cx: c_int, cy: c_int);

        pub fn tb_change_cell(x: c_uint, y: c_uint, ch: u32, fg: u16, bg: u16);

        pub fn tb_select_input_mode(mode: c_int) -> c_int;
        pub fn tb_set_clear_attributes(fg: u16, bg: u16);

        pub fn tb_peek_event(ev: *::raw_event, timeout: c_uint) -> c_int;
        pub fn tb_poll_event(ev: *::raw_event) -> c_int;
    }
}

fn test() {

}


pub fn init() -> int {
    unsafe { c::tb_init() as int }
}


pub fn shutdown() {
    unsafe { c::tb_shutdown(); }
}


pub fn width() -> uint {
    unsafe {
        return  c::tb_width() as uint;
    }
}


pub fn height() -> uint {
    unsafe {
        return  c::tb_height() as uint;
    }
}

/**
 * Clear buffer.
 */

pub fn clear() {
    unsafe {
        c::tb_clear();
    }
}

// /**
//  * Write buffer to terminal.
//  */

pub fn present() {
    unsafe {
        c::tb_present();
    }
}


pub fn set_cursor(cx: int, cy: int) {
    unsafe {
        c::tb_set_cursor(cx as c_int, cy as c_int);
    }
}


// low-level wrapper
pub fn change_cell(x: uint, y: uint, ch: u32, fg: u16, bg: u16) {
    unsafe {
        c::tb_change_cell(x as c_uint, y as c_uint, ch, fg, bg);
    }
}

/// Convert from enums to u16
pub fn convert_color(c: color) -> u16 {
    match c {
        black   => 0x00,
        red     => 0x01,
        green   => 0x02,
        yellow  => 0x03,
        blue    => 0x04,
        magenta => 0x05,
        cyan    => 0x06,
        white   => 0x07,
    }
}

pub fn convert_style(sty: style) -> u16 {
    match sty {
        normal         => 0x00,
        bold           => 0x10,
        underline      => 0x20,
        bold_underline => 0x30,
    }
}

/**
 * Print a string to the buffer.  Leftmost charater is at (x, y).
 */

pub fn print(x: uint, y: uint, sty: style, fg: color, bg: color, s: ~str) {
    let fg: u16 = convert_color(fg) | convert_style(sty);
    let bg: u16 = convert_color(bg);
    for (i, ch) in s.chars().enumerate() {
        unsafe {
            c::tb_change_cell((x + i) as c_uint, y as c_uint, ch as u32, fg, bg);
        }
    }
}

// /**
//  * Print a charater to the buffer.
//  */

pub fn print_ch(x: uint, y: uint, sty: style, fg: color, bg: color, ch: char) {
    unsafe {
        let fg: u16 = convert_color(fg) | convert_style(sty);
        let bg: u16 = convert_color(bg);
        c::tb_change_cell(x as c_uint, y as c_uint, ch as u32, fg, bg);
    }
}

pub enum color {
    black,
    red,
    green,
    yellow,
    blue,
    magenta,
    cyan,
    white
}

pub enum style {
    normal,
    bold,
    underline,
    bold_underline
}

//Convenience functions
pub fn with_term(f: proc():Send) {
    init();
    let res = task::try(f);
    shutdown();
    match res {
        Err(_) => {
            error!("with_term: An error occured.");
        }
        _ => {}
    }
}

pub fn nil_raw_event() -> raw_event {
    raw_event{etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0}
}

enum event {
    key_event(u8, u16, u32),
    resize_event(i32, i32),
    no_event
}

/**
 * Get an event if within timeout milliseconds, otherwise return urn no_event.
 */


pub fn peek_event(timeout: uint) -> event {
    unsafe {
        let ev = nil_raw_event();
        let rc = c::tb_peek_event(&ev, timeout as c_uint);
        return unpack_event(rc, &ev);
    }
}

// /**
//  * Blocking function to return urn next event.
//  */

pub fn poll_event() -> event {
    unsafe {
        let ev = nil_raw_event();
        let rc = c::tb_poll_event(&ev);
        return unpack_event(rc, &ev);
    }
}

// /* helper pub fn
//  *
//  * ev_type
//  *   0 -> no event
//  *   1 -> key
//  *   2 -> resize
//  *   -1 -> error
//  */
pub fn unpack_event(ev_type: c_int, ev: &raw_event) -> event {
    match ev_type {
        0 => no_event,
        1 => {
            return key_event(ev.emod, ev.key, ev.ch);
        },
        2 => {
            return resize_event(ev.w, ev.h);
        },
        _ => { fail!("asdf"); }
    }
}
