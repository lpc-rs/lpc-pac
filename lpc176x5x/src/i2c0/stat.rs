#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `Status`"]
pub type STATUS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 3:7 - These bits give the actual status information about the I 2C interface."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
