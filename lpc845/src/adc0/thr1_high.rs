#[doc = "Reader of register THR1_HIGH"]
pub type R = crate::R<u32, super::THR1_HIGH>;
#[doc = "Writer for register THR1_HIGH"]
pub type W = crate::W<u32, super::THR1_HIGH>;
#[doc = "Register THR1_HIGH `reset()`'s with value 0"]
impl crate::ResetValue for super::THR1_HIGH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `THRHIGH`"]
pub type THRHIGH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `THRHIGH`"]
pub struct THRHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> THRHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:15 - High threshold value against which ADC results will be compared"]
    #[inline(always)]
    pub fn thrhigh(&self) -> THRHIGH_R {
        THRHIGH_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - High threshold value against which ADC results will be compared"]
    #[inline(always)]
    pub fn thrhigh(&mut self) -> THRHIGH_W {
        THRHIGH_W { w: self }
    }
}
