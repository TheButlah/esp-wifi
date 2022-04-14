#![no_std]
#![feature(c_variadic)]
#![cfg_attr(target_arch = "xtensa", feature(asm_experimental_arch))]

pub mod binary;
pub mod compat;
pub mod preempt;
pub mod print;
#[cfg_attr(feature = "esp32c3", path = "timer_esp32c3.rs")]
#[cfg_attr(feature = "esp32", path = "timer_esp32.rs")]
pub mod timer;
pub mod wifi;

pub mod tasks;

pub(crate) mod memory_fence;

pub use critical_section;

extern "C" {
    // ROM functions, see esp32c3-link.x
    pub fn uart_tx_one_char(byte: u8) -> i32;
}
pub struct Uart;

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(for &b in s.as_bytes() {
            unsafe { uart_tx_one_char(b) };
        })
    }
}
