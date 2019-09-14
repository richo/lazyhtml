extern crate lazyhtml;
#[macro_use]
extern crate afl;

use std::ptr::null_mut;
use lazyhtml::*;
use std::os::raw::c_void;

struct HandlerState {
    handler: lhtml_token_handler_t,
}

impl HandlerState {
    pub fn new() -> Self {
        HandlerState {
            handler: lhtml_token_handler_t {
                callback: Some(Self::callback),
                next: null_mut(),
            },
        }
    }

    unsafe extern "C" fn callback(token: *mut lhtml_token_t, extra: *mut c_void) {
        lhtml_emit(token, extra);
    }
}

fn main() {
    fuzz!(|input: &[u8]| {
        let mut test_state = HandlerState::new();
        let mut tokenizer = Tokenizer::new(2048, 256);
        test_state.handler.inject_into(&mut tokenizer);
        match std::str::from_utf8(input) {
            Ok(data) => {
                tokenizer.feed(data).expect("Could not feed input");
                tokenizer.end().expect("Could not finalize input");
            },
            Err(_) => {
                // We don't care about invalid UTF for the time being, although we need to look
                // closer at where that trust boundary lies
                return
            },
        }
    })
}
