#[doc = "Reader of register STCTRL"]
pub type R = crate::R<u32, super::STCTRL>;
#[doc = "Writer for register STCTRL"]
pub type W = crate::W<u32, super::STCTRL>;
#[doc = "Register STCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::STCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORT_FUNC`"]
pub type PORT_FUNC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PORT_FUNC`"]
pub struct PORT_FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_FUNC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TMR_SCALE`"]
pub type TMR_SCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMR_SCALE`"]
pub struct TMR_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TMR_MODE`"]
pub type TMR_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMR_MODE`"]
pub struct TMR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_MODE_W<'a> {
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
#[doc = "Reader of field `TMR_EN`"]
pub type TMR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMR_EN`"]
pub struct TMR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_EN_W<'a> {
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
#[doc = "Reader of field `TMR_RST`"]
pub type TMR_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMR_RST`"]
pub struct TMR_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_RST_W<'a> {
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
#[doc = "Reader of field `B_HNP_TRACK`"]
pub type B_HNP_TRACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B_HNP_TRACK`"]
pub struct B_HNP_TRACK_W<'a> {
    w: &'a mut W,
}
impl<'a> B_HNP_TRACK_W<'a> {
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
#[doc = "Reader of field `A_HNP_TRACK`"]
pub type A_HNP_TRACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A_HNP_TRACK`"]
pub struct A_HNP_TRACK_W<'a> {
    w: &'a mut W,
}
impl<'a> A_HNP_TRACK_W<'a> {
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
#[doc = "Reader of field `PU_REMOVED`"]
pub type PU_REMOVED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PU_REMOVED`"]
pub struct PU_REMOVED_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_REMOVED_W<'a> {
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
#[doc = "Reader of field `TMR_CNT`"]
pub type TMR_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TMR_CNT`"]
pub struct TMR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
    #[inline(always)]
    pub fn port_func(&self) -> PORT_FUNC_R {
        PORT_FUNC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
    #[inline(always)]
    pub fn tmr_scale(&self) -> TMR_SCALE_R {
        TMR_SCALE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Timer mode selection. 0: monoshot 1: free running"]
    #[inline(always)]
    pub fn tmr_mode(&self) -> TMR_MODE_R {
        TMR_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
    #[inline(always)]
    pub fn tmr_en(&self) -> TMR_EN_R {
        TMR_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
    #[inline(always)]
    pub fn tmr_rst(&self) -> TMR_RST_R {
        TMR_RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn b_hnp_track(&self) -> B_HNP_TRACK_R {
        B_HNP_TRACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn a_hnp_track(&self) -> A_HNP_TRACK_R {
        A_HNP_TRACK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn pu_removed(&self) -> PU_REMOVED_R {
        PU_REMOVED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Current timer count value."]
    #[inline(always)]
    pub fn tmr_cnt(&self) -> TMR_CNT_R {
        TMR_CNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
    #[inline(always)]
    pub fn port_func(&mut self) -> PORT_FUNC_W {
        PORT_FUNC_W { w: self }
    }
    #[doc = "Bits 2:3 - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
    #[inline(always)]
    pub fn tmr_scale(&mut self) -> TMR_SCALE_W {
        TMR_SCALE_W { w: self }
    }
    #[doc = "Bit 4 - Timer mode selection. 0: monoshot 1: free running"]
    #[inline(always)]
    pub fn tmr_mode(&mut self) -> TMR_MODE_W {
        TMR_MODE_W { w: self }
    }
    #[doc = "Bit 5 - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
    #[inline(always)]
    pub fn tmr_en(&mut self) -> TMR_EN_W {
        TMR_EN_W { w: self }
    }
    #[doc = "Bit 6 - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
    #[inline(always)]
    pub fn tmr_rst(&mut self) -> TMR_RST_W {
        TMR_RST_W { w: self }
    }
    #[doc = "Bit 8 - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn b_hnp_track(&mut self) -> B_HNP_TRACK_W {
        B_HNP_TRACK_W { w: self }
    }
    #[doc = "Bit 9 - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn a_hnp_track(&mut self) -> A_HNP_TRACK_W {
        A_HNP_TRACK_W { w: self }
    }
    #[doc = "Bit 10 - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn pu_removed(&mut self) -> PU_REMOVED_W {
        PU_REMOVED_W { w: self }
    }
    #[doc = "Bits 16:31 - Current timer count value."]
    #[inline(always)]
    pub fn tmr_cnt(&mut self) -> TMR_CNT_W {
        TMR_CNT_W { w: self }
    }
}
