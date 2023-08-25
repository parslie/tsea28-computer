mod computer;
mod rendering;
mod functionality;

use std::{error::Error, time::Duration};
use crossterm::event;
use ratatui::prelude::*;

use self::{computer::Computer, functionality::handle_event, rendering::render};

#[derive(PartialEq)]
pub enum State { Idling, Quitting }

pub struct Data {
    state: State,
    computer: Computer,
    pm_offset: u8,
    pm_index: u8,
    grx_offset: u8,
    grx_index: u8,
    k1_offset: u8,
    k1_index: u8,
    k2_offset: u8,
    k2_index: u8,
    um_offset: u8,
    um_index: u8,
    selected: u8,
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let poll_duration = Duration::from_millis(250);
    let mut data: Data = Data {
        state: State::Idling,
        computer: Computer::new(),
        pm_offset: 0,
        pm_index: 0,
        grx_offset: 0,
        grx_index: 0,
        k1_offset: 0,
        k1_index: 0,
        k2_offset: 0,
        k2_index: 0,
        um_offset: 0,
        um_index: 0,
        selected: 0,
    };

    while data.state != State::Quitting {
        terminal.draw(|frame| render(frame, &data))?;
        if event::poll(poll_duration)? {
            handle_event(event::read()?, &mut data);
        }
    }

    Ok(())
}