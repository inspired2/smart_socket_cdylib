mod power_socket;

use power_socket::{PowerSocket, PowerSocketState};

#[no_mangle]
pub extern "C" fn create_power_socket() -> PowerSocket {
    PowerSocket::new()
}

#[no_mangle]
pub extern "C" fn turn_off(socket: &mut PowerSocket) {
    socket.turn_off()
}

#[no_mangle]
pub extern "C" fn turn_on(socket: &mut PowerSocket) {
    socket.turn_on()
}

#[no_mangle]
pub extern "C" fn get_state(socket: &PowerSocket) -> PowerSocketState {
    socket.get_state()
}