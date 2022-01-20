#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTROLLER_RESET` reader - Controller reset."]
pub struct CONTROLLER_RESET_R(crate::FieldReader<bool, bool>);
impl CONTROLLER_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONTROLLER_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONTROLLER_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTROLLER_RESET` writer - Controller reset."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `FIFO_RESET` reader - Fifo reset."]
pub struct FIFO_RESET_R(crate::FieldReader<bool, bool>);
impl FIFO_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_RESET` writer - Fifo reset."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DMA_RESET` reader - DMA reset."]
pub struct DMA_RESET_R(crate::FieldReader<bool, bool>);
impl DMA_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_RESET` writer - DMA reset."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `INT_ENABLE` reader - Global interrupt enable/disable bit."]
pub struct INT_ENABLE_R(crate::FieldReader<bool, bool>);
impl INT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_ENABLE` writer - Global interrupt enable/disable bit."]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `READ_WAIT` reader - Read/wait."]
pub struct READ_WAIT_R(crate::FieldReader<bool, bool>);
impl READ_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READ_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_WAIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_WAIT` writer - Read/wait."]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `SEND_IRQ_RESPONSE` reader - Send irq response."]
pub struct SEND_IRQ_RESPONSE_R(crate::FieldReader<bool, bool>);
impl SEND_IRQ_RESPONSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_IRQ_RESPONSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_IRQ_RESPONSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_IRQ_RESPONSE` writer - Send irq response."]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ABORT_READ_DATA` reader - Abort read data."]
pub struct ABORT_READ_DATA_R(crate::FieldReader<bool, bool>);
impl ABORT_READ_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABORT_READ_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABORT_READ_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABORT_READ_DATA` writer - Abort read data."]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SEND_CCSD` reader - Send ccsd."]
pub struct SEND_CCSD_R(crate::FieldReader<bool, bool>);
impl SEND_CCSD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_CCSD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_CCSD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_CCSD` writer - Send ccsd."]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SEND_AUTO_STOP_CCSD` reader - Send auto stop ccsd."]
pub struct SEND_AUTO_STOP_CCSD_R(crate::FieldReader<bool, bool>);
impl SEND_AUTO_STOP_CCSD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_AUTO_STOP_CCSD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_AUTO_STOP_CCSD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_AUTO_STOP_CCSD` writer - Send auto stop ccsd."]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CEATA_DEVICE_INTERRUPT_STATUS` reader - CEATA device interrupt status."]
pub struct CEATA_DEVICE_INTERRUPT_STATUS_R(crate::FieldReader<bool, bool>);
impl CEATA_DEVICE_INTERRUPT_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CEATA_DEVICE_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEATA_DEVICE_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEATA_DEVICE_INTERRUPT_STATUS` writer - CEATA device interrupt status."]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CARD_VOLTAGE_A0` reader - Controls the state of the SD_VOLT0 pin."]
pub struct CARD_VOLTAGE_A0_R(crate::FieldReader<bool, bool>);
impl CARD_VOLTAGE_A0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARD_VOLTAGE_A0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_VOLTAGE_A0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_VOLTAGE_A0` writer - Controls the state of the SD_VOLT0 pin."]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CARD_VOLTAGE_A1` reader - Controls the state of the SD_VOLT1 pin."]
pub struct CARD_VOLTAGE_A1_R(crate::FieldReader<bool, bool>);
impl CARD_VOLTAGE_A1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARD_VOLTAGE_A1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_VOLTAGE_A1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_VOLTAGE_A1` writer - Controls the state of the SD_VOLT1 pin."]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CARD_VOLTAGE_A2` reader - Controls the state of the SD_VOLT2 pin."]
pub struct CARD_VOLTAGE_A2_R(crate::FieldReader<bool, bool>);
impl CARD_VOLTAGE_A2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARD_VOLTAGE_A2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_VOLTAGE_A2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_VOLTAGE_A2` writer - Controls the state of the SD_VOLT2 pin."]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `USE_INTERNAL_DMAC` reader - SD/MMC DMA use."]
pub struct USE_INTERNAL_DMAC_R(crate::FieldReader<bool, bool>);
impl USE_INTERNAL_DMAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USE_INTERNAL_DMAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USE_INTERNAL_DMAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USE_INTERNAL_DMAC` writer - SD/MMC DMA use."]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
