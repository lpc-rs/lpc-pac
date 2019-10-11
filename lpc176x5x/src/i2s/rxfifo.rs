#[doc = "Reader of register RXFIFO"]
pub type R = crate::R<u32, super::RXFIFO>;
#[doc = "Reader of field `I2SRXFIFO`"]
pub type I2SRXFIFO_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 8 x 32-bit transmit FIFO."]
    #[inline(always)]
    pub fn i2srxfifo(&self) -> I2SRXFIFO_R {
        I2SRXFIFO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
