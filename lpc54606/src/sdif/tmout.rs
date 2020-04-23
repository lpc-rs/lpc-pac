#[doc = "Reader of register TMOUT"]
pub type R = crate::R<u32, super::TMOUT>;
#[doc = "Writer for register TMOUT"]
pub type W = crate::W<u32, super::TMOUT>;
#[doc = "Register TMOUT `reset()`'s with value 0xffff_ff40"]
impl crate::ResetValue for super::TMOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ff40
    }
}
#[doc = "Reader of field `RESPONSE_TIMEOUT`"]
pub type RESPONSE_TIMEOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESPONSE_TIMEOUT`"]
pub struct RESPONSE_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPONSE_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA_TIMEOUT`"]
pub type DATA_TIMEOUT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATA_TIMEOUT`"]
pub struct DATA_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Response time-out value."]
    #[inline(always)]
    pub fn response_timeout(&self) -> RESPONSE_TIMEOUT_R {
        RESPONSE_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
    #[inline(always)]
    pub fn data_timeout(&self) -> DATA_TIMEOUT_R {
        DATA_TIMEOUT_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:7 - Response time-out value."]
    #[inline(always)]
    pub fn response_timeout(&mut self) -> RESPONSE_TIMEOUT_W {
        RESPONSE_TIMEOUT_W { w: self }
    }
    #[doc = "Bits 8:31 - Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
    #[inline(always)]
    pub fn data_timeout(&mut self) -> DATA_TIMEOUT_W {
        DATA_TIMEOUT_W { w: self }
    }
}
