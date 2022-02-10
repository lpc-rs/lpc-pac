///Register `FIFO_CTRL` reader
pub struct R(crate::R<FIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FIFO_CTRL` writer
pub struct W(crate::W<FIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CTRL_SPEC>;
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
impl From<crate::W<FIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
///FIFO enable.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    ///0: FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed.
    DISABLED = 0,
    ///1: FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register.
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - FIFO enable.
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENABLE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENABLE_A::ENABLED
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ENABLE` writer - FIFO enable.
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    ///FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///FIFO reset.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETN_A {
    ///0: Reset the FIFO.
    RESET = 0,
    ///1: Normal operation
    NORMAL = 1,
}
impl From<RESETN_A> for bool {
    #[inline(always)]
    fn from(variant: RESETN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RESETN` reader - FIFO reset.
pub struct RESETN_R(crate::FieldReader<bool, RESETN_A>);
impl RESETN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESETN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RESETN_A {
        match self.bits {
            false => RESETN_A::RESET,
            true => RESETN_A::NORMAL,
        }
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == RESETN_A::RESET
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == RESETN_A::NORMAL
    }
}
impl core::ops::Deref for RESETN_R {
    type Target = crate::FieldReader<bool, RESETN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RESETN` writer - FIFO reset.
pub struct RESETN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RESETN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Reset the FIFO.
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESETN_A::RESET)
    }
    ///Normal operation
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RESETN_A::NORMAL)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Interrupt enable.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    ///0: FIFO level interrupts are not enabled.
    DISABLED = 0,
    ///1: FIFO level interrupts are enabled.
    ENABLED = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INTEN` reader - Interrupt enable.
pub struct INTEN_R(crate::FieldReader<bool, INTEN_A>);
impl INTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::DISABLED,
            true => INTEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == INTEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == INTEN_A::ENABLED
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<bool, INTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INTEN` writer - Interrupt enable.
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: INTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///FIFO level interrupts are not enabled.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN_A::DISABLED)
    }
    ///FIFO level interrupts are enabled.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///DMA enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    ///0: DMA requests are not enabled.
    DISABLED = 0,
    ///1: DMA requests based on FIFO level are enabled.
    ENABLED = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - DMA enable
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLED,
            true => DMAEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAEN_A::ENABLED
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAEN` writer - DMA enable
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA requests are not enabled.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLED)
    }
    ///DMA requests based on FIFO level are enabled.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Field `TRIGLVL` reader - FIFO trigger level. Selects the data trigger level for interrupt or DMA operation. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode See Section 4.5.66 'Hardware Wake-up control register'. 0 = trigger when the FIFO has received one entry (is no longer empty). 1 = trigger when the FIFO has received two entries. 15 = trigger when the FIFO has received 16 entries (has become full).
pub struct TRIGLVL_R(crate::FieldReader<u8, u8>);
impl TRIGLVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRIGLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGLVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRIGLVL` writer - FIFO trigger level. Selects the data trigger level for interrupt or DMA operation. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode See Section 4.5.66 'Hardware Wake-up control register'. 0 = trigger when the FIFO has received one entry (is no longer empty). 1 = trigger when the FIFO has received two entries. 15 = trigger when the FIFO has received 16 entries (has become full).
pub struct TRIGLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGLVL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - FIFO enable.
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - FIFO reset.
    #[inline(always)]
    pub fn resetn(&self) -> RESETN_R {
        RESETN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Interrupt enable.
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - DMA enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 16:20 - FIFO trigger level. Selects the data trigger level for interrupt or DMA operation. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode See Section 4.5.66 'Hardware Wake-up control register'. 0 = trigger when the FIFO has received one entry (is no longer empty). 1 = trigger when the FIFO has received two entries. 15 = trigger when the FIFO has received 16 entries (has become full).
    #[inline(always)]
    pub fn triglvl(&self) -> TRIGLVL_R {
        TRIGLVL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    ///Bit 0 - FIFO enable.
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    ///Bit 1 - FIFO reset.
    #[inline(always)]
    pub fn resetn(&mut self) -> RESETN_W {
        RESETN_W { w: self }
    }
    ///Bit 2 - Interrupt enable.
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    ///Bit 3 - DMA enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    ///Bits 16:20 - FIFO trigger level. Selects the data trigger level for interrupt or DMA operation. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode See Section 4.5.66 'Hardware Wake-up control register'. 0 = trigger when the FIFO has received one entry (is no longer empty). 1 = trigger when the FIFO has received two entries. 15 = trigger when the FIFO has received 16 entries (has become full).
    #[inline(always)]
    pub fn triglvl(&mut self) -> TRIGLVL_W {
        TRIGLVL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FIFO Control register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fifo_ctrl](index.html) module
pub struct FIFO_CTRL_SPEC;
impl crate::RegisterSpec for FIFO_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [fifo_ctrl::R](R) reader structure
impl crate::Readable for FIFO_CTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fifo_ctrl::W](W) writer structure
impl crate::Writable for FIFO_CTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets FIFO_CTRL to value 0
impl crate::Resettable for FIFO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
