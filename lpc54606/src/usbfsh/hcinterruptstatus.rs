#[doc = "Reader of register HCINTERRUPTSTATUS"]
pub type R = crate::R<u32, super::HCINTERRUPTSTATUS>;
#[doc = "Writer for register HCINTERRUPTSTATUS"]
pub type W = crate::W<u32, super::HCINTERRUPTSTATUS>;
#[doc = "Register HCINTERRUPTSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::HCINTERRUPTSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SO`"]
pub type SO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SO`"]
pub struct SO_W<'a> {
    w: &'a mut W,
}
impl<'a> SO_W<'a> {
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
#[doc = "Reader of field `WDH`"]
pub type WDH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDH`"]
pub struct WDH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDH_W<'a> {
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
#[doc = "Reader of field `SF`"]
pub type SF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SF`"]
pub struct SF_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RD`"]
pub type RD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD`"]
pub struct RD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `UE`"]
pub type UE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UE`"]
pub struct UE_W<'a> {
    w: &'a mut W,
}
impl<'a> UE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `FNO`"]
pub type FNO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FNO`"]
pub struct FNO_W<'a> {
    w: &'a mut W,
}
impl<'a> FNO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RHSC`"]
pub type RHSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RHSC`"]
pub struct RHSC_W<'a> {
    w: &'a mut W,
}
impl<'a> RHSC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `OC`"]
pub type OC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OC`"]
pub struct OC_W<'a> {
    w: &'a mut W,
}
impl<'a> OC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn so(&self) -> SO_R {
        SO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
    #[inline(always)]
    pub fn wdh(&self) -> WDH_R {
        WDH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn sf(&self) -> SF_R {
        SF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UnrecoverableError This bit is set when HC detects a system error not related to USB."]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
    #[inline(always)]
    pub fn fno(&self) -> FNO_R {
        FNO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\]
has changed."]
    #[inline(always)]
    pub fn rhsc(&self) -> RHSC_R {
        RHSC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 10:31 - OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
    #[inline(always)]
    pub fn oc(&self) -> OC_R {
        OC_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn so(&mut self) -> SO_W {
        SO_W { w: self }
    }
    #[doc = "Bit 1 - WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
    #[inline(always)]
    pub fn wdh(&mut self) -> WDH_W {
        WDH_W { w: self }
    }
    #[doc = "Bit 2 - StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn sf(&mut self) -> SF_W {
        SF_W { w: self }
    }
    #[doc = "Bit 3 - ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W { w: self }
    }
    #[doc = "Bit 4 - UnrecoverableError This bit is set when HC detects a system error not related to USB."]
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W {
        UE_W { w: self }
    }
    #[doc = "Bit 5 - FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
    #[inline(always)]
    pub fn fno(&mut self) -> FNO_W {
        FNO_W { w: self }
    }
    #[doc = "Bit 6 - RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\]
has changed."]
    #[inline(always)]
    pub fn rhsc(&mut self) -> RHSC_W {
        RHSC_W { w: self }
    }
    #[doc = "Bits 10:31 - OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
    #[inline(always)]
    pub fn oc(&mut self) -> OC_W {
        OC_W { w: self }
    }
}
