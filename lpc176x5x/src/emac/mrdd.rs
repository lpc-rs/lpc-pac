#[doc = "Reader of register MRDD"]
pub type R = crate::R<u32, super::MRDD>;
#[doc = "Reader of field `READDATA`"]
pub type READDATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - READ DATA. Following an MII Mgmt Read Cycle, the 16-bit data can be read from this location."]
    #[inline(always)]
    pub fn readdata(&self) -> READDATA_R {
        READDATA_R::new((self.bits & 0xffff) as u16)
    }
}
