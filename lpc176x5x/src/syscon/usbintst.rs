#[doc = "Reader of register USBINTST"]
pub type R = crate::R<u32, super::USBINTST>;
#[doc = "Writer for register USBINTST"]
pub type W = crate::W<u32, super::USBINTST>;
#[doc = "Register USBINTST `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::USBINTST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `USB_INT_REQ_LP`"]
pub type USB_INT_REQ_LP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_INT_REQ_LP`"]
pub struct USB_INT_REQ_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_INT_REQ_LP_W<'a> {
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
#[doc = "Reader of field `USB_INT_REQ_HP`"]
pub type USB_INT_REQ_HP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_INT_REQ_HP`"]
pub struct USB_INT_REQ_HP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_INT_REQ_HP_W<'a> {
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
#[doc = "Reader of field `USB_INT_REQ_DMA`"]
pub type USB_INT_REQ_DMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_INT_REQ_DMA`"]
pub struct USB_INT_REQ_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_INT_REQ_DMA_W<'a> {
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
#[doc = "Reader of field `USB_HOST_INT`"]
pub type USB_HOST_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_HOST_INT`"]
pub struct USB_HOST_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_HOST_INT_W<'a> {
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
#[doc = "Reader of field `USB_ATX_INT`"]
pub type USB_ATX_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_ATX_INT`"]
pub struct USB_ATX_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ATX_INT_W<'a> {
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
#[doc = "Reader of field `USB_OTG_INT`"]
pub type USB_OTG_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_OTG_INT`"]
pub struct USB_OTG_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_OTG_INT_W<'a> {
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
#[doc = "Reader of field `USB_I2C_INT`"]
pub type USB_I2C_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_I2C_INT`"]
pub struct USB_I2C_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_I2C_INT_W<'a> {
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
#[doc = "Reader of field `USB_NEED_CLK`"]
pub type USB_NEED_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_NEED_CLK`"]
pub struct USB_NEED_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_NEED_CLK_W<'a> {
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
#[doc = "Reader of field `EN_USB_INTS`"]
pub type EN_USB_INTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_USB_INTS`"]
pub struct EN_USB_INTS_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_USB_INTS_W<'a> {
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
    #[doc = "Bit 0 - Low priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_lp(&self) -> USB_INT_REQ_LP_R {
        USB_INT_REQ_LP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - High priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_hp(&self) -> USB_INT_REQ_HP_R {
        USB_INT_REQ_HP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_dma(&self) -> USB_INT_REQ_DMA_R {
        USB_INT_REQ_DMA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB host interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_host_int(&self) -> USB_HOST_INT_R {
        USB_HOST_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External ATX interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_atx_int(&self) -> USB_ATX_INT_R {
        USB_ATX_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OTG interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_otg_int(&self) -> USB_OTG_INT_R {
        USB_OTG_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C module interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_i2c_int(&self) -> USB_I2C_INT_R {
        USB_I2C_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
    #[inline(always)]
    pub fn usb_need_clk(&self) -> USB_NEED_CLK_R {
        USB_NEED_CLK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
    #[inline(always)]
    pub fn en_usb_ints(&self) -> EN_USB_INTS_R {
        EN_USB_INTS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_lp(&mut self) -> USB_INT_REQ_LP_W {
        USB_INT_REQ_LP_W { w: self }
    }
    #[doc = "Bit 1 - High priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_hp(&mut self) -> USB_INT_REQ_HP_W {
        USB_INT_REQ_HP_W { w: self }
    }
    #[doc = "Bit 2 - DMA interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_dma(&mut self) -> USB_INT_REQ_DMA_W {
        USB_INT_REQ_DMA_W { w: self }
    }
    #[doc = "Bit 3 - USB host interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_host_int(&mut self) -> USB_HOST_INT_W {
        USB_HOST_INT_W { w: self }
    }
    #[doc = "Bit 4 - External ATX interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_atx_int(&mut self) -> USB_ATX_INT_W {
        USB_ATX_INT_W { w: self }
    }
    #[doc = "Bit 5 - OTG interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_otg_int(&mut self) -> USB_OTG_INT_W {
        USB_OTG_INT_W { w: self }
    }
    #[doc = "Bit 6 - I2C module interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_i2c_int(&mut self) -> USB_I2C_INT_W {
        USB_I2C_INT_W { w: self }
    }
    #[doc = "Bit 8 - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
    #[inline(always)]
    pub fn usb_need_clk(&mut self) -> USB_NEED_CLK_W {
        USB_NEED_CLK_W { w: self }
    }
    #[doc = "Bit 31 - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
    #[inline(always)]
    pub fn en_usb_ints(&mut self) -> EN_USB_INTS_W {
        EN_USB_INTS_W { w: self }
    }
}
