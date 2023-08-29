mod computer;
mod update;
mod render;

use self::computer::Computer;

#[derive(PartialEq)]
enum State {
    Idling,
    Exiting,
}

#[derive(PartialEq)]
enum Component {
    ASR,
    PC,
    PM,
    AR,
    HR,
    GRX,
    FLAGS,
    LC,
    IR,
    UPC,
    SUPC,
    K1,
    K2,
    UM,
}

pub struct App {
    state: State,
    computer: Computer,

    pm_offset: usize,
    pm_idx: usize,

    grx_offset: usize,
    grx_idx: usize,

    k1_offset: usize,
    k1_idx: usize,

    k2_offset: usize,
    k2_idx: usize,

    um_offset: usize,
    um_idx: usize,

    selected_comp: Component,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::Idling,
            computer: Computer::new(),

            pm_offset: 0,
            pm_idx: 0,
        
            grx_offset: 0,
            grx_idx: 0,
        
            k1_offset: 0,
            k1_idx: 0,
        
            k2_offset: 0,
            k2_idx: 0,
        
            um_offset: 0,
            um_idx: 0,

            selected_comp: Component::ASR,
        }
    }

    pub fn is_exiting(&self) -> bool {
        self.state == State::Exiting
    }

    fn next_component(&mut self) {
        self.selected_comp = match self.selected_comp {
            Component::ASR => Component::PC,
            Component::PC => Component::PM,
            Component::PM => Component::AR,
            Component::AR => Component::HR,
            Component::HR => Component::GRX,
            Component::GRX => Component::FLAGS,
            Component::FLAGS => Component::LC,
            Component::LC => Component::IR,
            Component::IR => Component::UPC,
            Component::UPC => Component::SUPC,
            Component::SUPC => Component::K1,
            Component::K1 => Component::K2,
            Component::K2 => Component::UM,
            Component::UM => Component::ASR,
        }
    }
}