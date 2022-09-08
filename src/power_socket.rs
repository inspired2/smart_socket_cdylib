use std::ffi::{CStr, CString};

#[derive(Debug)]
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
    pub extern "C" fn new() -> Self {
        Self::default()
    }
    pub extern "C" fn turn_on(&mut self) {
        self.state = PowerSocketState::Powered;
        self.power_consumption = 220;
    }
    pub extern "C" fn turn_off(&mut self) {
        self.state = PowerSocketState::NotPowered;
        self.power_consumption = 0;
    }
    pub extern "C" fn get_state(&self) -> PowerSocketState {
        self.state
    }
    pub extern "C" fn get_voltage(&self) -> usize {
        self.power_consumption
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum PowerSocketState {
    Powered = 1,
    NotPowered = 0,
}
impl Default for PowerSocketState {
    fn default() -> Self {
        Self::NotPowered
    }
}
