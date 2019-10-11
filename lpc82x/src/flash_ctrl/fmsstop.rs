#[doc = "Reader of register FMSSTOP"]
pub type R = crate::R<u32, super::FMSSTOP>;
#[doc = "Writer for register FMSSTOP"]
pub type W = crate::W<u32, super::FMSSTOP>;
#[doc = "Register FMSSTOP `reset()`'s with value 0"]
impl crate::ResetValue for super::FMSSTOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STOPA`"]
pub type STOPA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STOPA`"]
pub struct STOPA_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Reader of field `STRTBIST`"]
pub type STRTBIST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STRTBIST`"]
pub struct STRTBIST_W<'a> {
    w: &'a mut W,
}
impl<'a> STRTBIST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Stop address for signature generation (the word specified by STOP is included in the address range). The address is in units of memory words, not bytes."]
    #[inline(always)]
    pub fn stopa(&self) -> STOPA_R {
        STOPA_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 31 - When this bit is written to 1, signature generation starts. At the end of signature generation, this bit is automatically cleared."]
    #[inline(always)]
    pub fn strtbist(&self) -> STRTBIST_R {
        STRTBIST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Stop address for signature generation (the word specified by STOP is included in the address range). The address is in units of memory words, not bytes."]
    #[inline(always)]
    pub fn stopa(&mut self) -> STOPA_W {
        STOPA_W { w: self }
    }
    #[doc = "Bit 31 - When this bit is written to 1, signature generation starts. At the end of signature generation, this bit is automatically cleared."]
    #[inline(always)]
    pub fn strtbist(&mut self) -> STRTBIST_W {
        STRTBIST_W { w: self }
    }
}
