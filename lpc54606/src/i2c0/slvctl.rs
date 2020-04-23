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
    #[doc = "1: Continue. Informs the Slave function to continue to the next operation, by clearing the SLVPENDING flag in the STAT register. This must be done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation. Automatic Operation has different requirements. SLVCONTINUE should not be set unless SLVPENDING = 1."]
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
    #[doc = "Continue. Informs the Slave function to continue to the next operation, by clearing the SLVPENDING flag in the STAT register. This must be done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation. Automatic Operation has different requirements. SLVCONTINUE should not be set unless SLVPENDING = 1."]
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
#[doc = "Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOACK_A {
    #[doc = "0: Normal, non-automatic operation. If AUTONACK = 0, an SlvPending interrupt is generated when a matching address is received. If AUTONACK = 1, received addresses are NACKed (ignored)."]
    NORMAL = 0,
    #[doc = "1: A header with matching SLVADR0 and matching direction as set by AUTOMATCHREAD will be ACKed immediately, allowing the master to move on to the data bytes. If the address matches SLVADR0, but the direction does not match AUTOMATCHREAD, the behavior will depend on the AUTONACK bit in the SLVADR0 register: if AUTONACK is set, then it will be Nacked; else if AUTONACK is clear, then a SlvPending interrupt is generated."]
    AUTOMATIC_ACK = 1,
}
impl From<AUTOACK_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTOACK`"]
pub type AUTOACK_R = crate::R<bool, AUTOACK_A>;
impl AUTOACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOACK_A {
        match self.bits {
            false => AUTOACK_A::NORMAL,
            true => AUTOACK_A::AUTOMATIC_ACK,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == AUTOACK_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC_ACK`"]
    #[inline(always)]
    pub fn is_automatic_ack(&self) -> bool {
        *self == AUTOACK_A::AUTOMATIC_ACK
    }
}
#[doc = "Write proxy for field `AUTOACK`"]
pub struct AUTOACK_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal, non-automatic operation. If AUTONACK = 0, an SlvPending interrupt is generated when a matching address is received. If AUTONACK = 1, received addresses are NACKed (ignored)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(AUTOACK_A::NORMAL)
    }
    #[doc = "A header with matching SLVADR0 and matching direction as set by AUTOMATCHREAD will be ACKed immediately, allowing the master to move on to the data bytes. If the address matches SLVADR0, but the direction does not match AUTOMATCHREAD, the behavior will depend on the AUTONACK bit in the SLVADR0 register: if AUTONACK is set, then it will be Nacked; else if AUTONACK is clear, then a SlvPending interrupt is generated."]
    #[inline(always)]
    pub fn automatic_ack(self) -> &'a mut W {
        self.variant(AUTOACK_A::AUTOMATIC_ACK)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOMATCHREAD_A {
    #[doc = "0: The expected next operation in Automatic Mode is an I2C write."]
    I2C_WRITE = 0,
    #[doc = "1: The expected next operation in Automatic Mode is an I2C read."]
    I2C_READ = 1,
}
impl From<AUTOMATCHREAD_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOMATCHREAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTOMATCHREAD`"]
pub type AUTOMATCHREAD_R = crate::R<bool, AUTOMATCHREAD_A>;
impl AUTOMATCHREAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOMATCHREAD_A {
        match self.bits {
            false => AUTOMATCHREAD_A::I2C_WRITE,
            true => AUTOMATCHREAD_A::I2C_READ,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_WRITE`"]
    #[inline(always)]
    pub fn is_i2c_write(&self) -> bool {
        *self == AUTOMATCHREAD_A::I2C_WRITE
    }
    #[doc = "Checks if the value of the field is `I2C_READ`"]
    #[inline(always)]
    pub fn is_i2c_read(&self) -> bool {
        *self == AUTOMATCHREAD_A::I2C_READ
    }
}
#[doc = "Write proxy for field `AUTOMATCHREAD`"]
pub struct AUTOMATCHREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOMATCHREAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOMATCHREAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The expected next operation in Automatic Mode is an I2C write."]
    #[inline(always)]
    pub fn i2c_write(self) -> &'a mut W {
        self.variant(AUTOMATCHREAD_A::I2C_WRITE)
    }
    #[doc = "The expected next operation in Automatic Mode is an I2C read."]
    #[inline(always)]
    pub fn i2c_read(self) -> &'a mut W {
        self.variant(AUTOMATCHREAD_A::I2C_READ)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
    #[doc = "Bit 8 - Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
    #[inline(always)]
    pub fn autoack(&self) -> AUTOACK_R {
        AUTOACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
    #[inline(always)]
    pub fn automatchread(&self) -> AUTOMATCHREAD_R {
        AUTOMATCHREAD_R::new(((self.bits >> 9) & 0x01) != 0)
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
    #[doc = "Bit 8 - Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
    #[inline(always)]
    pub fn autoack(&mut self) -> AUTOACK_W {
        AUTOACK_W { w: self }
    }
    #[doc = "Bit 9 - When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
    #[inline(always)]
    pub fn automatchread(&mut self) -> AUTOMATCHREAD_W {
        AUTOMATCHREAD_W { w: self }
    }
}
