#[doc = "Reader of register COMMAND"]
pub type R = crate::R<u32, super::COMMAND>;
#[doc = "Writer for register COMMAND"]
pub type W = crate::W<u32, super::COMMAND>;
#[doc = "Register COMMAND `reset()`'s with value 0"]
impl crate::ResetValue for super::COMMAND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `TXENABLE`"]
pub type TXENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXENABLE`"]
pub struct TXENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENABLE_W<'a> {
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
#[doc = "Reader of field `REGRESET`"]
pub type REGRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGRESET`"]
pub struct REGRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> REGRESET_W<'a> {
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
#[doc = "Reader of field `TXRESET`"]
pub type TXRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRESET`"]
pub struct TXRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRESET_W<'a> {
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
#[doc = "Reader of field `RXRESET`"]
pub type RXRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXRESET`"]
pub struct RXRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRESET_W<'a> {
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
#[doc = "Reader of field `PASSRUNTFRAME`"]
pub type PASSRUNTFRAME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PASSRUNTFRAME`"]
pub struct PASSRUNTFRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> PASSRUNTFRAME_W<'a> {
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
#[doc = "Reader of field `PASSRXFILTER`"]
pub type PASSRXFILTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PASSRXFILTER`"]
pub struct PASSRXFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> PASSRXFILTER_W<'a> {
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
#[doc = "Reader of field `TXFLOWCONTROL`"]
pub type TXFLOWCONTROL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFLOWCONTROL`"]
pub struct TXFLOWCONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFLOWCONTROL_W<'a> {
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
#[doc = "Reader of field `RMII`"]
pub type RMII_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMII`"]
pub struct RMII_W<'a> {
    w: &'a mut W,
}
impl<'a> RMII_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable receive."]
    #[inline(always)]
    pub fn rxenable(&self) -> RXENABLE_R {
        RXENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable transmit."]
    #[inline(always)]
    pub fn txenable(&self) -> TXENABLE_R {
        TXENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
    #[inline(always)]
    pub fn regreset(&self) -> REGRESET_R {
        REGRESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When a 1 is written, the transmit datapath is reset."]
    #[inline(always)]
    pub fn txreset(&self) -> TXRESET_R {
        TXRESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When a 1 is written, the receive datapath is reset."]
    #[inline(always)]
    pub fn rxreset(&self) -> RXRESET_R {
        RXRESET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
    #[inline(always)]
    pub fn passruntframe(&self) -> PASSRUNTFRAME_R {
        PASSRUNTFRAME_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
    #[inline(always)]
    pub fn passrxfilter(&self) -> PASSRXFILTER_R {
        PASSRXFILTER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
    #[inline(always)]
    pub fn txflowcontrol(&self) -> TXFLOWCONTROL_R {
        TXFLOWCONTROL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
    #[inline(always)]
    pub fn rmii(&self) -> RMII_R {
        RMII_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - When set to 1 , indicates full duplex operation."]
    #[inline(always)]
    pub fn fullduplex(&self) -> FULLDUPLEX_R {
        FULLDUPLEX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable receive."]
    #[inline(always)]
    pub fn rxenable(&mut self) -> RXENABLE_W {
        RXENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Enable transmit."]
    #[inline(always)]
    pub fn txenable(&mut self) -> TXENABLE_W {
        TXENABLE_W { w: self }
    }
    #[doc = "Bit 3 - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
    #[inline(always)]
    pub fn regreset(&mut self) -> REGRESET_W {
        REGRESET_W { w: self }
    }
    #[doc = "Bit 4 - When a 1 is written, the transmit datapath is reset."]
    #[inline(always)]
    pub fn txreset(&mut self) -> TXRESET_W {
        TXRESET_W { w: self }
    }
    #[doc = "Bit 5 - When a 1 is written, the receive datapath is reset."]
    #[inline(always)]
    pub fn rxreset(&mut self) -> RXRESET_W {
        RXRESET_W { w: self }
    }
    #[doc = "Bit 6 - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
    #[inline(always)]
    pub fn passruntframe(&mut self) -> PASSRUNTFRAME_W {
        PASSRUNTFRAME_W { w: self }
    }
    #[doc = "Bit 7 - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
    #[inline(always)]
    pub fn passrxfilter(&mut self) -> PASSRXFILTER_W {
        PASSRXFILTER_W { w: self }
    }
    #[doc = "Bit 8 - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
    #[inline(always)]
    pub fn txflowcontrol(&mut self) -> TXFLOWCONTROL_W {
        TXFLOWCONTROL_W { w: self }
    }
    #[doc = "Bit 9 - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
    #[inline(always)]
    pub fn rmii(&mut self) -> RMII_W {
        RMII_W { w: self }
    }
    #[doc = "Bit 10 - When set to 1 , indicates full duplex operation."]
    #[inline(always)]
    pub fn fullduplex(&mut self) -> FULLDUPLEX_W {
        FULLDUPLEX_W { w: self }
    }
}
