mod power_socket;

pub use power_socket::PowerSocket;

pub extern "C" fn create_power_socket() -> PowerSocket {
    PowerSocket::new()
}

// pub extern "C" fn new() -> PowerSocket {
//     Default::default()
// }
