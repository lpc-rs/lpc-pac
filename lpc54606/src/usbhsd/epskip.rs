#[doc = "Reader of register EPSKIP"]
pub type R = crate::R<u32, super::EPSKIP>;
#[doc = "Writer for register EPSKIP"]
pub type W = crate::W<u32, super::EPSKIP>;
#[doc = "Register EPSKIP `reset()`'s with value 0"]
impl crate::ResetValue for super::EPSKIP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SKIP`"]
pub type SKIP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SKIP`"]
pub struct SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> SKIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
    #[inline(always)]
    pub fn skip(&self) -> SKIP_R {
        SKIP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
    #[inline(always)]
    pub fn skip(&mut self) -> SKIP_W {
        SKIP_W { w: self }
    }
}
