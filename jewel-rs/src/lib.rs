#![no_std]

pub trait BleRadio<'a> {
    fn set_buffer(&mut self, buffer: &'a [u8]);

    /// Transmit the packaget in the  buffer
    #[allow(async_fn_in_trait)]
    async fn transmit(&mut self);

    /// Receive the packaget to the buffer
    #[allow(async_fn_in_trait)]
    async fn receive(&mut self);
}
