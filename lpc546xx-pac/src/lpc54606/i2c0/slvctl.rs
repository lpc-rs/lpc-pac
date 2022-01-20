#[doc = "Register `SLVCTL` reader"]
pub struct R(crate::R<SLVCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLVCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLVCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLVCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLVCTL` writer"]
pub struct W(crate::W<SLVCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLVCTL_SPEC>;
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
impl From<crate::W<SLVCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLVCTL_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `SLVCONTINUE` reader - Slave Continue."]
pub struct SLVCONTINUE_R(crate::FieldReader<bool, SLVCONTINUE_A>);
impl SLVCONTINUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLVCONTINUE_R(crate::FieldReader::new(bits))
    }
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
        **self == SLVCONTINUE_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        **self == SLVCONTINUE_A::CONTINUE
    }
}
impl core::ops::Deref for SLVCONTINUE_R {
    type Target = crate::FieldReader<bool, SLVCONTINUE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVCONTINUE` writer - Slave Continue."]
pub struct SLVCONTINUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVCONTINUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVCONTINUE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
#[doc = "Field `SLVNACK` reader - Slave NACK."]
pub struct SLVNACK_R(crate::FieldReader<bool, SLVNACK_A>);
impl SLVNACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLVNACK_R(crate::FieldReader::new(bits))
    }
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
        **self == SLVNACK_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        **self == SLVNACK_A::NACK
    }
}
impl core::ops::Deref for SLVNACK_R {
    type Target = crate::FieldReader<bool, SLVNACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVNACK` writer - Slave NACK."]
pub struct SLVNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVNACK_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
#[doc = "Field `SLVDMA` reader - Slave DMA enable."]
pub struct SLVDMA_R(crate::FieldReader<bool, SLVDMA_A>);
impl SLVDMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLVDMA_R(crate::FieldReader::new(bits))
    }
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
        **self == SLVDMA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SLVDMA_A::ENABLED
    }
}
impl core::ops::Deref for SLVDMA_R {
    type Target = crate::FieldReader<bool, SLVDMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVDMA` writer - Slave DMA enable."]
pub struct SLVDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVDMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVDMA_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
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
#[doc = "Field `AUTOACK` reader - Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
pub struct AUTOACK_R(crate::FieldReader<bool, AUTOACK_A>);
impl AUTOACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOACK_R(crate::FieldReader::new(bits))
    }
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
        **self == AUTOACK_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC_ACK`"]
    #[inline(always)]
    pub fn is_automatic_ack(&self) -> bool {
        **self == AUTOACK_A::AUTOMATIC_ACK
    }
}
impl core::ops::Deref for AUTOACK_R {
    type Target = crate::FieldReader<bool, AUTOACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOACK` writer - Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
pub struct AUTOACK_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOACK_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
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
#[doc = "Field `AUTOMATCHREAD` reader - When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
pub struct AUTOMATCHREAD_R(crate::FieldReader<bool, AUTOMATCHREAD_A>);
impl AUTOMATCHREAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOMATCHREAD_R(crate::FieldReader::new(bits))
    }
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
        **self == AUTOMATCHREAD_A::I2C_WRITE
    }
    #[doc = "Checks if the value of the field is `I2C_READ`"]
    #[inline(always)]
    pub fn is_i2c_read(&self) -> bool {
        **self == AUTOMATCHREAD_A::I2C_READ
    }
}
impl core::ops::Deref for AUTOMATCHREAD_R {
    type Target = crate::FieldReader<bool, AUTOMATCHREAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOMATCHREAD` writer - When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
pub struct AUTOMATCHREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOMATCHREAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOMATCHREAD_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvctl](index.html) module"]
pub struct SLVCTL_SPEC;
impl crate::RegisterSpec for SLVCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slvctl::R](R) reader structure"]
impl crate::Readable for SLVCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slvctl::W](W) writer structure"]
impl crate::Writable for SLVCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLVCTL to value 0"]
impl crate::Resettable for SLVCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
