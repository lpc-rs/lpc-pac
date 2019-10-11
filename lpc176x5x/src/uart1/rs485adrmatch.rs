#[doc = "Reader of register RS485ADRMATCH"]
pub type R = crate::R<u32, super::RS485ADRMATCH>;
#[doc = "Writer for register RS485ADRMATCH"]
pub type W = crate::W<u32, super::RS485ADRMATCH>;
#[doc = "Register RS485ADRMATCH `reset()`'s with value 0"]
impl crate::ResetValue for super::RS485ADRMATCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADRMATCH`"]
pub type ADRMATCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADRMATCH`"]
pub struct ADRMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRMATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Contains the address match value."]
    #[inline(always)]
    pub fn adrmatch(&self) -> ADRMATCH_R {
        ADRMATCH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the address match value."]
    #[inline(always)]
    pub fn adrmatch(&mut self) -> ADRMATCH_W {
        ADRMATCH_W { w: self }
    }
}
