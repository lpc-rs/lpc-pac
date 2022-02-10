///Register `FIFOINTENSET` reader
pub struct R(crate::R<FIFOINTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOINTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOINTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOINTENSET_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FIFOINTENSET` writer
pub struct W(crate::W<FIFOINTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOINTENSET_SPEC>;
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
impl From<crate::W<FIFOINTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOINTENSET_SPEC>) -> Self {
        W(writer)
    }
}
///Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXERR_A {
    ///0: No interrupt will be generated for a transmit error.
    DISABLED = 0,
    ///1: An interrupt will be generated when a transmit error occurs.
    ENABLED = 1,
}
impl From<TXERR_A> for bool {
    #[inline(always)]
    fn from(variant: TXERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXERR` reader - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register.
pub struct TXERR_R(crate::FieldReader<bool, TXERR_A>);
impl TXERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXERR_A {
        match self.bits {
            false => TXERR_A::DISABLED,
            true => TXERR_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXERR_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXERR_A::ENABLED
    }
}
impl core::ops::Deref for TXERR_R {
    type Target = crate::FieldReader<bool, TXERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXERR` writer - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register.
pub struct TXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No interrupt will be generated for a transmit error.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXERR_A::DISABLED)
    }
    ///An interrupt will be generated when a transmit error occurs.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXERR_A::ENABLED)
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
///Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERR_A {
    ///0: No interrupt will be generated for a receive error.
    DISABLED = 0,
    ///1: An interrupt will be generated when a receive error occurs.
    ENABLED = 1,
}
impl From<RXERR_A> for bool {
    #[inline(always)]
    fn from(variant: RXERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXERR` reader - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register.
pub struct RXERR_R(crate::FieldReader<bool, RXERR_A>);
impl RXERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXERR_A {
        match self.bits {
            false => RXERR_A::DISABLED,
            true => RXERR_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXERR_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXERR_A::ENABLED
    }
}
impl core::ops::Deref for RXERR_R {
    type Target = crate::FieldReader<bool, RXERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXERR` writer - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register.
pub struct RXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No interrupt will be generated for a receive error.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXERR_A::DISABLED)
    }
    ///An interrupt will be generated when a receive error occurs.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXERR_A::ENABLED)
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
///Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLVL_A {
    ///0: No interrupt will be generated based on the TX FIFO level.
    DISABLED = 0,
    ///1: If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register.
    ENABLED = 1,
}
impl From<TXLVL_A> for bool {
    #[inline(always)]
    fn from(variant: TXLVL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXLVL` reader - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.
pub struct TXLVL_R(crate::FieldReader<bool, TXLVL_A>);
impl TXLVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXLVL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXLVL_A {
        match self.bits {
            false => TXLVL_A::DISABLED,
            true => TXLVL_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXLVL_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXLVL_A::ENABLED
    }
}
impl core::ops::Deref for TXLVL_R {
    type Target = crate::FieldReader<bool, TXLVL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXLVL` writer - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.
pub struct TXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLVL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXLVL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No interrupt will be generated based on the TX FIFO level.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXLVL_A::DISABLED)
    }
    ///If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXLVL_A::ENABLED)
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
///Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLVL_A {
    ///0: No interrupt will be generated based on the RX FIFO level.
    DISABLED = 0,
    ///1: If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register.
    ENABLED = 1,
}
impl From<RXLVL_A> for bool {
    #[inline(always)]
    fn from(variant: RXLVL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXLVL` reader - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.
pub struct RXLVL_R(crate::FieldReader<bool, RXLVL_A>);
impl RXLVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXLVL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXLVL_A {
        match self.bits {
            false => RXLVL_A::DISABLED,
            true => RXLVL_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXLVL_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXLVL_A::ENABLED
    }
}
impl core::ops::Deref for RXLVL_R {
    type Target = crate::FieldReader<bool, RXLVL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXLVL` writer - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.
pub struct RXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLVL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXLVL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No interrupt will be generated based on the RX FIFO level.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXLVL_A::DISABLED)
    }
    ///If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXLVL_A::ENABLED)
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
    ///Bit 0 - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register.
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register.
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register.
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W {
        TXERR_W { w: self }
    }
    ///Bit 1 - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register.
    #[inline(always)]
    pub fn rxerr(&mut self) -> RXERR_W {
        RXERR_W { w: self }
    }
    ///Bit 2 - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.
    #[inline(always)]
    pub fn txlvl(&mut self) -> TXLVL_W {
        TXLVL_W { w: self }
    }
    ///Bit 3 - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.
    #[inline(always)]
    pub fn rxlvl(&mut self) -> RXLVL_W {
        RXLVL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FIFO interrupt enable set (enable) and read register.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fifointenset](index.html) module
pub struct FIFOINTENSET_SPEC;
impl crate::RegisterSpec for FIFOINTENSET_SPEC {
    type Ux = u32;
}
///`read()` method returns [fifointenset::R](R) reader structure
impl crate::Readable for FIFOINTENSET_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fifointenset::W](W) writer structure
impl crate::Writable for FIFOINTENSET_SPEC {
    type Writer = W;
}
///`reset()` method sets FIFOINTENSET to value 0
impl crate::Resettable for FIFOINTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
