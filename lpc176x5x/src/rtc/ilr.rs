#[doc = "Reader of register ILR"]
pub type R = crate::R<u32, super::ILR>;
#[doc = "Writer for register ILR"]
pub type W = crate::W<u32, super::ILR>;
#[doc = "Register ILR `reset()`'s with value 0"]
impl crate::ResetValue for super::ILR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCCIF`"]
pub type RTCCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCCIF`"]
pub struct RTCCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCIF_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RTCALF`"]
pub type RTCALF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCALF`"]
pub struct RTCALF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCALF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
    #[inline(always)]
    pub fn rtccif(&self) -> RTCCIF_R {
        RTCCIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
    #[inline(always)]
    pub fn rtcalf(&self) -> RTCALF_R {
        RTCALF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
    #[inline(always)]
    pub fn rtccif(&mut self) -> RTCCIF_W {
        RTCCIF_W { w: self }
    }
    #[doc = "Bit 1 - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
    #[inline(always)]
    pub fn rtcalf(&mut self) -> RTCALF_W {
        RTCALF_W { w: self }
    }
}
