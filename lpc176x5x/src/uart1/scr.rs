#[doc = "Reader of register SCR"]
pub type R = crate::R<u32, super::SCR>;
#[doc = "Writer for register SCR"]
pub type W = crate::W<u32, super::SCR>;
#[doc = "Register SCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Pad`"]
pub type PAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Pad`"]
pub struct PAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - A readable, writable byte."]
    #[inline(always)]
    pub fn pad(&self) -> PAD_R {
        PAD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - A readable, writable byte."]
    #[inline(always)]
    pub fn pad(&mut self) -> PAD_W {
        PAD_W { w: self }
    }
}
