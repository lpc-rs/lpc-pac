#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTENSET_SPEC>> for R {
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
impl core::convert::From<crate::W<INTENSET_SPEC>> for W {
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `SSAEN` reader - Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
pub struct SSAEN_R(crate::FieldReader<bool, SSAEN_A>);
impl SSAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSAEN_R(crate::FieldReader::new(bits))
    }
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
        **self == SSAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SSAEN_A::ENABLED
    }
}
impl core::ops::Deref for SSAEN_R {
    type Target = crate::FieldReader<bool, SSAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSAEN` writer - Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
pub struct SSAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSAEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
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
#[doc = "Field `SSDEN` reader - Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
pub struct SSDEN_R(crate::FieldReader<bool, SSDEN_A>);
impl SSDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSDEN_R(crate::FieldReader::new(bits))
    }
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
        **self == SSDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SSDEN_A::ENABLED
    }
}
impl core::ops::Deref for SSDEN_R {
    type Target = crate::FieldReader<bool, SSDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSDEN` writer - Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
pub struct SSDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSDEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
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
#[doc = "Field `MSTIDLEEN` reader - Master idle interrupt enable."]
pub struct MSTIDLEEN_R(crate::FieldReader<bool, MSTIDLEEN_A>);
impl MSTIDLEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTIDLEEN_R(crate::FieldReader::new(bits))
    }
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
        **self == MSTIDLEEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MSTIDLEEN_A::ENABLED
    }
}
impl core::ops::Deref for MSTIDLEEN_R {
    type Target = crate::FieldReader<bool, MSTIDLEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTIDLEEN` writer - Master idle interrupt enable."]
pub struct MSTIDLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTIDLEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTIDLEEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
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
    #[doc = "Writes raw bits to the register."]
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
