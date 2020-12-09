#[doc = "Reader of register HCFMINTERVAL"]
pub type R = crate::R<u32, super::HCFMINTERVAL>;
#[doc = "Writer for register HCFMINTERVAL"]
pub type W = crate::W<u32, super::HCFMINTERVAL>;
#[doc = "Register HCFMINTERVAL `reset()`'s with value 0x2edf"]
impl crate::ResetValue for super::HCFMINTERVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2edf
    }
}
#[doc = "Reader of field `FI`"]
pub type FI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FI`"]
pub struct FI_W<'a> {
    w: &'a mut W,
}
impl<'a> FI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `FSMPS`"]
pub type FSMPS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FSMPS`"]
pub struct FSMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `FIT`"]
pub type FIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIT`"]
pub struct FIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIT_W<'a> {
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
    #[doc = "Bits 0:13 - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[inline(always)]
    pub fn fi(&self) -> FI_R {
        FI_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:30 - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[inline(always)]
    pub fn fsmps(&self) -> FSMPS_R {
        FSMPS_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    pub fn fit(&self) -> FIT_R {
        FIT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[inline(always)]
    pub fn fi(&mut self) -> FI_W {
        FI_W { w: self }
    }
    #[doc = "Bits 16:30 - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[inline(always)]
    pub fn fsmps(&mut self) -> FSMPS_W {
        FSMPS_W { w: self }
    }
    #[doc = "Bit 31 - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    pub fn fit(&mut self) -> FIT_W {
        FIT_W { w: self }
    }
}
