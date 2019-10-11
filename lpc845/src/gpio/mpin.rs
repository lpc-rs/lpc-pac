#[doc = "Reader of register MPIN[%s]"]
pub type R = crate::R<u32, super::MPIN>;
#[doc = "Writer for register MPIN[%s]"]
pub type W = crate::W<u32, super::MPIN>;
#[doc = "Register MPIN[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::MPIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPORTP`"]
pub type MPORTP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MPORTP`"]
pub struct MPORTP_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp(&self) -> MPORTP_R {
        MPORTP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp(&mut self) -> MPORTP_W {
        MPORTP_W { w: self }
    }
}
