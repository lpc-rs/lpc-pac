#[doc = "Writer for register MWTD"]
pub type W = crate::W<u32, super::MWTD>;
#[doc = "Register MWTD `reset()`'s with value 0"]
impl crate::ResetValue for super::MWTD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `WRITEDATA`"]
pub struct WRITEDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - WRITE DATA. When written, an MII Mgmt write cycle is performed using the 16-bit data and the pre-configured PHY and Register addresses from the MII Mgmt Address register (MADR)."]
    #[inline(always)]
    pub fn writedata(&mut self) -> WRITEDATA_W {
        WRITEDATA_W { w: self }
    }
}
