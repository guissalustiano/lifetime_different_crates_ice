#![no_std]
#![allow(async_fn_in_trait)]

use jewel::BleRadio;

/// Radio driver.
pub struct Radio {}

impl BleRadio for Radio {
    fn set_buffer(&mut self, buffer: &[u8]) {}

    /// Send packet
    async fn transmit(&mut self) {}

    /// Send packet
    async fn receive(&mut self) {}
}
