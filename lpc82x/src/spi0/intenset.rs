#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `RXRDYEN` reader - Determines whether an interrupt occurs when receiver data is available."]
pub struct RXRDYEN_R(crate::FieldReader<bool, RXRDYEN_A>);
impl RXRDYEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRDYEN_R(crate::FieldReader::new(bits))
    }
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
        **self == RXRDYEN_A::RXRDYEN_0
    }
    #[doc = "Checks if the value of the field is `RXRDYEN_1`"]
    #[inline(always)]
    pub fn is_rxrdyen_1(&self) -> bool {
        **self == RXRDYEN_A::RXRDYEN_1
    }
}
impl core::ops::Deref for RXRDYEN_R {
    type Target = crate::FieldReader<bool, RXRDYEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRDYEN` writer - Determines whether an interrupt occurs when receiver data is available."]
pub struct RXRDYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRDYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXRDYEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
#[doc = "Field `TXRDYEN` reader - Determines whether an interrupt occurs when the transmitter holding register is available."]
pub struct TXRDYEN_R(crate::FieldReader<bool, TXRDYEN_A>);
impl TXRDYEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRDYEN_R(crate::FieldReader::new(bits))
    }
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
        **self == TXRDYEN_A::TXRDYEN_0
    }
    #[doc = "Checks if the value of the field is `TXRDYEN_1`"]
    #[inline(always)]
    pub fn is_txrdyen_1(&self) -> bool {
        **self == TXRDYEN_A::TXRDYEN_1
    }
}
impl core::ops::Deref for TXRDYEN_R {
    type Target = crate::FieldReader<bool, TXRDYEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDYEN` writer - Determines whether an interrupt occurs when the transmitter holding register is available."]
pub struct TXRDYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRDYEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
#[doc = "Field `RXOVEN` reader - Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur."]
pub struct RXOVEN_R(crate::FieldReader<bool, RXOVEN_A>);
impl RXOVEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVEN_R(crate::FieldReader::new(bits))
    }
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
        **self == RXOVEN_A::RXOVEN_0
    }
    #[doc = "Checks if the value of the field is `RXOVEN_1`"]
    #[inline(always)]
    pub fn is_rxoven_1(&self) -> bool {
        **self == RXOVEN_A::RXOVEN_1
    }
}
impl core::ops::Deref for RXOVEN_R {
    type Target = crate::FieldReader<bool, RXOVEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVEN` writer - Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur."]
pub struct RXOVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOVEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
#[doc = "Field `TXUREN` reader - Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available."]
pub struct TXUREN_R(crate::FieldReader<bool, TXUREN_A>);
impl TXUREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUREN_R(crate::FieldReader::new(bits))
    }
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
        **self == TXUREN_A::TXUREN_0
    }
    #[doc = "Checks if the value of the field is `TXUREN_1`"]
    #[inline(always)]
    pub fn is_txuren_1(&self) -> bool {
        **self == TXUREN_A::TXUREN_1
    }
}
impl core::ops::Deref for TXUREN_R {
    type Target = crate::FieldReader<bool, TXUREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUREN` writer - Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available."]
pub struct TXUREN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXUREN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
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
#[doc = "Field `SSAEN` reader - Determines whether an interrupt occurs when the Slave Select is asserted."]
pub struct SSAEN_R(crate::FieldReader<bool, SSAEN_A>);
impl SSAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSAEN_R(crate::FieldReader::new(bits))
    }
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
        **self == SSAEN_A::SSAEN_0
    }
    #[doc = "Checks if the value of the field is `SSAEN_1`"]
    #[inline(always)]
    pub fn is_ssaen_1(&self) -> bool {
        **self == SSAEN_A::SSAEN_1
    }
}
impl core::ops::Deref for SSAEN_R {
    type Target = crate::FieldReader<bool, SSAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSAEN` writer - Determines whether an interrupt occurs when the Slave Select is asserted."]
pub struct SSAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSAEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
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
#[doc = "Field `SSDEN` reader - Determines whether an interrupt occurs when the Slave Select is deasserted."]
pub struct SSDEN_R(crate::FieldReader<bool, SSDEN_A>);
impl SSDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSDEN_R(crate::FieldReader::new(bits))
    }
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
        **self == SSDEN_A::SSDEN_0
    }
    #[doc = "Checks if the value of the field is `SSDEN_1`"]
    #[inline(always)]
    pub fn is_ssden_1(&self) -> bool {
        **self == SSDEN_A::SSDEN_1
    }
}
impl core::ops::Deref for SSDEN_R {
    type Target = crate::FieldReader<bool, SSDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSDEN` writer - Determines whether an interrupt occurs when the Slave Select is deasserted."]
pub struct SSDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSDEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Determines whether an interrupt occurs when the MSTIDLE enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTIDLEEN_A {
    #[doc = "0: No interrupt will be generated when MSTIDLE enabled."]
    MSTIDLEEN_0 = 0,
    #[doc = "1: An interrupt will be generated when MSTIDLE enabled."]
    MSTIDLEEN_1 = 1,
}
impl From<MSTIDLEEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSTIDLEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTIDLEEN` reader - Determines whether an interrupt occurs when the MSTIDLE enable"]
pub struct MSTIDLEEN_R(crate::FieldReader<bool, MSTIDLEEN_A>);
impl MSTIDLEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTIDLEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTIDLEEN_A {
        match self.bits {
            false => MSTIDLEEN_A::MSTIDLEEN_0,
            true => MSTIDLEEN_A::MSTIDLEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MSTIDLEEN_0`"]
    #[inline(always)]
    pub fn is_mstidleen_0(&self) -> bool {
        **self == MSTIDLEEN_A::MSTIDLEEN_0
    }
    #[doc = "Checks if the value of the field is `MSTIDLEEN_1`"]
    #[inline(always)]
    pub fn is_mstidleen_1(&self) -> bool {
        **self == MSTIDLEEN_A::MSTIDLEEN_1
    }
}
impl core::ops::Deref for MSTIDLEEN_R {
    type Target = crate::FieldReader<bool, MSTIDLEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTIDLEEN` writer - Determines whether an interrupt occurs when the MSTIDLE enable"]
pub struct MSTIDLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTIDLEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTIDLEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt will be generated when MSTIDLE enabled."]
    #[inline(always)]
    pub fn mstidleen_0(self) -> &'a mut W {
        self.variant(MSTIDLEEN_A::MSTIDLEEN_0)
    }
    #[doc = "An interrupt will be generated when MSTIDLE enabled."]
    #[inline(always)]
    pub fn mstidleen_1(self) -> &'a mut W {
        self.variant(MSTIDLEEN_A::MSTIDLEEN_1)
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
    #[doc = "Bit 8 - Determines whether an interrupt occurs when the MSTIDLE enable"]
    #[inline(always)]
    pub fn mstidleen(&self) -> MSTIDLEEN_R {
        MSTIDLEEN_R::new(((self.bits >> 8) & 0x01) != 0)
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
    #[doc = "Bit 8 - Determines whether an interrupt occurs when the MSTIDLE enable"]
    #[inline(always)]
    pub fn mstidleen(&mut self) -> MSTIDLEEN_W {
        MSTIDLEEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
