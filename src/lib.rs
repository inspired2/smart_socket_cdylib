mod power_socket;

use power_socket::{PowerSocket, PowerSocketState};

type New = extern "C" fn() -> SocketHandle;
type TurnOn = extern "C" fn(*mut PowerSocket);
type TurnOff = extern "C" fn(*mut PowerSocket);
type State = extern "C" fn(*const PowerSocket) -> StateHandle;

#[repr(C)]
pub struct Functions {
    new: New,
    turn_off: TurnOff,
    turn_on: TurnOn,
    status: State,
}

impl Default for Functions {
    fn default() -> Self {
        Self {
            turn_off,
            turn_on,
            status: get_state,
            new,
        }
    }
}
#[repr(transparent)]
pub struct SocketHandle(*mut PowerSocket);

#[repr(transparent)]
pub struct StateHandle(*mut PowerSocketState);

#[no_mangle]
pub extern "C" fn new() -> SocketHandle {
    SocketHandle(Box::into_raw(Box::new(PowerSocket::new())))
}

#[no_mangle]
pub extern "C" fn turn_off(socket: *mut PowerSocket) {
    unsafe { (*socket).turn_off() }
}

#[no_mangle]
pub extern "C" fn turn_on(socket: *mut PowerSocket) {
    unsafe { (*socket).turn_on() }
}

#[no_mangle]
pub extern "C" fn get_state(socket: *const PowerSocket) -> StateHandle {
    let state = unsafe { (*socket).get_state() };
    StateHandle(Box::into_raw(Box::new(state)))
}
