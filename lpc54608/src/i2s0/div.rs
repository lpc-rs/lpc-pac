#[doc = "Reader of register DIV"]
pub type R = crate::R<u32, super::DIV>;
#[doc = "Writer for register DIV"]
pub type W = crate::W<u32, super::DIV>;
#[doc = "Register DIV `reset()`'s with value 0"]
impl crate::ResetValue for super::DIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - This field controls how this I2S block uses the Flexcomm function clock. 0x000 = The Flexcomm function clock is used directly. 0x001 = The Flexcomm function clock is divided by 2. 0x002 = The Flexcomm function clock is divided by 3. 0xFFF = The Flexcomm function clock is divided by 4,096."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This field controls how this I2S block uses the Flexcomm function clock. 0x000 = The Flexcomm function clock is used directly. 0x001 = The Flexcomm function clock is divided by 2. 0x002 = The Flexcomm function clock is divided by 3. 0xFFF = The Flexcomm function clock is divided by 4,096."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
}
