#[doc = "Reader of register MAC_PMT_CRTL_STAT"]
pub type R = crate::R<u32, super::MAC_PMT_CRTL_STAT>;
#[doc = "Writer for register MAC_PMT_CRTL_STAT"]
pub type W = crate::W<u32, super::MAC_PMT_CRTL_STAT>;
#[doc = "Register MAC_PMT_CRTL_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_PMT_CRTL_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWRDWN`"]
pub type PWRDWN_R = crate::R<bool, bool>;
#[doc = "Reader of field `MGKPKTEN`"]
pub type MGKPKTEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RWKPKTEN`"]
pub type RWKPKTEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `MGKPRCVD`"]
pub type MGKPRCVD_R = crate::R<bool, bool>;
#[doc = "Reader of field `RWKPRCVD`"]
pub type RWKPRCVD_R = crate::R<bool, bool>;
#[doc = "Reader of field `GLBLUCAST`"]
pub type GLBLUCAST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLBLUCAST`"]
pub struct GLBLUCAST_W<'a> {
    w: &'a mut W,
}
impl<'a> GLBLUCAST_W<'a> {
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
#[doc = "Reader of field `RWKPFE`"]
pub type RWKPFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWKPFE`"]
pub struct RWKPFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKPFE_W<'a> {
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
#[doc = "Reader of field `RWKPTR`"]
pub type RWKPTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RWKPTR`"]
pub struct RWKPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RWKFILTRST`"]
pub type RWKFILTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWKFILTRST`"]
pub struct RWKFILTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKFILTRST_W<'a> {
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
    #[doc = "Bit 0 - Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit."]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable."]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Remote Wake-Up Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wake-up packet."]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received."]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Remote Wake-Up Packet Received."]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wake-up frame."]
    #[inline(always)]
    pub fn rwkpfe(&self) -> RWKPFE_R {
        RWKPFE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Remote Wake-up FIFO Pointer This field gives the current value (0 to 7) of the Remote Wake-up Packet Filter register pointer."]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
    #[inline(always)]
    pub fn glblucast(&mut self) -> GLBLUCAST_W {
        GLBLUCAST_W { w: self }
    }
    #[doc = "Bit 10 - Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wake-up frame."]
    #[inline(always)]
    pub fn rwkpfe(&mut self) -> RWKPFE_W {
        RWKPFE_W { w: self }
    }
    #[doc = "Bits 24:28 - Remote Wake-up FIFO Pointer This field gives the current value (0 to 7) of the Remote Wake-up Packet Filter register pointer."]
    #[inline(always)]
    pub fn rwkptr(&mut self) -> RWKPTR_W {
        RWKPTR_W { w: self }
    }
    #[doc = "Bit 31 - Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
    #[inline(always)]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W {
        RWKFILTRST_W { w: self }
    }
}
