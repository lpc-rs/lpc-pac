#[doc = "Reader of register MSTCTL"]
pub type R = crate::R<u32, super::MSTCTL>;
#[doc = "Writer for register MSTCTL"]
pub type W = crate::W<u32, super::MSTCTL>;
#[doc = "Register MSTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::MSTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Master Continue. This bit is write-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTCONTINUE_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    CONTINUE = 1,
}
impl From<MSTCONTINUE_AW> for bool {
    #[inline(always)]
    fn from(variant: MSTCONTINUE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `MSTCONTINUE`"]
pub struct MSTCONTINUE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCONTINUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCONTINUE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCONTINUE_AW::NO_EFFECT)
    }
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(MSTCONTINUE_AW::CONTINUE)
    }
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
#[doc = "Master Start control. This bit is write-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTART_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Start. A Start will be generated on the I2C bus at the next allowed time."]
    START = 1,
}
impl From<MSTSTART_A> for bool {
    #[inline(always)]
    fn from(variant: MSTSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTSTART`"]
pub type MSTSTART_R = crate::R<bool, MSTSTART_A>;
impl MSTSTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSTART_A {
        match self.bits {
            false => MSTSTART_A::NO_EFFECT,
            true => MSTSTART_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTSTART_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == MSTSTART_A::START
    }
}
#[doc = "Write proxy for field `MSTSTART`"]
pub struct MSTSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTSTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTSTART_A::NO_EFFECT)
    }
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(MSTSTART_A::START)
    }
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
#[doc = "Master Stop control. This bit is write-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTOP_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    STOP = 1,
}
impl From<MSTSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: MSTSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTSTOP`"]
pub type MSTSTOP_R = crate::R<bool, MSTSTOP_A>;
impl MSTSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSTOP_A {
        match self.bits {
            false => MSTSTOP_A::NO_EFFECT,
            true => MSTSTOP_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTSTOP_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == MSTSTOP_A::STOP
    }
}
#[doc = "Write proxy for field `MSTSTOP`"]
pub struct MSTSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTSTOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTSTOP_A::NO_EFFECT)
    }
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(MSTSTOP_A::STOP)
    }
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
#[doc = "Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTDMA_A {
    #[doc = "0: Disable. No DMA requests are generated for master operation."]
    DISABLED = 0,
    #[doc = "1: Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    ENABLED = 1,
}
impl From<MSTDMA_A> for bool {
    #[inline(always)]
    fn from(variant: MSTDMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTDMA`"]
pub type MSTDMA_R = crate::R<bool, MSTDMA_A>;
impl MSTDMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTDMA_A {
        match self.bits {
            false => MSTDMA_A::DISABLED,
            true => MSTDMA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTDMA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTDMA_A::ENABLED
    }
}
#[doc = "Write proxy for field `MSTDMA`"]
pub struct MSTDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTDMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTDMA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable. No DMA requests are generated for master operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTDMA_A::DISABLED)
    }
    #[doc = "Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTDMA_A::ENABLED)
    }
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
impl R {
    #[doc = "Bit 1 - Master Start control. This bit is write-only."]
    #[inline(always)]
    pub fn mststart(&self) -> MSTSTART_R {
        MSTSTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Master Stop control. This bit is write-only."]
    #[inline(always)]
    pub fn mststop(&self) -> MSTSTOP_R {
        MSTSTOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline(always)]
    pub fn mstdma(&self) -> MSTDMA_R {
        MSTDMA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Continue. This bit is write-only."]
    #[inline(always)]
    pub fn mstcontinue(&mut self) -> MSTCONTINUE_W {
        MSTCONTINUE_W { w: self }
    }
    #[doc = "Bit 1 - Master Start control. This bit is write-only."]
    #[inline(always)]
    pub fn mststart(&mut self) -> MSTSTART_W {
        MSTSTART_W { w: self }
    }
    #[doc = "Bit 2 - Master Stop control. This bit is write-only."]
    #[inline(always)]
    pub fn mststop(&mut self) -> MSTSTOP_W {
        MSTSTOP_W { w: self }
    }
    #[doc = "Bit 3 - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline(always)]
    pub fn mstdma(&mut self) -> MSTDMA_W {
        MSTDMA_W { w: self }
    }
}
