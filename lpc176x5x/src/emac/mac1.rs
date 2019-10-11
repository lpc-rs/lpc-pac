#[doc = "Reader of register MAC1"]
pub type R = crate::R<u32, super::MAC1>;
#[doc = "Writer for register MAC1"]
pub type W = crate::W<u32, super::MAC1>;
#[doc = "Register MAC1 `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::MAC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Reader of field `RXENABLE`"]
pub type RXENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXENABLE`"]
pub struct RXENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENABLE_W<'a> {
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
#[doc = "Reader of field `PARF`"]
pub type PARF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARF`"]
pub struct PARF_W<'a> {
    w: &'a mut W,
}
impl<'a> PARF_W<'a> {
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
#[doc = "Reader of field `RXFLOWCTRL`"]
pub type RXFLOWCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFLOWCTRL`"]
pub struct RXFLOWCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLOWCTRL_W<'a> {
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
#[doc = "Reader of field `TXFLOWCTRL`"]
pub type TXFLOWCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFLOWCTRL`"]
pub struct TXFLOWCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFLOWCTRL_W<'a> {
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
#[doc = "Reader of field `LOOPBACK`"]
pub type LOOPBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPBACK`"]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
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
#[doc = "Reader of field `RESETTX`"]
pub type RESETTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETTX`"]
pub struct RESETTX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETTX_W<'a> {
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
#[doc = "Reader of field `RESETMCSTX`"]
pub type RESETMCSTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETMCSTX`"]
pub struct RESETMCSTX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETMCSTX_W<'a> {
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
#[doc = "Reader of field `RESETRX`"]
pub type RESETRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETRX`"]
pub struct RESETRX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETRX_W<'a> {
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
#[doc = "Reader of field `RESETMCSRX`"]
pub type RESETMCSRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETMCSRX`"]
pub struct RESETMCSRX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETMCSRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SIMRESET`"]
pub type SIMRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIMRESET`"]
pub struct SIMRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SOFTRESET`"]
pub type SOFTRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTRESET`"]
pub struct SOFTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
    #[inline(always)]
    pub fn rxenable(&self) -> RXENABLE_R {
        RXENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
    #[inline(always)]
    pub fn parf(&self) -> PARF_R {
        PARF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
    #[inline(always)]
    pub fn rxflowctrl(&self) -> RXFLOWCTRL_R {
        RXFLOWCTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
    #[inline(always)]
    pub fn txflowctrl(&self) -> TXFLOWCTRL_R {
        TXFLOWCTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Setting this bit will put the Transmit Function logic in reset."]
    #[inline(always)]
    pub fn resettx(&self) -> RESETTX_R {
        RESETTX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcstx(&self) -> RESETMCSTX_R {
        RESETMCSTX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Setting this bit will put the Ethernet receive logic in reset."]
    #[inline(always)]
    pub fn resetrx(&self) -> RESETRX_R {
        RESETRX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcsrx(&self) -> RESETMCSRX_R {
        RESETMCSRX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
    #[inline(always)]
    pub fn simreset(&self) -> SIMRESET_R {
        SIMRESET_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
    #[inline(always)]
    pub fn rxenable(&mut self) -> RXENABLE_W {
        RXENABLE_W { w: self }
    }
    #[doc = "Bit 1 - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
    #[inline(always)]
    pub fn parf(&mut self) -> PARF_W {
        PARF_W { w: self }
    }
    #[doc = "Bit 2 - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
    #[inline(always)]
    pub fn rxflowctrl(&mut self) -> RXFLOWCTRL_W {
        RXFLOWCTRL_W { w: self }
    }
    #[doc = "Bit 3 - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
    #[inline(always)]
    pub fn txflowctrl(&mut self) -> TXFLOWCTRL_W {
        TXFLOWCTRL_W { w: self }
    }
    #[doc = "Bit 4 - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bit 8 - Setting this bit will put the Transmit Function logic in reset."]
    #[inline(always)]
    pub fn resettx(&mut self) -> RESETTX_W {
        RESETTX_W { w: self }
    }
    #[doc = "Bit 9 - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcstx(&mut self) -> RESETMCSTX_W {
        RESETMCSTX_W { w: self }
    }
    #[doc = "Bit 10 - Setting this bit will put the Ethernet receive logic in reset."]
    #[inline(always)]
    pub fn resetrx(&mut self) -> RESETRX_W {
        RESETRX_W { w: self }
    }
    #[doc = "Bit 11 - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcsrx(&mut self) -> RESETMCSRX_W {
        RESETMCSRX_W { w: self }
    }
    #[doc = "Bit 14 - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
    #[inline(always)]
    pub fn simreset(&mut self) -> SIMRESET_W {
        SIMRESET_W { w: self }
    }
    #[doc = "Bit 15 - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
    #[inline(always)]
    pub fn softreset(&mut self) -> SOFTRESET_W {
        SOFTRESET_W { w: self }
    }
}
