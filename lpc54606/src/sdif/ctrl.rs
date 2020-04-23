#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONTROLLER_RESET`"]
pub type CONTROLLER_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONTROLLER_RESET`"]
pub struct CONTROLLER_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTROLLER_RESET_W<'a> {
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
#[doc = "Reader of field `FIFO_RESET`"]
pub type FIFO_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_RESET`"]
pub struct FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_RESET_W<'a> {
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
#[doc = "Reader of field `DMA_RESET`"]
pub type DMA_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_RESET`"]
pub struct DMA_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RESET_W<'a> {
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
#[doc = "Reader of field `INT_ENABLE`"]
pub type INT_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_ENABLE`"]
pub struct INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_ENABLE_W<'a> {
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
#[doc = "Reader of field `READ_WAIT`"]
pub type READ_WAIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READ_WAIT`"]
pub struct READ_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_WAIT_W<'a> {
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
#[doc = "Reader of field `SEND_IRQ_RESPONSE`"]
pub type SEND_IRQ_RESPONSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_IRQ_RESPONSE`"]
pub struct SEND_IRQ_RESPONSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_IRQ_RESPONSE_W<'a> {
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
#[doc = "Reader of field `ABORT_READ_DATA`"]
pub type ABORT_READ_DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABORT_READ_DATA`"]
pub struct ABORT_READ_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_READ_DATA_W<'a> {
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
#[doc = "Reader of field `SEND_CCSD`"]
pub type SEND_CCSD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_CCSD`"]
pub struct SEND_CCSD_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_CCSD_W<'a> {
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
#[doc = "Reader of field `SEND_AUTO_STOP_CCSD`"]
pub type SEND_AUTO_STOP_CCSD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_AUTO_STOP_CCSD`"]
pub struct SEND_AUTO_STOP_CCSD_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_AUTO_STOP_CCSD_W<'a> {
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
#[doc = "Reader of field `CEATA_DEVICE_INTERRUPT_STATUS`"]
pub type CEATA_DEVICE_INTERRUPT_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEATA_DEVICE_INTERRUPT_STATUS`"]
pub struct CEATA_DEVICE_INTERRUPT_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CEATA_DEVICE_INTERRUPT_STATUS_W<'a> {
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
#[doc = "Reader of field `CARD_VOLTAGE_A0`"]
pub type CARD_VOLTAGE_A0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_VOLTAGE_A0`"]
pub struct CARD_VOLTAGE_A0_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_VOLTAGE_A0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CARD_VOLTAGE_A1`"]
pub type CARD_VOLTAGE_A1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_VOLTAGE_A1`"]
pub struct CARD_VOLTAGE_A1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_VOLTAGE_A1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CARD_VOLTAGE_A2`"]
pub type CARD_VOLTAGE_A2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_VOLTAGE_A2`"]
pub struct CARD_VOLTAGE_A2_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_VOLTAGE_A2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `USE_INTERNAL_DMAC`"]
pub type USE_INTERNAL_DMAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USE_INTERNAL_DMAC`"]
pub struct USE_INTERNAL_DMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> USE_INTERNAL_DMAC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Controller reset."]
    #[inline(always)]
    pub fn controller_reset(&self) -> CONTROLLER_RESET_R {
        CONTROLLER_RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fifo reset."]
    #[inline(always)]
    pub fn fifo_reset(&self) -> FIFO_RESET_R {
        FIFO_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA reset."]
    #[inline(always)]
    pub fn dma_reset(&self) -> DMA_RESET_R {
        DMA_RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit."]
    #[inline(always)]
    pub fn int_enable(&self) -> INT_ENABLE_R {
        INT_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read/wait."]
    #[inline(always)]
    pub fn read_wait(&self) -> READ_WAIT_R {
        READ_WAIT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Send irq response."]
    #[inline(always)]
    pub fn send_irq_response(&self) -> SEND_IRQ_RESPONSE_R {
        SEND_IRQ_RESPONSE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Abort read data."]
    #[inline(always)]
    pub fn abort_read_data(&self) -> ABORT_READ_DATA_R {
        ABORT_READ_DATA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Send ccsd."]
    #[inline(always)]
    pub fn send_ccsd(&self) -> SEND_CCSD_R {
        SEND_CCSD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Send auto stop ccsd."]
    #[inline(always)]
    pub fn send_auto_stop_ccsd(&self) -> SEND_AUTO_STOP_CCSD_R {
        SEND_AUTO_STOP_CCSD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CEATA device interrupt status."]
    #[inline(always)]
    pub fn ceata_device_interrupt_status(&self) -> CEATA_DEVICE_INTERRUPT_STATUS_R {
        CEATA_DEVICE_INTERRUPT_STATUS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Controls the state of the SD_VOLT0 pin."]
    #[inline(always)]
    pub fn card_voltage_a0(&self) -> CARD_VOLTAGE_A0_R {
        CARD_VOLTAGE_A0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Controls the state of the SD_VOLT1 pin."]
    #[inline(always)]
    pub fn card_voltage_a1(&self) -> CARD_VOLTAGE_A1_R {
        CARD_VOLTAGE_A1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Controls the state of the SD_VOLT2 pin."]
    #[inline(always)]
    pub fn card_voltage_a2(&self) -> CARD_VOLTAGE_A2_R {
        CARD_VOLTAGE_A2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SD/MMC DMA use."]
    #[inline(always)]
    pub fn use_internal_dmac(&self) -> USE_INTERNAL_DMAC_R {
        USE_INTERNAL_DMAC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controller reset."]
    #[inline(always)]
    pub fn controller_reset(&mut self) -> CONTROLLER_RESET_W {
        CONTROLLER_RESET_W { w: self }
    }
    #[doc = "Bit 1 - Fifo reset."]
    #[inline(always)]
    pub fn fifo_reset(&mut self) -> FIFO_RESET_W {
        FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 2 - DMA reset."]
    #[inline(always)]
    pub fn dma_reset(&mut self) -> DMA_RESET_W {
        DMA_RESET_W { w: self }
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit."]
    #[inline(always)]
    pub fn int_enable(&mut self) -> INT_ENABLE_W {
        INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 6 - Read/wait."]
    #[inline(always)]
    pub fn read_wait(&mut self) -> READ_WAIT_W {
        READ_WAIT_W { w: self }
    }
    #[doc = "Bit 7 - Send irq response."]
    #[inline(always)]
    pub fn send_irq_response(&mut self) -> SEND_IRQ_RESPONSE_W {
        SEND_IRQ_RESPONSE_W { w: self }
    }
    #[doc = "Bit 8 - Abort read data."]
    #[inline(always)]
    pub fn abort_read_data(&mut self) -> ABORT_READ_DATA_W {
        ABORT_READ_DATA_W { w: self }
    }
    #[doc = "Bit 9 - Send ccsd."]
    #[inline(always)]
    pub fn send_ccsd(&mut self) -> SEND_CCSD_W {
        SEND_CCSD_W { w: self }
    }
    #[doc = "Bit 10 - Send auto stop ccsd."]
    #[inline(always)]
    pub fn send_auto_stop_ccsd(&mut self) -> SEND_AUTO_STOP_CCSD_W {
        SEND_AUTO_STOP_CCSD_W { w: self }
    }
    #[doc = "Bit 11 - CEATA device interrupt status."]
    #[inline(always)]
    pub fn ceata_device_interrupt_status(&mut self) -> CEATA_DEVICE_INTERRUPT_STATUS_W {
        CEATA_DEVICE_INTERRUPT_STATUS_W { w: self }
    }
    #[doc = "Bit 16 - Controls the state of the SD_VOLT0 pin."]
    #[inline(always)]
    pub fn card_voltage_a0(&mut self) -> CARD_VOLTAGE_A0_W {
        CARD_VOLTAGE_A0_W { w: self }
    }
    #[doc = "Bit 17 - Controls the state of the SD_VOLT1 pin."]
    #[inline(always)]
    pub fn card_voltage_a1(&mut self) -> CARD_VOLTAGE_A1_W {
        CARD_VOLTAGE_A1_W { w: self }
    }
    #[doc = "Bit 18 - Controls the state of the SD_VOLT2 pin."]
    #[inline(always)]
    pub fn card_voltage_a2(&mut self) -> CARD_VOLTAGE_A2_W {
        CARD_VOLTAGE_A2_W { w: self }
    }
    #[doc = "Bit 25 - SD/MMC DMA use."]
    #[inline(always)]
    pub fn use_internal_dmac(&mut self) -> USE_INTERNAL_DMAC_W {
        USE_INTERNAL_DMAC_W { w: self }
    }
}
