#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IER_SPEC>> for R {
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl core::convert::From<crate::W<IER_SPEC>> for W {
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBRINTEN_A {
    #[doc = "0: Disable the RDA interrupt."]
    DISABLE_THE_RDA_INTE = 0,
    #[doc = "1: Enable the RDA interrupt."]
    ENABLE_THE_RDA_INTER = 1,
}
impl From<RBRINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RBRINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBRINTEN` reader - RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt."]
pub struct RBRINTEN_R(crate::FieldReader<bool, RBRINTEN_A>);
impl RBRINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBRINTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBRINTEN_A {
        match self.bits {
            false => RBRINTEN_A::DISABLE_THE_RDA_INTE,
            true => RBRINTEN_A::ENABLE_THE_RDA_INTER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_RDA_INTE`"]
    #[inline(always)]
    pub fn is_disable_the_rda_inte(&self) -> bool {
        **self == RBRINTEN_A::DISABLE_THE_RDA_INTE
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_RDA_INTER`"]
    #[inline(always)]
    pub fn is_enable_the_rda_inter(&self) -> bool {
        **self == RBRINTEN_A::ENABLE_THE_RDA_INTER
    }
}
impl core::ops::Deref for RBRINTEN_R {
    type Target = crate::FieldReader<bool, RBRINTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBRINTEN` writer - RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt."]
pub struct RBRINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RBRINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBRINTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the RDA interrupt."]
    #[inline(always)]
    pub fn disable_the_rda_inte(self) -> &'a mut W {
        self.variant(RBRINTEN_A::DISABLE_THE_RDA_INTE)
    }
    #[doc = "Enable the RDA interrupt."]
    #[inline(always)]
    pub fn enable_the_rda_inter(self) -> &'a mut W {
        self.variant(RBRINTEN_A::ENABLE_THE_RDA_INTER)
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
#[doc = "THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREINTEN_A {
    #[doc = "0: Disable the THRE interrupt."]
    DISABLE_THE_THRE_INT = 0,
    #[doc = "1: Enable the THRE interrupt."]
    ENABLE_THE_THRE_INTE = 1,
}
impl From<THREINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: THREINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THREINTEN` reader - THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\]."]
pub struct THREINTEN_R(crate::FieldReader<bool, THREINTEN_A>);
impl THREINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        THREINTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREINTEN_A {
        match self.bits {
            false => THREINTEN_A::DISABLE_THE_THRE_INT,
            true => THREINTEN_A::ENABLE_THE_THRE_INTE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_THRE_INT`"]
    #[inline(always)]
    pub fn is_disable_the_thre_int(&self) -> bool {
        **self == THREINTEN_A::DISABLE_THE_THRE_INT
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_THRE_INTE`"]
    #[inline(always)]
    pub fn is_enable_the_thre_inte(&self) -> bool {
        **self == THREINTEN_A::ENABLE_THE_THRE_INTE
    }
}
impl core::ops::Deref for THREINTEN_R {
    type Target = crate::FieldReader<bool, THREINTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THREINTEN` writer - THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\]."]
pub struct THREINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> THREINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THREINTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the THRE interrupt."]
    #[inline(always)]
    pub fn disable_the_thre_int(self) -> &'a mut W {
        self.variant(THREINTEN_A::DISABLE_THE_THRE_INT)
    }
    #[doc = "Enable the THRE interrupt."]
    #[inline(always)]
    pub fn enable_the_thre_inte(self) -> &'a mut W {
        self.variant(THREINTEN_A::ENABLE_THE_THRE_INTE)
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
#[doc = "Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLSINTEN_A {
    #[doc = "0: Disable the RLS interrupt."]
    DISABLE_THE_RLS_INTE = 0,
    #[doc = "1: Enable the RLS interrupt."]
    ENABLE_THE_RLS_INTER = 1,
}
impl From<RLSINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLSINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLSINTEN` reader - Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\]."]
pub struct RLSINTEN_R(crate::FieldReader<bool, RLSINTEN_A>);
impl RLSINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RLSINTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLSINTEN_A {
        match self.bits {
            false => RLSINTEN_A::DISABLE_THE_RLS_INTE,
            true => RLSINTEN_A::ENABLE_THE_RLS_INTER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_RLS_INTE`"]
    #[inline(always)]
    pub fn is_disable_the_rls_inte(&self) -> bool {
        **self == RLSINTEN_A::DISABLE_THE_RLS_INTE
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_RLS_INTER`"]
    #[inline(always)]
    pub fn is_enable_the_rls_inter(&self) -> bool {
        **self == RLSINTEN_A::ENABLE_THE_RLS_INTER
    }
}
impl core::ops::Deref for RLSINTEN_R {
    type Target = crate::FieldReader<bool, RLSINTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLSINTEN` writer - Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\]."]
pub struct RLSINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RLSINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RLSINTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the RLS interrupt."]
    #[inline(always)]
    pub fn disable_the_rls_inte(self) -> &'a mut W {
        self.variant(RLSINTEN_A::DISABLE_THE_RLS_INTE)
    }
    #[doc = "Enable the RLS interrupt."]
    #[inline(always)]
    pub fn enable_the_rls_inter(self) -> &'a mut W {
        self.variant(RLSINTEN_A::ENABLE_THE_RLS_INTER)
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
#[doc = "Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSINTEN_A {
    #[doc = "0: Disable the MS interrupt."]
    DISABLE_THE_MS_INTER = 0,
    #[doc = "1: Enable the MS interrupt."]
    ENABLE_THE_MS_INTERR = 1,
}
impl From<MSINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSINTEN` reader - Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR."]
pub struct MSINTEN_R(crate::FieldReader<bool, MSINTEN_A>);
impl MSINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSINTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSINTEN_A {
        match self.bits {
            false => MSINTEN_A::DISABLE_THE_MS_INTER,
            true => MSINTEN_A::ENABLE_THE_MS_INTERR,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_MS_INTER`"]
    #[inline(always)]
    pub fn is_disable_the_ms_inter(&self) -> bool {
        **self == MSINTEN_A::DISABLE_THE_MS_INTER
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_MS_INTERR`"]
    #[inline(always)]
    pub fn is_enable_the_ms_interr(&self) -> bool {
        **self == MSINTEN_A::ENABLE_THE_MS_INTERR
    }
}
impl core::ops::Deref for MSINTEN_R {
    type Target = crate::FieldReader<bool, MSINTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSINTEN` writer - Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR."]
pub struct MSINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSINTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the MS interrupt."]
    #[inline(always)]
    pub fn disable_the_ms_inter(self) -> &'a mut W {
        self.variant(MSINTEN_A::DISABLE_THE_MS_INTER)
    }
    #[doc = "Enable the MS interrupt."]
    #[inline(always)]
    pub fn enable_the_ms_interr(self) -> &'a mut W {
        self.variant(MSINTEN_A::ENABLE_THE_MS_INTERR)
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
#[doc = "Enables the end of auto-baud interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTEN_A {
    #[doc = "0: Disable end of auto-baud Interrupt."]
    DISABLED = 0,
    #[doc = "1: Enable end of auto-baud Interrupt."]
    ENABLED = 1,
}
impl From<ABEOINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABEOINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABEOINTEN` reader - Enables the end of auto-baud interrupt."]
pub struct ABEOINTEN_R(crate::FieldReader<bool, ABEOINTEN_A>);
impl ABEOINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABEOINTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABEOINTEN_A {
        match self.bits {
            false => ABEOINTEN_A::DISABLED,
            true => ABEOINTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ABEOINTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ABEOINTEN_A::ENABLED
    }
}
impl core::ops::Deref for ABEOINTEN_R {
    type Target = crate::FieldReader<bool, ABEOINTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABEOINTEN` writer - Enables the end of auto-baud interrupt."]
pub struct ABEOINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABEOINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABEOINTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ABEOINTEN_A::DISABLED)
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ABEOINTEN_A::ENABLED)
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
#[doc = "Enables the auto-baud time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTEN_A {
    #[doc = "0: Disable auto-baud time-out Interrupt."]
    DISABLED = 0,
    #[doc = "1: Enable auto-baud time-out Interrupt."]
    ENABLED = 1,
}
impl From<ABTOINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABTOINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTOINTEN` reader - Enables the auto-baud time-out interrupt."]
pub struct ABTOINTEN_R(crate::FieldReader<bool, ABTOINTEN_A>);
impl ABTOINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTOINTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTOINTEN_A {
        match self.bits {
            false => ABTOINTEN_A::DISABLED,
            true => ABTOINTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ABTOINTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ABTOINTEN_A::ENABLED
    }
}
impl core::ops::Deref for ABTOINTEN_R {
    type Target = crate::FieldReader<bool, ABTOINTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABTOINTEN` writer - Enables the auto-baud time-out interrupt."]
pub struct ABTOINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTOINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTOINTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ABTOINTEN_A::DISABLED)
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ABTOINTEN_A::ENABLED)
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
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrinten(&self) -> RBRINTEN_R {
        RBRINTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threinten(&self) -> THREINTEN_R {
        THREINTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rlsinten(&self) -> RLSINTEN_R {
        RLSINTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR."]
    #[inline(always)]
    pub fn msinten(&self) -> MSINTEN_R {
        MSINTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&self) -> ABEOINTEN_R {
        ABEOINTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&self) -> ABTOINTEN_R {
        ABTOINTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrinten(&mut self) -> RBRINTEN_W {
        RBRINTEN_W { w: self }
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threinten(&mut self) -> THREINTEN_W {
        THREINTEN_W { w: self }
    }
    #[doc = "Bit 2 - Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rlsinten(&mut self) -> RLSINTEN_W {
        RLSINTEN_W { w: self }
    }
    #[doc = "Bit 3 - Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR."]
    #[inline(always)]
    pub fn msinten(&mut self) -> MSINTEN_W {
        MSINTEN_W { w: self }
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&mut self) -> ABEOINTEN_W {
        ABEOINTEN_W { w: self }
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&mut self) -> ABTOINTEN_W {
        ABTOINTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
