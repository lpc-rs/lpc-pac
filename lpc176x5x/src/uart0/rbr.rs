#[doc = "Reader of register RBR"]
pub type R = crate::R<u32, super::RBR>;
#[doc = "Reader of field `RBR`"]
pub type RBR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The UARTn Receiver Buffer Register contains the oldest received byte in the UARTn Rx FIFO."]
    #[inline(always)]
    pub fn rbr(&self) -> RBR_R {
        RBR_R::new((self.bits & 0xff) as u8)
    }
}
