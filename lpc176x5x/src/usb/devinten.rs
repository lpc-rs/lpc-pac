#[doc = "Reader of register DEVINTEN"]
pub type R = crate::R<u32, super::DEVINTEN>;
#[doc = "Writer for register DEVINTEN"]
pub type W = crate::W<u32, super::DEVINTEN>;
#[doc = "Register DEVINTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVINTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRAMEEN`"]
pub type FRAMEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAMEEN`"]
pub struct FRAMEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEEN_W<'a> {
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
#[doc = "Reader of field `EP_FASTEN`"]
pub type EP_FASTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP_FASTEN`"]
pub struct EP_FASTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_FASTEN_W<'a> {
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
#[doc = "Reader of field `EP_SLOWEN`"]
pub type EP_SLOWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP_SLOWEN`"]
pub struct EP_SLOWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_SLOWEN_W<'a> {
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
#[doc = "Reader of field `DEV_STATEN`"]
pub type DEV_STATEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_STATEN`"]
pub struct DEV_STATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_STATEN_W<'a> {
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
#[doc = "Reader of field `CCEMPTYEN`"]
pub type CCEMPTYEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCEMPTYEN`"]
pub struct CCEMPTYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCEMPTYEN_W<'a> {
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
#[doc = "Reader of field `CDFULLEN`"]
pub type CDFULLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDFULLEN`"]
pub struct CDFULLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CDFULLEN_W<'a> {
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
#[doc = "Reader of field `RxENDPKTEN`"]
pub type RXENDPKTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RxENDPKTEN`"]
pub struct RXENDPKTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENDPKTEN_W<'a> {
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
#[doc = "Reader of field `TxENDPKTEN`"]
pub type TXENDPKTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TxENDPKTEN`"]
pub struct TXENDPKTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDPKTEN_W<'a> {
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
#[doc = "Reader of field `EP_RLZEDEN`"]
pub type EP_RLZEDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP_RLZEDEN`"]
pub struct EP_RLZEDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_RLZEDEN_W<'a> {
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
#[doc = "Reader of field `ERR_INTEN`"]
pub type ERR_INTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR_INTEN`"]
pub struct ERR_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_INTEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn frameen(&self) -> FRAMEEN_R {
        FRAMEEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ep_fasten(&self) -> EP_FASTEN_R {
        EP_FASTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ep_slowen(&self) -> EP_SLOWEN_R {
        EP_SLOWEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn dev_staten(&self) -> DEV_STATEN_R {
        DEV_STATEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ccemptyen(&self) -> CCEMPTYEN_R {
        CCEMPTYEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn cdfullen(&self) -> CDFULLEN_R {
        CDFULLEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn rx_endpkten(&self) -> RXENDPKTEN_R {
        RXENDPKTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn tx_endpkten(&self) -> TXENDPKTEN_R {
        TXENDPKTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ep_rlzeden(&self) -> EP_RLZEDEN_R {
        EP_RLZEDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn err_inten(&self) -> ERR_INTEN_R {
        ERR_INTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn frameen(&mut self) -> FRAMEEN_W {
        FRAMEEN_W { w: self }
    }
    #[doc = "Bit 1 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ep_fasten(&mut self) -> EP_FASTEN_W {
        EP_FASTEN_W { w: self }
    }
    #[doc = "Bit 2 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ep_slowen(&mut self) -> EP_SLOWEN_W {
        EP_SLOWEN_W { w: self }
    }
    #[doc = "Bit 3 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn dev_staten(&mut self) -> DEV_STATEN_W {
        DEV_STATEN_W { w: self }
    }
    #[doc = "Bit 4 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ccemptyen(&mut self) -> CCEMPTYEN_W {
        CCEMPTYEN_W { w: self }
    }
    #[doc = "Bit 5 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn cdfullen(&mut self) -> CDFULLEN_W {
        CDFULLEN_W { w: self }
    }
    #[doc = "Bit 6 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn rx_endpkten(&mut self) -> RXENDPKTEN_W {
        RXENDPKTEN_W { w: self }
    }
    #[doc = "Bit 7 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn tx_endpkten(&mut self) -> TXENDPKTEN_W {
        TXENDPKTEN_W { w: self }
    }
    #[doc = "Bit 8 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ep_rlzeden(&mut self) -> EP_RLZEDEN_W {
        EP_RLZEDEN_W { w: self }
    }
    #[doc = "Bit 9 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn err_inten(&mut self) -> ERR_INTEN_W {
        ERR_INTEN_W { w: self }
    }
}
