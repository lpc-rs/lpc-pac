#[doc = "Reader of register RESP[%s]"]
pub type R = crate::R<u32, super::RESP>;
#[doc = "Writer for register RESP[%s]"]
pub type W = crate::W<u32, super::RESP>;
#[doc = "Register RESP[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::RESP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESPONSE`"]
pub type RESPONSE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESPONSE`"]
pub struct RESPONSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPONSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bits of response."]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits of response."]
    #[inline(always)]
    pub fn response(&mut self) -> RESPONSE_W {
        RESPONSE_W { w: self }
    }
}
