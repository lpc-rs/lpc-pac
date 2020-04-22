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
#[doc = "Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSAEN_A {
    #[doc = "0: Disabled. No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    DISABLED = 0,
    #[doc = "1: Enabled. An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    ENABLED = 1,
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
            false => SSAEN_A::DISABLED,
            true => SSAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSAEN_A::ENABLED
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
    #[doc = "Disabled. No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSAEN_A::DISABLED)
    }
    #[doc = "Enabled. An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSAEN_A::ENABLED)
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
#[doc = "Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDEN_A {
    #[doc = "0: Disabled. No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    DISABLED = 0,
    #[doc = "1: Enabled. An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    ENABLED = 1,
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
            false => SSDEN_A::DISABLED,
            true => SSDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSDEN_A::ENABLED
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
    #[doc = "Disabled. No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSDEN_A::DISABLED)
    }
    #[doc = "Enabled. An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSDEN_A::ENABLED)
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
#[doc = "Master idle interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTIDLEEN_A {
    #[doc = "0: No interrupt will be generated when the SPI master function is idle."]
    DISABLED = 0,
    #[doc = "1: An interrupt will be generated when the SPI master function is fully idle."]
    ENABLED = 1,
}
impl From<MSTIDLEEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSTIDLEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTIDLEEN`"]
pub type MSTIDLEEN_R = crate::R<bool, MSTIDLEEN_A>;
impl MSTIDLEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTIDLEEN_A {
        match self.bits {
            false => MSTIDLEEN_A::DISABLED,
            true => MSTIDLEEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTIDLEEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTIDLEEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `MSTIDLEEN`"]
pub struct MSTIDLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTIDLEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTIDLEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt will be generated when the SPI master function is idle."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTIDLEEN_A::DISABLED)
    }
    #[doc = "An interrupt will be generated when the SPI master function is fully idle."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTIDLEEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 4 - Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub fn ssaen(&self) -> SSAEN_R {
        SSAEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub fn ssden(&self) -> SSDEN_R {
        SSDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master idle interrupt enable."]
    #[inline(always)]
    pub fn mstidleen(&self) -> MSTIDLEEN_R {
        MSTIDLEEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub fn ssaen(&mut self) -> SSAEN_W {
        SSAEN_W { w: self }
    }
    #[doc = "Bit 5 - Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub fn ssden(&mut self) -> SSDEN_W {
        SSDEN_W { w: self }
    }
    #[doc = "Bit 8 - Master idle interrupt enable."]
    #[inline(always)]
    pub fn mstidleen(&mut self) -> MSTIDLEEN_W {
        MSTIDLEEN_W { w: self }
    }
}
