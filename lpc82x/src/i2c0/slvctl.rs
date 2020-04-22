#[doc = "Reader of register SLVCTL"]
pub type R = crate::R<u32, super::SLVCTL>;
#[doc = "Writer for register SLVCTL"]
pub type W = crate::W<u32, super::SLVCTL>;
#[doc = "Register SLVCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::SLVCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Slave Continue.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVCONTINUE_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Informs the Slave function to continue to the next operation."]
    CONTINUE = 1,
}
impl From<SLVCONTINUE_A> for bool {
    #[inline(always)]
    fn from(variant: SLVCONTINUE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLVCONTINUE`"]
pub type SLVCONTINUE_R = crate::R<bool, SLVCONTINUE_A>;
impl SLVCONTINUE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVCONTINUE_A {
        match self.bits {
            false => SLVCONTINUE_A::NO_EFFECT,
            true => SLVCONTINUE_A::CONTINUE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SLVCONTINUE_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue_(&self) -> bool {
        *self == SLVCONTINUE_A::CONTINUE
    }
}
#[doc = "Write proxy for field `SLVCONTINUE`"]
pub struct SLVCONTINUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVCONTINUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVCONTINUE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SLVCONTINUE_A::NO_EFFECT)
    }
    #[doc = "Informs the Slave function to continue to the next operation."]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(SLVCONTINUE_A::CONTINUE)
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
#[doc = "Slave NACK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNACK_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    NACK = 1,
}
impl From<SLVNACK_A> for bool {
    #[inline(always)]
    fn from(variant: SLVNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLVNACK`"]
pub type SLVNACK_R = crate::R<bool, SLVNACK_A>;
impl SLVNACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVNACK_A {
        match self.bits {
            false => SLVNACK_A::NO_EFFECT,
            true => SLVNACK_A::NACK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SLVNACK_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == SLVNACK_A::NACK
    }
}
#[doc = "Write proxy for field `SLVNACK`"]
pub struct SLVNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVNACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SLVNACK_A::NO_EFFECT)
    }
    #[doc = "NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(SLVNACK_A::NACK)
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
#[doc = "Slave DMA enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDMA_A {
    #[doc = "0: Disabled. No DMA requests are issued for Slave mode operation."]
    DISABLED = 0,
    #[doc = "1: Enabled. DMA requests are issued for I2C slave data transmission and reception."]
    ENABLED = 1,
}
impl From<SLVDMA_A> for bool {
    #[inline(always)]
    fn from(variant: SLVDMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLVDMA`"]
pub type SLVDMA_R = crate::R<bool, SLVDMA_A>;
impl SLVDMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVDMA_A {
        match self.bits {
            false => SLVDMA_A::DISABLED,
            true => SLVDMA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVDMA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVDMA_A::ENABLED
    }
}
#[doc = "Write proxy for field `SLVDMA`"]
pub struct SLVDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVDMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVDMA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. No DMA requests are issued for Slave mode operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVDMA_A::DISABLED)
    }
    #[doc = "Enabled. DMA requests are issued for I2C slave data transmission and reception."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVDMA_A::ENABLED)
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
    #[doc = "Bit 0 - Slave Continue."]
    #[inline(always)]
    pub fn slvcontinue(&self) -> SLVCONTINUE_R {
        SLVCONTINUE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave NACK."]
    #[inline(always)]
    pub fn slvnack(&self) -> SLVNACK_R {
        SLVNACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Slave DMA enable."]
    #[inline(always)]
    pub fn slvdma(&self) -> SLVDMA_R {
        SLVDMA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Continue."]
    #[inline(always)]
    pub fn slvcontinue(&mut self) -> SLVCONTINUE_W {
        SLVCONTINUE_W { w: self }
    }
    #[doc = "Bit 1 - Slave NACK."]
    #[inline(always)]
    pub fn slvnack(&mut self) -> SLVNACK_W {
        SLVNACK_W { w: self }
    }
    #[doc = "Bit 3 - Slave DMA enable."]
    #[inline(always)]
    pub fn slvdma(&mut self) -> SLVDMA_W {
        SLVDMA_W { w: self }
    }
}
