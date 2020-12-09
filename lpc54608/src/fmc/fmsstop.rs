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
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Reader of field `SIG_START`"]
pub type SIG_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIG_START`"]
pub struct SIG_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_START_W<'a> {
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
    #[doc = "Bits 0:16 - Stop address for signature generation (the word specified by STOP is included in the address range)."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 17 - When this bit is written to 1, signature generation starts."]
    #[inline(always)]
    pub fn sig_start(&self) -> SIG_START_R {
        SIG_START_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Stop address for signature generation (the word specified by STOP is included in the address range)."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 17 - When this bit is written to 1, signature generation starts."]
    #[inline(always)]
    pub fn sig_start(&mut self) -> SIG_START_W {
        SIG_START_W { w: self }
    }
}
