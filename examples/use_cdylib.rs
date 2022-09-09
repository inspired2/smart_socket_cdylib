use libloading::{Library, Symbol};

#[derive(Debug)]
#[repr(C)]
struct PowerSocket {
    name: String,
    state: PowerSocketState,
    power_consumption: usize,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
enum PowerSocketState {
    PoweredOn = 1,
    PoweredOff = 0,
}
type CreateSocketFn = unsafe extern "C" fn() -> PowerSocket;
type TurnOn = unsafe extern "C" fn(&mut PowerSocket);
type TurnOff = unsafe extern "C" fn(&mut PowerSocket);
type GetState = unsafe extern "C" fn(&PowerSocket) -> PowerSocketState;

fn main() {
    let filename = "/home/alex/projects/smart_socket_cdylib/target/release/libsmart_socket.so";
    let lib = unsafe { Library::new(filename).expect(
        "Unable to load library at {filename}"
    ) };

    let create_power_socket: Symbol<CreateSocketFn> = unsafe { lib.get(b"create_power_socket").expect("not found") };
    let turn_off: Symbol<TurnOn> = unsafe { lib.get(b"turn_off").unwrap()};
    let turn_on: Symbol<TurnOff> = unsafe { lib.get(b"turn_on").unwrap()};
    let get_state: Symbol<GetState> = unsafe { lib.get(b"get_state").unwrap()};
    
    let mut power_socket = unsafe { create_power_socket() };

    unsafe {turn_on(&mut power_socket)};
    println!("Turned on. State: {:?}", unsafe {get_state(&power_socket)});

    unsafe {turn_off(&mut power_socket)};
    println!("Turned off. State: {:?}", unsafe {get_state(&power_socket)});

}