#[doc = "Reader of register OSR"]
pub type R = crate::R<u32, super::OSR>;
#[doc = "Writer for register OSR"]
pub type W = crate::W<u32, super::OSR>;
#[doc = "Register OSR `reset()`'s with value 0"]
impl crate::ResetValue for super::OSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSR`"]
pub type OSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSR`"]
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects the oversample rate for the related input channel."]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects the oversample rate for the related input channel."]
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W {
        OSR_W { w: self }
    }
}
