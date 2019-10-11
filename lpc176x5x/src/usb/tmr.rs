#[doc = "Reader of register TMR"]
pub type R = crate::R<u32, super::TMR>;
#[doc = "Writer for register TMR"]
pub type W = crate::W<u32, super::TMR>;
#[doc = "Register TMR `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::TMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `TIMEOUT_CNT`"]
pub type TIMEOUT_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMEOUT_CNT`"]
pub struct TIMEOUT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The TMR interrupt is set when TMR_CNT reaches this value."]
    #[inline(always)]
    pub fn timeout_cnt(&self) -> TIMEOUT_CNT_R {
        TIMEOUT_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The TMR interrupt is set when TMR_CNT reaches this value."]
    #[inline(always)]
    pub fn timeout_cnt(&mut self) -> TIMEOUT_CNT_W {
        TIMEOUT_CNT_W { w: self }
    }
}
