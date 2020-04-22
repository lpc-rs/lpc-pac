#[doc = "Reader of register AUDPLLPDEC"]
pub type R = crate::R<u32, super::AUDPLLPDEC>;
#[doc = "Writer for register AUDPLLPDEC"]
pub type W = crate::W<u32, super::AUDPLLPDEC>;
#[doc = "Register AUDPLLPDEC `reset()`'s with value 0"]
impl crate::ResetValue for super::AUDPLLPDEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDEC`"]
pub type PDEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDEC`"]
pub struct PDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `PREQ`"]
pub type PREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREQ`"]
pub struct PREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Decoded P-divider coefficient value."]
    #[inline(always)]
    pub fn pdec(&self) -> PDEC_R {
        PDEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - PDEC reload request."]
    #[inline(always)]
    pub fn preq(&self) -> PREQ_R {
        PREQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Decoded P-divider coefficient value."]
    #[inline(always)]
    pub fn pdec(&mut self) -> PDEC_W {
        PDEC_W { w: self }
    }
    #[doc = "Bit 7 - PDEC reload request."]
    #[inline(always)]
    pub fn preq(&mut self) -> PREQ_W {
        PREQ_W { w: self }
    }
}
