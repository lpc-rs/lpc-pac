#[doc = "Reader of register DAT"]
pub type R = crate::R<u32, super::DAT>;
#[doc = "Writer for register DAT"]
pub type W = crate::W<u32, super::DAT>;
#[doc = "Register DAT `reset()`'s with value 0"]
impl crate::ResetValue for super::DAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Data`"]
pub type DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Data`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register holds data values that have been received or are to be transmitted."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register holds data values that have been received or are to be transmitted."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
