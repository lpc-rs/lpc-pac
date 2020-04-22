#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u32, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u32, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Determines whether an interrupt occurs when receiver data is available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDYEN_A {
    #[doc = "0: No interrupt will be generated when receiver data is available."]
    RXRDYEN_0 = 0,
    #[doc = "1: An interrupt will be generated when receiver data is available in the RXDAT register."]
    RXRDYEN_1 = 1,
}
impl From<RXRDYEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXRDYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXRDYEN`"]
pub type RXRDYEN_R = crate::R<bool, RXRDYEN_A>;
impl RXRDYEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRDYEN_A {
        match self.bits {
            false => RXRDYEN_A::RXRDYEN_0,
            true => RXRDYEN_A::RXRDYEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXRDYEN_0`"]
    #[inline(always)]
    pub fn is_rxrdyen_0(&self) -> bool {
        *self == RXRDYEN_A::RXRDYEN_0
    }
    #[doc = "Checks if the value of the field is `RXRDYEN_1`"]
    #[inline(always)]
    pub fn is_rxrdyen_1(&self) -> bool {
        *self == RXRDYEN_A::RXRDYEN_1
    }
}
#[doc = "Write proxy for field `RXRDYEN`"]
pub struct RXRDYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRDYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXRDYEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt will be generated when receiver data is available."]
    #[inline(always)]
    pub fn rxrdyen_0(self) -> &'a mut W {
        self.variant(RXRDYEN_A::RXRDYEN_0)
    }
    #[doc = "An interrupt will be generated when receiver data is available in the RXDAT register."]
    #[inline(always)]
    pub fn rxrdyen_1(self) -> &'a mut W {
        self.variant(RXRDYEN_A::RXRDYEN_1)
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
#[doc = "Determines whether an interrupt occurs when the transmitter holding register is available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDYEN_A {
    #[doc = "0: No interrupt will be generated when the transmitter holding register is available."]
    TXRDYEN_0 = 0,
    #[doc = "1: An interrupt will be generated when data may be written to TXDAT."]
    TXRDYEN_1 = 1,
}
impl From<TXRDYEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXRDYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXRDYEN`"]
pub type TXRDYEN_R = crate::R<bool, TXRDYEN_A>;
impl TXRDYEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRDYEN_A {
        match self.bits {
            false => TXRDYEN_A::TXRDYEN_0,
            true => TXRDYEN_A::TXRDYEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXRDYEN_0`"]
    #[inline(always)]
    pub fn is_txrdyen_0(&self) -> bool {
        *self == TXRDYEN_A::TXRDYEN_0
    }
    #[doc = "Checks if the value of the field is `TXRDYEN_1`"]
    #[inline(always)]
    pub fn is_txrdyen_1(&self) -> bool {
        *self == TXRDYEN_A::TXRDYEN_1
    }
}
#[doc = "Write proxy for field `TXRDYEN`"]
pub struct TXRDYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRDYEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt will be generated when the transmitter holding register is available."]
    #[inline(always)]
    pub fn txrdyen_0(self) -> &'a mut W {
        self.variant(TXRDYEN_A::TXRDYEN_0)
    }
    #[doc = "An interrupt will be generated when data may be written to TXDAT."]
    #[inline(always)]
    pub fn txrdyen_1(self) -> &'a mut W {
        self.variant(TXRDYEN_A::TXRDYEN_1)
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
#[doc = "Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVEN_A {
    #[doc = "0: No interrupt will be generated when a receiver overrun occurs."]
    RXOVEN_0 = 0,
    #[doc = "1: An interrupt will be generated if a receiver overrun occurs."]
    RXOVEN_1 = 1,
}
impl From<RXOVEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXOVEN`"]
pub type RXOVEN_R = crate::R<bool, RXOVEN_A>;
impl RXOVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOVEN_A {
        match self.bits {
            false => RXOVEN_A::RXOVEN_0,
            true => RXOVEN_A::RXOVEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXOVEN_0`"]
    #[inline(always)]
    pub fn is_rxoven_0(&self) -> bool {
        *self == RXOVEN_A::RXOVEN_0
    }
    #[doc = "Checks if the value of the field is `RXOVEN_1`"]
    #[inline(always)]
    pub fn is_rxoven_1(&self) -> bool {
        *self == RXOVEN_A::RXOVEN_1
    }
}
#[doc = "Write proxy for field `RXOVEN`"]
pub struct RXOVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt will be generated when a receiver overrun occurs."]
    #[inline(always)]
    pub fn rxoven_0(self) -> &'a mut W {
        self.variant(RXOVEN_A::RXOVEN_0)
    }
    #[doc = "An interrupt will be generated if a receiver overrun occurs."]
    #[inline(always)]
    pub fn rxoven_1(self) -> &'a mut W {
        self.variant(RXOVEN_A::RXOVEN_1)
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
#[doc = "Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUREN_A {
    #[doc = "0: No interrupt will be generated when the transmitter underruns."]
    TXUREN_0 = 0,
    #[doc = "1: An interrupt will be generated if the transmitter underruns."]
    TXUREN_1 = 1,
}
impl From<TXUREN_A> for bool {
    #[inline(always)]
    fn from(variant: TXUREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXUREN`"]
pub type TXUREN_R = crate::R<bool, TXUREN_A>;
impl TXUREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUREN_A {
        match self.bits {
            false => TXUREN_A::TXUREN_0,
            true => TXUREN_A::TXUREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXUREN_0`"]
    #[inline(always)]
    pub fn is_txuren_0(&self) -> bool {
        *self == TXUREN_A::TXUREN_0
    }
    #[doc = "Checks if the value of the field is `TXUREN_1`"]
    #[inline(always)]
    pub fn is_txuren_1(&self) -> bool {
        *self == TXUREN_A::TXUREN_1
    }
}
#[doc = "Write proxy for field `TXUREN`"]
pub struct TXUREN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXUREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt will be generated when the transmitter underruns."]
    #[inline(always)]
    pub fn txuren_0(self) -> &'a mut W {
        self.variant(TXUREN_A::TXUREN_0)
    }
    #[doc = "An interrupt will be generated if the transmitter underruns."]
    #[inline(always)]
    pub fn txuren_1(self) -> &'a mut W {
        self.variant(TXUREN_A::TXUREN_1)
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
#[doc = "Determines whether an interrupt occurs when the Slave Select is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSAEN_A {
    #[doc = "0: No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    SSAEN_0 = 0,
    #[doc = "1: An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    SSAEN_1 = 1,
}
impl From<SSAEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSAEN`"]
pub type SSAEN_R = crate::R<bool, SSAEN_A>;
impl SSAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSAEN_A {
        match self.bits {
            false => SSAEN_A::SSAEN_0,
            true => SSAEN_A::SSAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSAEN_0`"]
    #[inline(always)]
    pub fn is_ssaen_0(&self) -> bool {
        *self == SSAEN_A::SSAEN_0
    }
    #[doc = "Checks if the value of the field is `SSAEN_1`"]
    #[inline(always)]
    pub fn is_ssaen_1(&self) -> bool {
        *self == SSAEN_A::SSAEN_1
    }
}
#[doc = "Write proxy for field `SSAEN`"]
pub struct SSAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline(always)]
    pub fn ssaen_0(self) -> &'a mut W {
        self.variant(SSAEN_A::SSAEN_0)
    }
    #[doc = "An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline(always)]
    pub fn ssaen_1(self) -> &'a mut W {
        self.variant(SSAEN_A::SSAEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Determines whether an interrupt occurs when the Slave Select is deasserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDEN_A {
    #[doc = "0: No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    SSDEN_0 = 0,
    #[doc = "1: An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    SSDEN_1 = 1,
}
impl From<SSDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSDEN`"]
pub type SSDEN_R = crate::R<bool, SSDEN_A>;
impl SSDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSDEN_A {
        match self.bits {
            false => SSDEN_A::SSDEN_0,
            true => SSDEN_A::SSDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSDEN_0`"]
    #[inline(always)]
    pub fn is_ssden_0(&self) -> bool {
        *self == SSDEN_A::SSDEN_0
    }
    #[doc = "Checks if the value of the field is `SSDEN_1`"]
    #[inline(always)]
    pub fn is_ssden_1(&self) -> bool {
        *self == SSDEN_A::SSDEN_1
    }
}
#[doc = "Write proxy for field `SSDEN`"]
pub struct SSDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline(always)]
    pub fn ssden_0(self) -> &'a mut W {
        self.variant(SSDEN_A::SSDEN_0)
    }
    #[doc = "An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline(always)]
    pub fn ssden_1(self) -> &'a mut W {
        self.variant(SSDEN_A::SSDEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Determines whether an interrupt occurs when receiver data is available."]
    #[inline(always)]
    pub fn rxrdyen(&self) -> RXRDYEN_R {
        RXRDYEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when the transmitter holding register is available."]
    #[inline(always)]
    pub fn txrdyen(&self) -> TXRDYEN_R {
        TXRDYEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur."]
    #[inline(always)]
    pub fn rxoven(&self) -> RXOVEN_R {
        RXOVEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available."]
    #[inline(always)]
    pub fn txuren(&self) -> TXUREN_R {
        TXUREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub fn ssaen(&self) -> SSAEN_R {
        SSAEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub fn ssden(&self) -> SSDEN_R {
        SSDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Determines whether an interrupt occurs when receiver data is available."]
    #[inline(always)]
    pub fn rxrdyen(&mut self) -> RXRDYEN_W {
        RXRDYEN_W { w: self }
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when the transmitter holding register is available."]
    #[inline(always)]
    pub fn txrdyen(&mut self) -> TXRDYEN_W {
        TXRDYEN_W { w: self }
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur."]
    #[inline(always)]
    pub fn rxoven(&mut self) -> RXOVEN_W {
        RXOVEN_W { w: self }
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available."]
    #[inline(always)]
    pub fn txuren(&mut self) -> TXUREN_W {
        TXUREN_W { w: self }
    }
    #[doc = "Bit 4 - Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub fn ssaen(&mut self) -> SSAEN_W {
        SSAEN_W { w: self }
    }
    #[doc = "Bit 5 - Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub fn ssden(&mut self) -> SSDEN_W {
        SSDEN_W { w: self }
    }
}
