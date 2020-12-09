#[doc = "Reader of register PREAC2FSCOEF"]
pub type R = crate::R<u32, super::PREAC2FSCOEF>;
#[doc = "Writer for register PREAC2FSCOEF"]
pub type W = crate::W<u32, super::PREAC2FSCOEF>;
#[doc = "Register PREAC2FSCOEF `reset()`'s with value 0"]
impl crate::ResetValue for super::PREAC2FSCOEF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP`"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Pre-emphasis filer coefficient for 2 FS mode. 0 = Compensation = 0 1 = Compensation = 16 2 = Compensation = 15 3 = Compensation = 13"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pre-emphasis filer coefficient for 2 FS mode. 0 = Compensation = 0 1 = Compensation = 16 2 = Compensation = 15 3 = Compensation = 13"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
}
