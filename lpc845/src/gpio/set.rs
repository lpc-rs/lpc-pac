#[doc = "Reader of register SET[%s]"]
pub type R = crate::R<u32, super::SET>;
#[doc = "Writer for register SET[%s]"]
pub type W = crate::W<u32, super::SET>;
#[doc = "Register SET[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SETP`"]
pub type SETP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SETP`"]
pub struct SETP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp(&self) -> SETP_R {
        SETP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp(&mut self) -> SETP_W {
        SETP_W { w: self }
    }
}
