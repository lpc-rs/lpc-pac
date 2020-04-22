#[doc = "Reader of register THR1_LOW"]
pub type R = crate::R<u32, super::THR1_LOW>;
#[doc = "Writer for register THR1_LOW"]
pub type W = crate::W<u32, super::THR1_LOW>;
#[doc = "Register THR1_LOW `reset()`'s with value 0"]
impl crate::ResetValue for super::THR1_LOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `THRLOW`"]
pub type THRLOW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `THRLOW`"]
pub struct THRLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> THRLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:15 - Low threshold value against which ADC results will be compared"]
    #[inline(always)]
    pub fn thrlow(&self) -> THRLOW_R {
        THRLOW_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - Low threshold value against which ADC results will be compared"]
    #[inline(always)]
    pub fn thrlow(&mut self) -> THRLOW_W {
        THRLOW_W { w: self }
    }
}
