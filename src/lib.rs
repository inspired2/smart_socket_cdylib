mod power_socket;

use power_socket::{PowerSocket, PowerSocketState};

type New = extern "C" fn() -> *mut PowerSocket;
type TurnOn = extern "C" fn(*mut PowerSocket);
type TurnOff = extern "C" fn(*mut PowerSocket);
type State = extern "C" fn(*const PowerSocket) -> *const PowerSocketState;

#[repr(C)]
pub struct Functions {
    size: usize,
    new: New,
    turn_off: TurnOff,
    turn_on: TurnOn,
    status: State,
}

impl Default for Functions {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<Self>(),
            turn_off,
            turn_on,
            status: get_state,
            new,
        }
    }
}
#[no_mangle]
pub extern "C" fn load_functions() -> Functions {
    Functions::default()
}

extern "C" fn new() -> *mut PowerSocket {
    Box::into_raw(Box::new(PowerSocket::new()))
}

extern "C" fn turn_off(socket: *mut PowerSocket) {
    unsafe { (*socket).turn_off() }
}

extern "C" fn turn_on(socket: *mut PowerSocket) {
    unsafe { (*socket).turn_on() }
}

extern "C" fn get_state(socket: *const PowerSocket) -> *const PowerSocketState {
    let state = unsafe { (*socket).get_state() };
    Box::into_raw(Box::new(state))
}
