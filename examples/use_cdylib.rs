use libloading::{Library, Symbol};
type FunctionsFn = extern "C" fn() -> Functions;
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

fn main() {
    let filename = "/home/alex/projects/smart_socket/target/release/libsmart_socket.so";
    let lib = unsafe { Library::new(filename).expect("Unable to load library at {filename}") };

    let get_functions: Symbol<FunctionsFn> = unsafe {
        lib.get(b"load_functions")
            .expect("Unable to load functions")
    };

    let functions = get_functions();

    //Box will handle drop of received allocated mem
    let mut power_socket = unsafe { Box::from_raw((functions.new)())};

    (functions.turn_on)(&mut *power_socket as *mut PowerSocket);

    println!("Turned on. State: {:?}", unsafe { *(functions.status)(&*power_socket as *const PowerSocket) });

    (functions.turn_off)(&mut *power_socket as *mut PowerSocket);
    println!("Turned off. State: {:?}", unsafe { *(functions.status)(&*power_socket as *const PowerSocket) });
}
