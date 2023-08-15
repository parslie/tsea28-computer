mod widget;

use std::{error::Error, time::Duration};

use crossterm::event::{self, Event, KeyModifiers, KeyCode};
use ratatui::{prelude::*, widgets::Paragraph};

#[derive(PartialEq)]
enum State {
    Idling,
    Exiting,
}

fn update(event: Event, state: &mut State) {
    if let Event::Key(key) = event {
        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
            *state = State::Exiting;
        }
    }
}

fn render<B: Backend>(frame: &mut Frame<B>) {
    let hello_world = Paragraph::new("Hello, world!\nPress CTRL+C to exit...");
    frame.render_widget(hello_world, frame.size());
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let poll_duration = Duration::from_millis(500);
    let mut state = State::Idling;

    while state != State::Exiting {
        terminal.draw(|frame| render(frame))?;
        if event::poll(poll_duration)? {
            update(event::read()?, &mut state);
        }
    } 
    
    Ok(())
}
