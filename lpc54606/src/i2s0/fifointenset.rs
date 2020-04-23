#[doc = "Reader of register FIFOINTENSET"]
pub type R = crate::R<u32, super::FIFOINTENSET>;
#[doc = "Writer for register FIFOINTENSET"]
pub type W = crate::W<u32, super::FIFOINTENSET>;
#[doc = "Register FIFOINTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOINTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXERR_A {
    #[doc = "0: No interrupt will be generated for a transmit error."]
    DISABLED = 0,
    #[doc = "1: An interrupt will be generated when a transmit error occurs."]
    ENABLED = 1,
}
impl From<TXERR_A> for bool {
    #[inline(always)]
    fn from(variant: TXERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXERR`"]
pub type TXERR_R = crate::R<bool, TXERR_A>;
impl TXERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXERR_A {
        match self.bits {
            false => TXERR_A::DISABLED,
            true => TXERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXERR_A::ENABLED
    }
}
#[doc = "Write proxy for field `TXERR`"]
pub struct TXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt will be generated for a transmit error."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXERR_A::DISABLED)
    }
    #[doc = "An interrupt will be generated when a transmit error occurs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXERR_A::ENABLED)
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
#[doc = "Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERR_A {
    #[doc = "0: No interrupt will be generated for a receive error."]
    DISABLED = 0,
    #[doc = "1: An interrupt will be generated when a receive error occurs."]
    ENABLED = 1,
}
impl From<RXERR_A> for bool {
    #[inline(always)]
    fn from(variant: RXERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXERR`"]
pub type RXERR_R = crate::R<bool, RXERR_A>;
impl RXERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXERR_A {
        match self.bits {
            false => RXERR_A::DISABLED,
            true => RXERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXERR_A::ENABLED
    }
}
#[doc = "Write proxy for field `RXERR`"]
pub struct RXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt will be generated for a receive error."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXERR_A::DISABLED)
    }
    #[doc = "An interrupt will be generated when a receive error occurs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXERR_A::ENABLED)
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
#[doc = "Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLVL_A {
    #[doc = "0: No interrupt will be generated based on the TX FIFO level."]
    DISABLED = 0,
    #[doc = "1: If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register."]
    ENABLED = 1,
}
impl From<TXLVL_A> for bool {
    #[inline(always)]
    fn from(variant: TXLVL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXLVL`"]
pub type TXLVL_R = crate::R<bool, TXLVL_A>;
impl TXLVL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXLVL_A {
        match self.bits {
            false => TXLVL_A::DISABLED,
            true => TXLVL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXLVL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXLVL_A::ENABLED
    }
}
#[doc = "Write proxy for field `TXLVL`"]
pub struct TXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXLVL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt will be generated based on the TX FIFO level."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXLVL_A::DISABLED)
    }
    #[doc = "If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXLVL_A::ENABLED)
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
#[doc = "Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLVL_A {
    #[doc = "0: No interrupt will be generated based on the RX FIFO level."]
    DISABLED = 0,
    #[doc = "1: If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register."]
    ENABLED = 1,
}
impl From<RXLVL_A> for bool {
    #[inline(always)]
    fn from(variant: RXLVL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXLVL`"]
pub type RXLVL_R = crate::R<bool, RXLVL_A>;
impl RXLVL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXLVL_A {
        match self.bits {
            false => RXLVL_A::DISABLED,
            true => RXLVL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXLVL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXLVL_A::ENABLED
    }
}
#[doc = "Write proxy for field `RXLVL`"]
pub struct RXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXLVL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt will be generated based on the RX FIFO level."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXLVL_A::DISABLED)
    }
    #[doc = "If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXLVL_A::ENABLED)
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
    #[doc = "Bit 0 - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W {
        TXERR_W { w: self }
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RXERR_W {
        RXERR_W { w: self }
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn txlvl(&mut self) -> TXLVL_W {
        TXLVL_W { w: self }
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn rxlvl(&mut self) -> RXLVL_W {
        RXLVL_W { w: self }
    }
}
