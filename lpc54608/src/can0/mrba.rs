#[doc = "Reader of register MRBA"]
pub type R = crate::R<u32, super::MRBA>;
#[doc = "Writer for register MRBA"]
pub type W = crate::W<u32, super::MRBA>;
#[doc = "Register MRBA `reset()`'s with value 0"]
impl crate::ResetValue for super::MRBA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BA`"]
pub type BA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BA`"]
pub struct BA_W<'a> {
    w: &'a mut W,
}
impl<'a> BA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Base address for the message RAM in the chip memory map."]
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Base address for the message RAM in the chip memory map."]
    #[inline(always)]
    pub fn ba(&mut self) -> BA_W {
        BA_W { w: self }
    }
}
