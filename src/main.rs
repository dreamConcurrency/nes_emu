pub mod cpu;
pub mod opcodes;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
mod cpu_tests;

fn main() {
    println!("Hello, world!");
}
