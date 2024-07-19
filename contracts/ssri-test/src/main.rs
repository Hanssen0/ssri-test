#![no_std]
#![cfg_attr(not(test), no_main)]

mod error;
mod fallback;
mod ssri;
mod syscall;

#[cfg(test)]
extern crate alloc;

#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

pub fn program_entry() -> i8 {
    let argv = ckb_std::env::argv();

    if argv.len() == 0 {
        return match fallback::fallback() {
            Ok(_) => 0,
            Err(err) => err as i8,
        };
    }

    return match ssri::ssri() {
        Ok(_) => 0,
        Err(err) => err as i8,
    };
}
