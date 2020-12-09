#[doc = "Reader of register SYSPLLNDEC"]
pub type R = crate::R<u32, super::SYSPLLNDEC>;
#[doc = "Writer for register SYSPLLNDEC"]
pub type W = crate::W<u32, super::SYSPLLNDEC>;
#[doc = "Register SYSPLLNDEC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSPLLNDEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NDEC`"]
pub type NDEC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NDEC`"]
pub struct NDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NDEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `NREQ`"]
pub type NREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NREQ`"]
pub struct NREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> NREQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Decoded N-divider coefficient value."]
    #[inline(always)]
    pub fn ndec(&self) -> NDEC_R {
        NDEC_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - NDEC reload request."]
    #[inline(always)]
    pub fn nreq(&self) -> NREQ_R {
        NREQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Decoded N-divider coefficient value."]
    #[inline(always)]
    pub fn ndec(&mut self) -> NDEC_W {
        NDEC_W { w: self }
    }
    #[doc = "Bit 10 - NDEC reload request."]
    #[inline(always)]
    pub fn nreq(&mut self) -> NREQ_W {
        NREQ_W { w: self }
    }
}
