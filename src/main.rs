/*
 * Copyright © 2023 Manuel Bachmann
 * 
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that copyright
 * notice and this permission notice appear in supporting documentation, and
 * that the name of the copyright holders not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  The copyright holders make no representations
 * about the suitability of this software for any purpose.  It is provided "as
 * is" without express or implied warranty.
 *
 * THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

#![no_std]
#![no_main]

extern crate libc;                   // required to interact with the terminal
extern crate serde;
extern crate serde_json_core;

use core::fmt::Write;                // for "write!()"

use serde::{Deserialize, Serialize};
use serde_json_core::de::{Result, Error};
use serde_json_core::heapless::Vec;
use serde_json_core::heapless::String;


#[derive(Deserialize, Serialize)]
struct MyJsonPow {
    v: u32,
    d: &'static str,   // static str instead of dynamic String
}
#[derive(Deserialize, Serialize)]
struct MyJson {
    name: &'static str,            // static str instead of dynamic String
    mails: Vec<&'static str, 4>,   // heapless::Vec instead of std::Vec (here, can have max 4 elements)
    pow: MyJsonPow,
}


// "no_mangle" allows the symbol to still be called "main" in the binary
#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {

    const JSON: &'static str = r#"
      {
        "name": "muirGlacier",
        "mails": [
            "tarnyko@caramail.com",
	    "heropanti@caramail.com"
        ],
        "pow": {
            "v": 9500000,
            "d": "the amount of blocks to delay the difficulty bomb with"
        }
      }"#;

    match serde_json_core::de::from_str::<MyJson>(JSON) {
      Ok(result) => {  unsafe { libc::printf(b"Successfully deserialized.\n\0".as_ptr() as *const _); }
                       // print 1 element (u32)
                       let pow_v = {result.0}.pow.v;       // result=(MyJson, usize) ; result.0=MyJson
                       let mut msg = String::<18>::new();  // alloc.18 (7 for "pow.v: ", 10 for u32, 2 for "\0\n")
                       write!(msg, "pow.v: {pow_v}\n");
                       unsafe { libc::printf(msg.as_ptr() as *const _); }
                    },
      err => {  match err {
                  Err(e) => {  unsafe { libc::printf(b"Error: \0".as_ptr() as *const _); }
                               match e {
                                 Error::InvalidType => unsafe { libc::printf("INVALID TYPE.\n\0".as_ptr() as *const _); },
                                                  _ => unsafe { libc::printf("UNKNOWN.\n\0".as_ptr() as *const _); },
                               }
                            },
                   _ => () // is this reachable ?
                }
             }
    }

    0
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
