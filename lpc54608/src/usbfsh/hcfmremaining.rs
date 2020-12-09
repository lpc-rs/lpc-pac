#[doc = "Reader of register HCFMREMAINING"]
pub type R = crate::R<u32, super::HCFMREMAINING>;
#[doc = "Writer for register HCFMREMAINING"]
pub type W = crate::W<u32, super::HCFMREMAINING>;
#[doc = "Register HCFMREMAINING `reset()`'s with value 0"]
impl crate::ResetValue for super::HCFMREMAINING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FR`"]
pub type FR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FR`"]
pub struct FR_W<'a> {
    w: &'a mut W,
}
impl<'a> FR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `FRT`"]
pub type FRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRT`"]
pub struct FRT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRT_W<'a> {
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
    #[doc = "Bits 0:13 - FrameRemaining This counter is decremented at each bit time."]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
    #[inline(always)]
    pub fn frt(&self) -> FRT_R {
        FRT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - FrameRemaining This counter is decremented at each bit time."]
    #[inline(always)]
    pub fn fr(&mut self) -> FR_W {
        FR_W { w: self }
    }
    #[doc = "Bit 31 - FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
    #[inline(always)]
    pub fn frt(&mut self) -> FRT_W {
        FRT_W { w: self }
    }
}
