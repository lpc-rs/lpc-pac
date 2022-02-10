///Register `MSTCTL` reader
pub struct R(crate::R<MSTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTCTL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MSTCTL` writer
pub struct W(crate::W<MSTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTCTL_SPEC>;
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
impl From<crate::W<MSTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTCTL_SPEC>) -> Self {
        W(writer)
    }
}
///Master Continue. This bit is write-only.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTCONTINUE_AW {
    ///0: No effect.
    NO_EFFECT = 0,
    ///1: Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation.
    CONTINUE = 1,
}
impl From<MSTCONTINUE_AW> for bool {
    #[inline(always)]
    fn from(variant: MSTCONTINUE_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTCONTINUE` writer - Master Continue. This bit is write-only.
pub struct MSTCONTINUE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCONTINUE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSTCONTINUE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect.
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCONTINUE_AW::NO_EFFECT)
    }
    ///Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation.
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(MSTCONTINUE_AW::CONTINUE)
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
///Master Start control. This bit is write-only.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTART_A {
    ///0: No effect.
    NO_EFFECT = 0,
    ///1: Start. A Start will be generated on the I2C bus at the next allowed time.
    START = 1,
}
impl From<MSTSTART_A> for bool {
    #[inline(always)]
    fn from(variant: MSTSTART_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTSTART` reader - Master Start control. This bit is write-only.
pub struct MSTSTART_R(crate::FieldReader<bool, MSTSTART_A>);
impl MSTSTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSTSTART_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSTSTART_A {
        match self.bits {
            false => MSTSTART_A::NO_EFFECT,
            true => MSTSTART_A::START,
        }
    }
    ///Checks if the value of the field is `NO_EFFECT`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == MSTSTART_A::NO_EFFECT
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == MSTSTART_A::START
    }
}
impl core::ops::Deref for MSTSTART_R {
    type Target = crate::FieldReader<bool, MSTSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MSTSTART` writer - Master Start control. This bit is write-only.
pub struct MSTSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSTART_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSTSTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect.
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTSTART_A::NO_EFFECT)
    }
    ///Start. A Start will be generated on the I2C bus at the next allowed time.
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(MSTSTART_A::START)
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
///Master Stop control. This bit is write-only.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTOP_A {
    ///0: No effect.
    NO_EFFECT = 0,
    ///1: Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode).
    STOP = 1,
}
impl From<MSTSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: MSTSTOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTSTOP` reader - Master Stop control. This bit is write-only.
pub struct MSTSTOP_R(crate::FieldReader<bool, MSTSTOP_A>);
impl MSTSTOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSTSTOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSTSTOP_A {
        match self.bits {
            false => MSTSTOP_A::NO_EFFECT,
            true => MSTSTOP_A::STOP,
        }
    }
    ///Checks if the value of the field is `NO_EFFECT`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == MSTSTOP_A::NO_EFFECT
    }
    ///Checks if the value of the field is `STOP`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == MSTSTOP_A::STOP
    }
}
impl core::ops::Deref for MSTSTOP_R {
    type Target = crate::FieldReader<bool, MSTSTOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MSTSTOP` writer - Master Stop control. This bit is write-only.
pub struct MSTSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSTOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSTSTOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect.
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTSTOP_A::NO_EFFECT)
    }
    ///Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode).
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(MSTSTOP_A::STOP)
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
///Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTDMA_A {
    ///0: Disable. No DMA requests are generated for master operation.
    DISABLED = 0,
    ///1: Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically.
    ENABLED = 1,
}
impl From<MSTDMA_A> for bool {
    #[inline(always)]
    fn from(variant: MSTDMA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTDMA` reader - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write.
pub struct MSTDMA_R(crate::FieldReader<bool, MSTDMA_A>);
impl MSTDMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSTDMA_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSTDMA_A {
        match self.bits {
            false => MSTDMA_A::DISABLED,
            true => MSTDMA_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MSTDMA_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MSTDMA_A::ENABLED
    }
}
impl core::ops::Deref for MSTDMA_R {
    type Target = crate::FieldReader<bool, MSTDMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MSTDMA` writer - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write.
pub struct MSTDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTDMA_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSTDMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable. No DMA requests are generated for master operation.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTDMA_A::DISABLED)
    }
    ///Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTDMA_A::ENABLED)
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
impl R {
    ///Bit 1 - Master Start control. This bit is write-only.
    #[inline(always)]
    pub fn mststart(&self) -> MSTSTART_R {
        MSTSTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Master Stop control. This bit is write-only.
    #[inline(always)]
    pub fn mststop(&self) -> MSTSTOP_R {
        MSTSTOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write.
    #[inline(always)]
    pub fn mstdma(&self) -> MSTDMA_R {
        MSTDMA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Master Continue. This bit is write-only.
    #[inline(always)]
    pub fn mstcontinue(&mut self) -> MSTCONTINUE_W {
        MSTCONTINUE_W { w: self }
    }
    ///Bit 1 - Master Start control. This bit is write-only.
    #[inline(always)]
    pub fn mststart(&mut self) -> MSTSTART_W {
        MSTSTART_W { w: self }
    }
    ///Bit 2 - Master Stop control. This bit is write-only.
    #[inline(always)]
    pub fn mststop(&mut self) -> MSTSTOP_W {
        MSTSTOP_W { w: self }
    }
    ///Bit 3 - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write.
    #[inline(always)]
    pub fn mstdma(&mut self) -> MSTDMA_W {
        MSTDMA_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Master control register.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mstctl](index.html) module
pub struct MSTCTL_SPEC;
impl crate::RegisterSpec for MSTCTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [mstctl::R](R) reader structure
impl crate::Readable for MSTCTL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mstctl::W](W) writer structure
impl crate::Writable for MSTCTL_SPEC {
    type Writer = W;
}
///`reset()` method sets MSTCTL to value 0
impl crate::Resettable for MSTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
