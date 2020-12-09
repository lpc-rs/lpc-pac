#[doc = "Reader of register AUDPLLMDEC"]
pub type R = crate::R<u32, super::AUDPLLMDEC>;
#[doc = "Writer for register AUDPLLMDEC"]
pub type W = crate::W<u32, super::AUDPLLMDEC>;
#[doc = "Register AUDPLLMDEC `reset()`'s with value 0"]
impl crate::ResetValue for super::AUDPLLMDEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MDEC`"]
pub type MDEC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MDEC`"]
pub struct MDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> MDEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Reader of field `MREQ`"]
pub type MREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MREQ`"]
pub struct MREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Decoded M-divider coefficient value."]
    #[inline(always)]
    pub fn mdec(&self) -> MDEC_R {
        MDEC_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 17 - MDEC reload request."]
    #[inline(always)]
    pub fn mreq(&self) -> MREQ_R {
        MREQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Decoded M-divider coefficient value."]
    #[inline(always)]
    pub fn mdec(&mut self) -> MDEC_W {
        MDEC_W { w: self }
    }
    #[doc = "Bit 17 - MDEC reload request."]
    #[inline(always)]
    pub fn mreq(&mut self) -> MREQ_W {
        MREQ_W { w: self }
    }
}
