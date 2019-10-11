#[doc = "Reader of register DATA_BUFFER"]
pub type R = crate::R<u32, super::DATA_BUFFER>;
#[doc = "Reader of field `Data`"]
pub type DATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - This register holds contents of the 8 MSBs of the DAT shift register."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
