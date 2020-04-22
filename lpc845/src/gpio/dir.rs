#[doc = "Reader of register DIR[%s]"]
pub type R = crate::R<u32, super::DIR>;
#[doc = "Writer for register DIR[%s]"]
pub type W = crate::W<u32, super::DIR>;
#[doc = "Register DIR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIRP`"]
pub type DIRP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIRP`"]
pub struct DIRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp(&self) -> DIRP_R {
        DIRP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp(&mut self) -> DIRP_W {
        DIRP_W { w: self }
    }
}
