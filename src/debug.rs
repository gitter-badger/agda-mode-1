static mut DEBUG_COMMAND: Option<Box<dyn Fn(String)>> = None;
static mut DEBUG_RESPONSE: Option<Box<dyn Fn(String)>> = None;

pub unsafe fn debug_command_via(f: impl Fn(String) + 'static) {
    DEBUG_COMMAND = Some(Box::new(f));
}

pub unsafe fn debug_response_via(f: impl Fn(String) + 'static) {
    DEBUG_RESPONSE = Some(Box::new(f));
}

pub unsafe fn dont_debug_command() {
    DEBUG_COMMAND = None;
}

pub unsafe fn dont_debug_response() {
    DEBUG_RESPONSE = None;
}

pub(crate) unsafe fn debug_command(s: String) -> bool {
    DEBUG_COMMAND.as_ref().map(|f| f(s)).is_some()
}

pub(crate) unsafe fn debug_response(s: String) -> bool {
    DEBUG_RESPONSE.as_ref().map(|f| f(s)).is_some()
}

pub unsafe fn toggle_debug_command() {
    match DEBUG_COMMAND {
        None => {
            println!("Command debug mode is ON");
            debug_command_via(|s| print!("{}", s))
        }
        Some(_) => {
            println!("Command debug mode is OFF");
            DEBUG_COMMAND = None
        }
    }
}

pub unsafe fn toggle_debug_response() {
    match DEBUG_RESPONSE {
        None => {
            println!("Response debug mode is ON");
            debug_response_via(|s| print!("{}", s))
        }
        Some(_) => {
            println!("Response debug mode is OFF");
            DEBUG_RESPONSE = None
        }
    }
}
