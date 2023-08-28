mod computer;
mod update;
mod render;

use self::computer::Computer;

#[derive(PartialEq)]
enum State {
    Idling,
    Exiting,
}

pub struct App {
    state: State,
    computer: Computer,

    pm_offset: usize,
    pm_index: usize,

    k1_offset: usize,
    k1_index: usize,

    k2_offset: usize,
    k2_index: usize,

    um_offset: usize,
    um_index: usize,

    grx_offset: usize,
    grx_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::Idling,
            computer: Computer::new(),

            pm_offset: 0,
            pm_index: 0,
        
            k1_offset: 0,
            k1_index: 0,
        
            k2_offset: 0,
            k2_index: 0,
        
            um_offset: 0,
            um_index: 0,
        
            grx_offset: 0,
            grx_index: 0,
        }
    }

    pub fn is_exiting(&self) -> bool {
        self.state == State::Exiting
    }
}