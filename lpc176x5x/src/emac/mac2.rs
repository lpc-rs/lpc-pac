#[doc = "Reader of register MAC2"]
pub type R = crate::R<u32, super::MAC2>;
#[doc = "Writer for register MAC2"]
pub type W = crate::W<u32, super::MAC2>;
#[doc = "Register MAC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FULLDUPLEX`"]
pub type FULLDUPLEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FULLDUPLEX`"]
pub struct FULLDUPLEX_W<'a> {
    w: &'a mut W,
}
impl<'a> FULLDUPLEX_W<'a> {
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
#[doc = "Reader of field `FLC`"]
pub type FLC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLC`"]
pub struct FLC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLC_W<'a> {
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
#[doc = "Reader of field `HFEN`"]
pub type HFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFEN`"]
pub struct HFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFEN_W<'a> {
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
#[doc = "Reader of field `DELAYEDCRC`"]
pub type DELAYEDCRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DELAYEDCRC`"]
pub struct DELAYEDCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAYEDCRC_W<'a> {
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
#[doc = "Reader of field `CRCEN`"]
pub type CRCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCEN`"]
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
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
#[doc = "Reader of field `PADCRCEN`"]
pub type PADCRCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PADCRCEN`"]
pub struct PADCRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PADCRCEN_W<'a> {
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
#[doc = "Reader of field `VLANPADEN`"]
pub type VLANPADEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLANPADEN`"]
pub struct VLANPADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANPADEN_W<'a> {
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
#[doc = "Reader of field `AUTODETPADEN`"]
pub type AUTODETPADEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTODETPADEN`"]
pub struct AUTODETPADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTODETPADEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PPENF`"]
pub type PPENF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PPENF`"]
pub struct PPENF_W<'a> {
    w: &'a mut W,
}
impl<'a> PPENF_W<'a> {
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
#[doc = "Reader of field `LPENF`"]
pub type LPENF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPENF`"]
pub struct LPENF_W<'a> {
    w: &'a mut W,
}
impl<'a> LPENF_W<'a> {
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
#[doc = "Reader of field `NOBACKOFF`"]
pub type NOBACKOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOBACKOFF`"]
pub struct NOBACKOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> NOBACKOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `BP_NOBACKOFF`"]
pub type BP_NOBACKOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BP_NOBACKOFF`"]
pub struct BP_NOBACKOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> BP_NOBACKOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `EXCESSDEFER`"]
pub type EXCESSDEFER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXCESSDEFER`"]
pub struct EXCESSDEFER_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCESSDEFER_W<'a> {
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
impl R {
    #[doc = "Bit 0 - When enabled (set to 1), the MAC operates in Full-Duplex mode. When disabled, the MAC operates in Half-Duplex mode."]
    #[inline(always)]
    pub fn fullduplex(&self) -> FULLDUPLEX_R {
        FULLDUPLEX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FRAMELENGTH CHECKING. When enabled (set to 1), both transmit and receive frame lengths are compared to the Length/Type field. If the Length/Type field represents a length then the check is performed. Mismatches are reported in the StatusInfo word for each received frame."]
    #[inline(always)]
    pub fn flc(&self) -> FLC_R {
        FLC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HUGE FRAME ENABLEWhen enabled (set to 1), frames of any length are transmitted and received."]
    #[inline(always)]
    pub fn hfen(&self) -> HFEN_R {
        HFEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DELAYED CRC. This bit determines the number of bytes, if any, of proprietary header information that exist on the front of IEEE 802.3 frames. When 1, four bytes of header (ignored by the CRC function) are added. When 0, there is no proprietary header."]
    #[inline(always)]
    pub fn delayedcrc(&self) -> DELAYEDCRC_R {
        DELAYEDCRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC ENABLESet this bit to append a CRC to every frame whether padding was required or not. Must be set if PAD/CRC ENABLE is set. Clear this bit if frames presented to the MAC contain a CRC."]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PAD CRC ENABLE. Set this bit to have the MAC pad all short frames. Clear this bit if frames presented to the MAC have a valid length. This bit is used in conjunction with AUTO PAD ENABLE and VLAN PAD ENABLE. See Table 153 - Pad Operation for details on the pad function."]
    #[inline(always)]
    pub fn padcrcen(&self) -> PADCRCEN_R {
        PADCRCEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VLAN PAD ENABLE. Set this bit to cause the MAC to pad all short frames to 64 bytes and append a valid CRC. Consult Table 153 - Pad Operation for more information on the various padding features. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
    #[inline(always)]
    pub fn vlanpaden(&self) -> VLANPADEN_R {
        VLANPADEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AUTODETECTPAD ENABLE. Set this bit to cause the MAC to automatically detect the type of frame, either tagged or un-tagged, by comparing the two octets following the source address with 0x8100 (VLAN Protocol ID) and pad accordingly. Table 153 - Pad Operation provides a description of the pad function based on the configuration of this register. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
    #[inline(always)]
    pub fn autodetpaden(&self) -> AUTODETPADEN_R {
        AUTODETPADEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PURE PREAMBLE ENFORCEMEN. When enabled (set to 1), the MAC will verify the content of the preamble to ensure it contains 0x55 and is error-free. A packet with an incorrect preamble is discarded. When disabled, no preamble checking is performed."]
    #[inline(always)]
    pub fn ppenf(&self) -> PPENF_R {
        PPENF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LONG PREAMBLE ENFORCEMENT. When enabled (set to 1), the MAC only allows receive packets which contain preamble fields less than 12 bytes in length. When disabled, the MAC allows any length preamble as per the Standard."]
    #[inline(always)]
    pub fn lpenf(&self) -> LPENF_R {
        LPENF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - When enabled (set to 1), the MAC will immediately retransmit following a collision rather than using the Binary Exponential Backoff algorithm as specified in the Standard."]
    #[inline(always)]
    pub fn nobackoff(&self) -> NOBACKOFF_R {
        NOBACKOFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - BACK PRESSURE / NO BACKOFF. When enabled (set to 1), after the MAC incidentally causes a collision during back pressure, it will immediately retransmit without backoff, reducing the chance of further collisions and ensuring transmit packets get sent."]
    #[inline(always)]
    pub fn bp_nobackoff(&self) -> BP_NOBACKOFF_R {
        BP_NOBACKOFF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - When enabled (set to 1) the MAC will defer to carrier indefinitely as per the Standard. When disabled, the MAC will abort when the excessive deferral limit is reached."]
    #[inline(always)]
    pub fn excessdefer(&self) -> EXCESSDEFER_R {
        EXCESSDEFER_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When enabled (set to 1), the MAC operates in Full-Duplex mode. When disabled, the MAC operates in Half-Duplex mode."]
    #[inline(always)]
    pub fn fullduplex(&mut self) -> FULLDUPLEX_W {
        FULLDUPLEX_W { w: self }
    }
    #[doc = "Bit 1 - FRAMELENGTH CHECKING. When enabled (set to 1), both transmit and receive frame lengths are compared to the Length/Type field. If the Length/Type field represents a length then the check is performed. Mismatches are reported in the StatusInfo word for each received frame."]
    #[inline(always)]
    pub fn flc(&mut self) -> FLC_W {
        FLC_W { w: self }
    }
    #[doc = "Bit 2 - HUGE FRAME ENABLEWhen enabled (set to 1), frames of any length are transmitted and received."]
    #[inline(always)]
    pub fn hfen(&mut self) -> HFEN_W {
        HFEN_W { w: self }
    }
    #[doc = "Bit 3 - DELAYED CRC. This bit determines the number of bytes, if any, of proprietary header information that exist on the front of IEEE 802.3 frames. When 1, four bytes of header (ignored by the CRC function) are added. When 0, there is no proprietary header."]
    #[inline(always)]
    pub fn delayedcrc(&mut self) -> DELAYEDCRC_W {
        DELAYEDCRC_W { w: self }
    }
    #[doc = "Bit 4 - CRC ENABLESet this bit to append a CRC to every frame whether padding was required or not. Must be set if PAD/CRC ENABLE is set. Clear this bit if frames presented to the MAC contain a CRC."]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    #[doc = "Bit 5 - PAD CRC ENABLE. Set this bit to have the MAC pad all short frames. Clear this bit if frames presented to the MAC have a valid length. This bit is used in conjunction with AUTO PAD ENABLE and VLAN PAD ENABLE. See Table 153 - Pad Operation for details on the pad function."]
    #[inline(always)]
    pub fn padcrcen(&mut self) -> PADCRCEN_W {
        PADCRCEN_W { w: self }
    }
    #[doc = "Bit 6 - VLAN PAD ENABLE. Set this bit to cause the MAC to pad all short frames to 64 bytes and append a valid CRC. Consult Table 153 - Pad Operation for more information on the various padding features. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
    #[inline(always)]
    pub fn vlanpaden(&mut self) -> VLANPADEN_W {
        VLANPADEN_W { w: self }
    }
    #[doc = "Bit 7 - AUTODETECTPAD ENABLE. Set this bit to cause the MAC to automatically detect the type of frame, either tagged or un-tagged, by comparing the two octets following the source address with 0x8100 (VLAN Protocol ID) and pad accordingly. Table 153 - Pad Operation provides a description of the pad function based on the configuration of this register. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
    #[inline(always)]
    pub fn autodetpaden(&mut self) -> AUTODETPADEN_W {
        AUTODETPADEN_W { w: self }
    }
    #[doc = "Bit 8 - PURE PREAMBLE ENFORCEMEN. When enabled (set to 1), the MAC will verify the content of the preamble to ensure it contains 0x55 and is error-free. A packet with an incorrect preamble is discarded. When disabled, no preamble checking is performed."]
    #[inline(always)]
    pub fn ppenf(&mut self) -> PPENF_W {
        PPENF_W { w: self }
    }
    #[doc = "Bit 9 - LONG PREAMBLE ENFORCEMENT. When enabled (set to 1), the MAC only allows receive packets which contain preamble fields less than 12 bytes in length. When disabled, the MAC allows any length preamble as per the Standard."]
    #[inline(always)]
    pub fn lpenf(&mut self) -> LPENF_W {
        LPENF_W { w: self }
    }
    #[doc = "Bit 12 - When enabled (set to 1), the MAC will immediately retransmit following a collision rather than using the Binary Exponential Backoff algorithm as specified in the Standard."]
    #[inline(always)]
    pub fn nobackoff(&mut self) -> NOBACKOFF_W {
        NOBACKOFF_W { w: self }
    }
    #[doc = "Bit 13 - BACK PRESSURE / NO BACKOFF. When enabled (set to 1), after the MAC incidentally causes a collision during back pressure, it will immediately retransmit without backoff, reducing the chance of further collisions and ensuring transmit packets get sent."]
    #[inline(always)]
    pub fn bp_nobackoff(&mut self) -> BP_NOBACKOFF_W {
        BP_NOBACKOFF_W { w: self }
    }
    #[doc = "Bit 14 - When enabled (set to 1) the MAC will defer to carrier indefinitely as per the Standard. When disabled, the MAC will abort when the excessive deferral limit is reached."]
    #[inline(always)]
    pub fn excessdefer(&mut self) -> EXCESSDEFER_W {
        EXCESSDEFER_W { w: self }
    }
}
