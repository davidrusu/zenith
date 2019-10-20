/**
 * Copyright 2019 Benjamin Vaisvil (ben@neuon.com)
 */
extern crate sysinfo;
extern crate hostname;
#[macro_use] extern crate byte_unit;
#[macro_use] extern crate maplit;
extern crate num_traits;
extern crate num;
#[macro_use] extern crate num_derive;

mod util;
mod constants;
mod zprocess;
mod metrics;
mod render;

use crate::constants::*;
use crate::util::*;
use crate::zprocess::*;
use crate::metrics::*;
use crate::render::*;
use std::io;
use std::time::{Duration, SystemTime};
use std::error::{Error};
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use std::fmt::Display;
use tui::backend::TermionBackend;

use tui::Terminal;
use tui::Frame;
use sysinfo::{NetworkExt, System, SystemExt, ProcessorExt, DiskExt, Pid, ProcessExt, Process, ProcessStatus};



use sys_info;

use std::panic::{PanicInfo};
use std::panic;
use std::io::{Write, Stdout};
use sysinfo::Signal::Continue;



fn panic_hook(info: &PanicInfo<'_>) {
	let location = info.location().unwrap();  // The current implementation always returns Some
	let msg = match info.payload().downcast_ref::<&'static str>() {
		Some(s) => *s,
		None => match info.payload().downcast_ref::<String>() {
			Some(s) => &s[..],
			None => "Box<Any>",
		}
	};
	println!("{}thread '<unnamed>' panicked at '{}', {}\r", termion::screen::ToMainScreen, msg, location);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().expect("Could not bind to STDOUT in raw mode.");
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Could not create new terminal.");
    //terminal.what_is_this();
    terminal.hide_cursor().expect("Hiding cursor failed.");

    panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));
    let mut r = TerminalRenderer::new();
    r.start();



    Ok(())
}