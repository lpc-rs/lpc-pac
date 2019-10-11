#[doc = "Reader of register PIN[%s]"]
pub type R = crate::R<u32, super::PIN>;
#[doc = "Writer for register PIN[%s]"]
pub type W = crate::W<u32, super::PIN>;
#[doc = "Register PIN[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::PIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORT`"]
pub type PORT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PORT`"]
pub struct PORT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W { w: self }
    }
}
