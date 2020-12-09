#[doc = "Reader of register HCCONTROL"]
pub type R = crate::R<u32, super::HCCONTROL>;
#[doc = "Writer for register HCCONTROL"]
pub type W = crate::W<u32, super::HCCONTROL>;
#[doc = "Register HCCONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::HCCONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CBSR`"]
pub type CBSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CBSR`"]
pub struct CBSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CBSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PLE`"]
pub type PLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLE`"]
pub struct PLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLE_W<'a> {
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
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
#[doc = "Reader of field `CLE`"]
pub type CLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLE`"]
pub struct CLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLE_W<'a> {
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
#[doc = "Reader of field `BLE`"]
pub type BLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLE`"]
pub struct BLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_W<'a> {
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
#[doc = "Reader of field `HCFS`"]
pub type HCFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HCFS`"]
pub struct HCFS_W<'a> {
    w: &'a mut W,
}
impl<'a> HCFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `IR`"]
pub type IR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR`"]
pub struct IR_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RWC`"]
pub type RWC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWC`"]
pub struct RWC_W<'a> {
    w: &'a mut W,
}
impl<'a> RWC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RWE`"]
pub type RWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWE`"]
pub struct RWE_W<'a> {
    w: &'a mut W,
}
impl<'a> RWE_W<'a> {
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
    #[doc = "Bits 0:1 - ControlBulkServiceRatio."]
    #[inline(always)]
    pub fn cbsr(&self) -> CBSR_R {
        CBSR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - PeriodicListEnable."]
    #[inline(always)]
    pub fn ple(&self) -> PLE_R {
        PLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IsochronousEnable."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ControlListEnable."]
    #[inline(always)]
    pub fn cle(&self) -> CLE_R {
        CLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BulkListEnable This bit is set to enable the processing of the Bulk list in the next Frame."]
    #[inline(always)]
    pub fn ble(&self) -> BLE_R {
        BLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - HostControllerFunctionalState for USB 00b: USBRESET 01b: USBRESUME 10b: USBOPERATIONAL 11b: USBSUSPEND A transition to USBOPERATIONAL from another state causes SOFgeneration to begin 1 ms later."]
    #[inline(always)]
    pub fn hcfs(&self) -> HCFS_R {
        HCFS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus."]
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RemoteWakeupConnected This bit indicates whether HC supports remote wake-up signaling."]
    #[inline(always)]
    pub fn rwc(&self) -> RWC_R {
        RWC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RemoteWakeupEnable This bit is used by HCD to enable or disable the remote wake-up feature upon the detection of upstream resume signaling."]
    #[inline(always)]
    pub fn rwe(&self) -> RWE_R {
        RWE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ControlBulkServiceRatio."]
    #[inline(always)]
    pub fn cbsr(&mut self) -> CBSR_W {
        CBSR_W { w: self }
    }
    #[doc = "Bit 2 - PeriodicListEnable."]
    #[inline(always)]
    pub fn ple(&mut self) -> PLE_W {
        PLE_W { w: self }
    }
    #[doc = "Bit 3 - IsochronousEnable."]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 4 - ControlListEnable."]
    #[inline(always)]
    pub fn cle(&mut self) -> CLE_W {
        CLE_W { w: self }
    }
    #[doc = "Bit 5 - BulkListEnable This bit is set to enable the processing of the Bulk list in the next Frame."]
    #[inline(always)]
    pub fn ble(&mut self) -> BLE_W {
        BLE_W { w: self }
    }
    #[doc = "Bits 6:7 - HostControllerFunctionalState for USB 00b: USBRESET 01b: USBRESUME 10b: USBOPERATIONAL 11b: USBSUSPEND A transition to USBOPERATIONAL from another state causes SOFgeneration to begin 1 ms later."]
    #[inline(always)]
    pub fn hcfs(&mut self) -> HCFS_W {
        HCFS_W { w: self }
    }
    #[doc = "Bit 8 - InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus."]
    #[inline(always)]
    pub fn ir(&mut self) -> IR_W {
        IR_W { w: self }
    }
    #[doc = "Bit 9 - RemoteWakeupConnected This bit indicates whether HC supports remote wake-up signaling."]
    #[inline(always)]
    pub fn rwc(&mut self) -> RWC_W {
        RWC_W { w: self }
    }
    #[doc = "Bit 10 - RemoteWakeupEnable This bit is used by HCD to enable or disable the remote wake-up feature upon the detection of upstream resume signaling."]
    #[inline(always)]
    pub fn rwe(&mut self) -> RWE_W {
        RWE_W { w: self }
    }
}
