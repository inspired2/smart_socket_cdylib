use std::ffi::CString;

#[repr(C)]
pub struct PowerSocket {
    name: CString,
    state: PowerSocketState,
    power_consumption: usize,
}
impl Default for PowerSocket {
    fn default() -> Self {
        Self {
            name: Default::default(),
            state: Default::default(),
            power_consumption: 0,
        }
    }
}

impl PowerSocket {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn turn_on(&mut self) {
        self.state = PowerSocketState::Powered;
        self.power_consumption = 220;
    }
    pub fn turn_off(&mut self) {
        self.state = PowerSocketState::NotPowered;
        self.power_consumption = 0;
    }
    pub fn get_state(&self) -> PowerSocketState {
        self.state
    }
    pub fn get_voltage(&self) -> usize {
        self.power_consumption
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum PowerSocketState {
    Powered = 1,
    NotPowered = 0,
}
impl Default for PowerSocketState {
    fn default() -> Self {
        Self::NotPowered
    }
}
