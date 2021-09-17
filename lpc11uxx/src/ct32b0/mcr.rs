#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0I_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "0: Disabled"]
    DISABLED = 0,
}
impl From<MR0I_A> for bool {
    #[inline(always)]
    fn from(variant: MR0I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0I` reader - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
pub struct MR0I_R(crate::FieldReader<bool, MR0I_A>);
impl MR0I_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR0I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0I_A {
        match self.bits {
            true => MR0I_A::ENABLED,
            false => MR0I_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MR0I_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MR0I_A::DISABLED
    }
}
impl core::ops::Deref for MR0I_R {
    type Target = crate::FieldReader<bool, MR0I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR0I` writer - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
pub struct MR0I_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR0I_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR0I_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR0I_A::DISABLED)
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
#[doc = "Reset on MR0: the TC will be reset if MR0 matches it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0R_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "0: Disabled"]
    DISABLED = 0,
}
impl From<MR0R_A> for bool {
    #[inline(always)]
    fn from(variant: MR0R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0R` reader - Reset on MR0: the TC will be reset if MR0 matches it."]
pub struct MR0R_R(crate::FieldReader<bool, MR0R_A>);
impl MR0R_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR0R_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0R_A {
        match self.bits {
            true => MR0R_A::ENABLED,
            false => MR0R_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MR0R_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MR0R_A::DISABLED
    }
}
impl core::ops::Deref for MR0R_R {
    type Target = crate::FieldReader<bool, MR0R_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR0R` writer - Reset on MR0: the TC will be reset if MR0 matches it."]
pub struct MR0R_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR0R_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR0R_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR0R_A::DISABLED)
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
#[doc = "Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0S_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "0: Disabled"]
    DISABLED = 0,
}
impl From<MR0S_A> for bool {
    #[inline(always)]
    fn from(variant: MR0S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0S` reader - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
pub struct MR0S_R(crate::FieldReader<bool, MR0S_A>);
impl MR0S_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR0S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0S_A {
        match self.bits {
            true => MR0S_A::ENABLED,
            false => MR0S_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MR0S_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MR0S_A::DISABLED
    }
}
impl core::ops::Deref for MR0S_R {
    type Target = crate::FieldReader<bool, MR0S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR0S` writer - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
pub struct MR0S_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR0S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR0S_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR0S_A::DISABLED)
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
#[doc = "Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1I_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "0: Disabled"]
    DISABLED = 0,
}
impl From<MR1I_A> for bool {
    #[inline(always)]
    fn from(variant: MR1I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1I` reader - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
pub struct MR1I_R(crate::FieldReader<bool, MR1I_A>);
impl MR1I_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR1I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1I_A {
        match self.bits {
            true => MR1I_A::ENABLED,
            false => MR1I_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MR1I_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MR1I_A::DISABLED
    }
}
impl core::ops::Deref for MR1I_R {
    type Target = crate::FieldReader<bool, MR1I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR1I` writer - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
pub struct MR1I_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR1I_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR1I_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR1I_A::DISABLED)
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
#[doc = "Reset on MR1: the TC will be reset if MR1 matches it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1R_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "0: Disabled"]
    DISABLED = 0,
}
impl From<MR1R_A> for bool {
    #[inline(always)]
    fn from(variant: MR1R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1R` reader - Reset on MR1: the TC will be reset if MR1 matches it."]
pub struct MR1R_R(crate::FieldReader<bool, MR1R_A>);
impl MR1R_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR1R_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1R_A {
        match self.bits {
            true => MR1R_A::ENABLED,
            false => MR1R_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MR1R_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MR1R_A::DISABLED
    }
}
impl core::ops::Deref for MR1R_R {
    type Target = crate::FieldReader<bool, MR1R_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR1R` writer - Reset on MR1: the TC will be reset if MR1 matches it."]
pub struct MR1R_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR1R_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR1R_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR1R_A::DISABLED)
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
#[doc = "Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1S_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "0: Disabled"]
    DISABLED = 0,
}
impl From<MR1S_A> for bool {
    #[inline(always)]
    fn from(variant: MR1S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1S` reader - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
pub struct MR1S_R(crate::FieldReader<bool, MR1S_A>);
impl MR1S_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR1S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1S_A {
        match self.bits {
            true => MR1S_A::ENABLED,
            false => MR1S_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MR1S_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MR1S_A::DISABLED
    }
}
impl core::ops::Deref for MR1S_R {
    type Target = crate::FieldReader<bool, MR1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR1S` writer - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
pub struct MR1S_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR1S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR1S_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR1S_A::DISABLED)
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
#[doc = "Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2I_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "0: Disabled"]
    DISABLED = 0,
}
impl From<MR2I_A> for bool {
    #[inline(always)]
    fn from(variant: MR2I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2I` reader - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
pub struct MR2I_R(crate::FieldReader<bool, MR2I_A>);
impl MR2I_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR2I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2I_A {
        match self.bits {
            true => MR2I_A::ENABLED,
            false => MR2I_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MR2I_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MR2I_A::DISABLED
    }
}
impl core::ops::Deref for MR2I_R {
    type Target = crate::FieldReader<bool, MR2I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR2I` writer - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
pub struct MR2I_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR2I_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR2I_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR2I_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Reset on MR2: the TC will be reset if MR2 matches it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2R_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "0: Disabled"]
    DISABLED = 0,
}
impl From<MR2R_A> for bool {
    #[inline(always)]
    fn from(variant: MR2R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2R` reader - Reset on MR2: the TC will be reset if MR2 matches it."]
pub struct MR2R_R(crate::FieldReader<bool, MR2R_A>);
impl MR2R_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR2R_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2R_A {
        match self.bits {
            true => MR2R_A::ENABLED,
            false => MR2R_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MR2R_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MR2R_A::DISABLED
    }
}
impl core::ops::Deref for MR2R_R {
    type Target = crate::FieldReader<bool, MR2R_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR2R` writer - Reset on MR2: the TC will be reset if MR2 matches it."]
pub struct MR2R_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR2R_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR2R_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR2R_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2S_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "0: Disabled"]
    DISABLED = 0,
}
impl From<MR2S_A> for bool {
    #[inline(always)]
    fn from(variant: MR2S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2S` reader - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC."]
pub struct MR2S_R(crate::FieldReader<bool, MR2S_A>);
impl MR2S_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR2S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2S_A {
        match self.bits {
            true => MR2S_A::ENABLED,
            false => MR2S_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MR2S_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MR2S_A::DISABLED
    }
}
impl core::ops::Deref for MR2S_R {
    type Target = crate::FieldReader<bool, MR2S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR2S` writer - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC."]
pub struct MR2S_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR2S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR2S_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR2S_A::DISABLED)
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
#[doc = "Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3I_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "0: Disabled"]
    DISABLED = 0,
}
impl From<MR3I_A> for bool {
    #[inline(always)]
    fn from(variant: MR3I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3I` reader - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
pub struct MR3I_R(crate::FieldReader<bool, MR3I_A>);
impl MR3I_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR3I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3I_A {
        match self.bits {
            true => MR3I_A::ENABLED,
            false => MR3I_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MR3I_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MR3I_A::DISABLED
    }
}
impl core::ops::Deref for MR3I_R {
    type Target = crate::FieldReader<bool, MR3I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR3I` writer - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
pub struct MR3I_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR3I_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR3I_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR3I_A::DISABLED)
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
#[doc = "Reset on MR3: the TC will be reset if MR3 matches it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3R_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "0: Disabled"]
    DISABLED = 0,
}
impl From<MR3R_A> for bool {
    #[inline(always)]
    fn from(variant: MR3R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3R` reader - Reset on MR3: the TC will be reset if MR3 matches it."]
pub struct MR3R_R(crate::FieldReader<bool, MR3R_A>);
impl MR3R_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR3R_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3R_A {
        match self.bits {
            true => MR3R_A::ENABLED,
            false => MR3R_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MR3R_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MR3R_A::DISABLED
    }
}
impl core::ops::Deref for MR3R_R {
    type Target = crate::FieldReader<bool, MR3R_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR3R` writer - Reset on MR3: the TC will be reset if MR3 matches it."]
pub struct MR3R_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR3R_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR3R_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR3R_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3S_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "0: Disabled"]
    DISABLED = 0,
}
impl From<MR3S_A> for bool {
    #[inline(always)]
    fn from(variant: MR3S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3S` reader - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
pub struct MR3S_R(crate::FieldReader<bool, MR3S_A>);
impl MR3S_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR3S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3S_A {
        match self.bits {
            true => MR3S_A::ENABLED,
            false => MR3S_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MR3S_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MR3S_A::DISABLED
    }
}
impl core::ops::Deref for MR3S_R {
    type Target = crate::FieldReader<bool, MR3S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR3S` writer - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
pub struct MR3S_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR3S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR3S_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR3S_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn mr0i(&self) -> MR0I_R {
        MR0I_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn mr0r(&self) -> MR0R_R {
        MR0R_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn mr0s(&self) -> MR0S_R {
        MR0S_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn mr1i(&self) -> MR1I_R {
        MR1I_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn mr1r(&self) -> MR1R_R {
        MR1R_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn mr1s(&self) -> MR1S_R {
        MR1S_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn mr2i(&self) -> MR2I_R {
        MR2I_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn mr2r(&self) -> MR2R_R {
        MR2R_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    pub fn mr2s(&self) -> MR2S_R {
        MR2S_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn mr3i(&self) -> MR3I_R {
        MR3I_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn mr3r(&self) -> MR3R_R {
        MR3R_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn mr3s(&self) -> MR3S_R {
        MR3S_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn mr0i(&mut self) -> MR0I_W {
        MR0I_W { w: self }
    }
    #[doc = "Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn mr0r(&mut self) -> MR0R_W {
        MR0R_W { w: self }
    }
    #[doc = "Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn mr0s(&mut self) -> MR0S_W {
        MR0S_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn mr1i(&mut self) -> MR1I_W {
        MR1I_W { w: self }
    }
    #[doc = "Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn mr1r(&mut self) -> MR1R_W {
        MR1R_W { w: self }
    }
    #[doc = "Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn mr1s(&mut self) -> MR1S_W {
        MR1S_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn mr2i(&mut self) -> MR2I_W {
        MR2I_W { w: self }
    }
    #[doc = "Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn mr2r(&mut self) -> MR2R_W {
        MR2R_W { w: self }
    }
    #[doc = "Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    pub fn mr2s(&mut self) -> MR2S_W {
        MR2S_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn mr3i(&mut self) -> MR3I_W {
        MR3I_W { w: self }
    }
    #[doc = "Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn mr3r(&mut self) -> MR3R_W {
        MR3R_W { w: self }
    }
    #[doc = "Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn mr3s(&mut self) -> MR3S_W {
        MR3S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Match Control Register. The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
