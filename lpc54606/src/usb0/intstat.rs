#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Writer for register INTSTAT"]
pub type W = crate::W<u32, super::INTSTAT>;
#[doc = "Register INTSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP0OUT`"]
pub type EP0OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0OUT`"]
pub struct EP0OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0OUT_W<'a> {
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
#[doc = "Reader of field `EP0IN`"]
pub type EP0IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0IN`"]
pub struct EP0IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0IN_W<'a> {
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
#[doc = "Reader of field `EP1OUT`"]
pub type EP1OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1OUT`"]
pub struct EP1OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1OUT_W<'a> {
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
#[doc = "Reader of field `EP1IN`"]
pub type EP1IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1IN`"]
pub struct EP1IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1IN_W<'a> {
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
#[doc = "Reader of field `EP2OUT`"]
pub type EP2OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2OUT`"]
pub struct EP2OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2OUT_W<'a> {
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
#[doc = "Reader of field `EP2IN`"]
pub type EP2IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2IN`"]
pub struct EP2IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2IN_W<'a> {
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
#[doc = "Reader of field `EP3OUT`"]
pub type EP3OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3OUT`"]
pub struct EP3OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3OUT_W<'a> {
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
#[doc = "Reader of field `EP3IN`"]
pub type EP3IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3IN`"]
pub struct EP3IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3IN_W<'a> {
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
#[doc = "Reader of field `EP4OUT`"]
pub type EP4OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4OUT`"]
pub struct EP4OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4OUT_W<'a> {
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
#[doc = "Reader of field `EP4IN`"]
pub type EP4IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4IN`"]
pub struct EP4IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4IN_W<'a> {
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
#[doc = "Reader of field `FRAME_INT`"]
pub type FRAME_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAME_INT`"]
pub struct FRAME_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DEV_INT`"]
pub type DEV_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_INT`"]
pub struct DEV_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_INT_W<'a> {
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
    #[doc = "Bit 0 - Interrupt status register bit for the Control EP0 OUT direction. This bit will be set if NBytes transitions to zero or the skip bit is set by software or a SETUP packet is successfully received for the control EP0. If the IntOnNAK_CO is set, this bit will also be set when a NAK is transmitted for the Control EP0 OUT direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep0out(&self) -> EP0OUT_R {
        EP0OUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt status register bit for the Control EP0 IN direction. This bit will be set if NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_CI is set, this bit will also be set when a NAK is transmitted for the Control EP0 IN direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep0in(&self) -> EP0IN_R {
        EP0IN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt status register bit for the EP1 OUT direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AO is set, this bit will also be set when a NAK is transmitted for the EP1 OUT direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep1out(&self) -> EP1OUT_R {
        EP1OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt status register bit for the EP1 IN direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AI is set, this bit will also be set when a NAK is transmitted for the EP1 IN direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep1in(&self) -> EP1IN_R {
        EP1IN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt status register bit for the EP2 OUT direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AO is set, this bit will also be set when a NAK is transmitted for the EP2 OUT direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep2out(&self) -> EP2OUT_R {
        EP2OUT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt status register bit for the EP2 IN direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AI is set, this bit will also be set when a NAK is transmitted for the EP2 IN direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep2in(&self) -> EP2IN_R {
        EP2IN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt status register bit for the EP3 OUT direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AO is set, this bit will also be set when a NAK is transmitted for the EP3 OUT direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep3out(&self) -> EP3OUT_R {
        EP3OUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt status register bit for the EP3 IN direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AI is set, this bit will also be set when a NAK is transmitted for the EP3 IN direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep3in(&self) -> EP3IN_R {
        EP3IN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt status register bit for the EP4 OUT direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AO is set, this bit will also be set when a NAK is transmitted for the EP4 OUT direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep4out(&self) -> EP4OUT_R {
        EP4OUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt status register bit for the EP4 IN direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AI is set, this bit will also be set when a NAK is transmitted for the EP4 IN direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep4in(&self) -> EP4IN_R {
        EP4IN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Frame interrupt. This bit is set to one every millisecond when the VbusDebounced bit and the DCON bit are set. This bit can be used by software when handling isochronous endpoints. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn frame_int(&self) -> FRAME_INT_R {
        FRAME_INT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Device status interrupt. This bit is set by HW when one of the bits in the Device Status Change register are set. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn dev_int(&self) -> DEV_INT_R {
        DEV_INT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status register bit for the Control EP0 OUT direction. This bit will be set if NBytes transitions to zero or the skip bit is set by software or a SETUP packet is successfully received for the control EP0. If the IntOnNAK_CO is set, this bit will also be set when a NAK is transmitted for the Control EP0 OUT direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep0out(&mut self) -> EP0OUT_W {
        EP0OUT_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt status register bit for the Control EP0 IN direction. This bit will be set if NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_CI is set, this bit will also be set when a NAK is transmitted for the Control EP0 IN direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep0in(&mut self) -> EP0IN_W {
        EP0IN_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt status register bit for the EP1 OUT direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AO is set, this bit will also be set when a NAK is transmitted for the EP1 OUT direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep1out(&mut self) -> EP1OUT_W {
        EP1OUT_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt status register bit for the EP1 IN direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AI is set, this bit will also be set when a NAK is transmitted for the EP1 IN direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep1in(&mut self) -> EP1IN_W {
        EP1IN_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt status register bit for the EP2 OUT direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AO is set, this bit will also be set when a NAK is transmitted for the EP2 OUT direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep2out(&mut self) -> EP2OUT_W {
        EP2OUT_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt status register bit for the EP2 IN direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AI is set, this bit will also be set when a NAK is transmitted for the EP2 IN direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep2in(&mut self) -> EP2IN_W {
        EP2IN_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt status register bit for the EP3 OUT direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AO is set, this bit will also be set when a NAK is transmitted for the EP3 OUT direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep3out(&mut self) -> EP3OUT_W {
        EP3OUT_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt status register bit for the EP3 IN direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AI is set, this bit will also be set when a NAK is transmitted for the EP3 IN direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep3in(&mut self) -> EP3IN_W {
        EP3IN_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt status register bit for the EP4 OUT direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AO is set, this bit will also be set when a NAK is transmitted for the EP4 OUT direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep4out(&mut self) -> EP4OUT_W {
        EP4OUT_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt status register bit for the EP4 IN direction. This bit will be set if the corresponding Active bit is cleared by HW. This is done in case the programmed NBytes transitions to zero or the skip bit is set by software. If the IntOnNAK_AI is set, this bit will also be set when a NAK is transmitted for the EP4 IN direction. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn ep4in(&mut self) -> EP4IN_W {
        EP4IN_W { w: self }
    }
    #[doc = "Bit 30 - Frame interrupt. This bit is set to one every millisecond when the VbusDebounced bit and the DCON bit are set. This bit can be used by software when handling isochronous endpoints. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn frame_int(&mut self) -> FRAME_INT_W {
        FRAME_INT_W { w: self }
    }
    #[doc = "Bit 31 - Device status interrupt. This bit is set by HW when one of the bits in the Device Status Change register are set. Software can clear this bit by writing a one to it."]
    #[inline(always)]
    pub fn dev_int(&mut self) -> DEV_INT_W {
        DEV_INT_W { w: self }
    }
}
