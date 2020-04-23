#[doc = "Reader of register HCPERIODICSTART"]
pub type R = crate::R<u32, super::HCPERIODICSTART>;
#[doc = "Writer for register HCPERIODICSTART"]
pub type W = crate::W<u32, super::HCPERIODICSTART>;
#[doc = "Register HCPERIODICSTART `reset()`'s with value 0"]
impl crate::ResetValue for super::HCPERIODICSTART {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
}
