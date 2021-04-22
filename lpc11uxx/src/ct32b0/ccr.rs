#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CCR_SPEC>> for R {
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl core::convert::From<crate::W<CCR_SPEC>> for W {
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Capture on CT32B0_CAP0 rising edge: a sequence of 0 then 1 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0RE_A {
    #[doc = "1: Enabled."]
    ENABLED = 1,
    #[doc = "0: Disabled."]
    DISABLED = 0,
}
impl From<CAP0RE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0RE` reader - Capture on CT32B0_CAP0 rising edge: a sequence of 0 then 1 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
pub struct CAP0RE_R(crate::FieldReader<bool, CAP0RE_A>);
impl CAP0RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP0RE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0RE_A {
        match self.bits {
            true => CAP0RE_A::ENABLED,
            false => CAP0RE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAP0RE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAP0RE_A::DISABLED
    }
}
impl core::ops::Deref for CAP0RE_R {
    type Target = crate::FieldReader<bool, CAP0RE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP0RE` writer - Capture on CT32B0_CAP0 rising edge: a sequence of 0 then 1 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
pub struct CAP0RE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0RE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP0RE_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP0RE_A::DISABLED)
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
#[doc = "Capture on CT32B0_CAP0 falling edge: a sequence of 1 then 0 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0FE_A {
    #[doc = "1: Enabled."]
    ENABLED = 1,
    #[doc = "0: Disabled."]
    DISABLED = 0,
}
impl From<CAP0FE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0FE` reader - Capture on CT32B0_CAP0 falling edge: a sequence of 1 then 0 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
pub struct CAP0FE_R(crate::FieldReader<bool, CAP0FE_A>);
impl CAP0FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP0FE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0FE_A {
        match self.bits {
            true => CAP0FE_A::ENABLED,
            false => CAP0FE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAP0FE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAP0FE_A::DISABLED
    }
}
impl core::ops::Deref for CAP0FE_R {
    type Target = crate::FieldReader<bool, CAP0FE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP0FE` writer - Capture on CT32B0_CAP0 falling edge: a sequence of 1 then 0 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
pub struct CAP0FE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0FE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0FE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP0FE_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP0FE_A::DISABLED)
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
#[doc = "Interrupt on CT32B0_CAP0 event: a CR0 load due to a CT32B0_CAP0 event will generate an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0I_A {
    #[doc = "1: Enabled."]
    ENABLED = 1,
    #[doc = "0: Disabled."]
    DISABLED = 0,
}
impl From<CAP0I_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0I` reader - Interrupt on CT32B0_CAP0 event: a CR0 load due to a CT32B0_CAP0 event will generate an interrupt."]
pub struct CAP0I_R(crate::FieldReader<bool, CAP0I_A>);
impl CAP0I_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP0I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0I_A {
        match self.bits {
            true => CAP0I_A::ENABLED,
            false => CAP0I_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAP0I_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAP0I_A::DISABLED
    }
}
impl core::ops::Deref for CAP0I_R {
    type Target = crate::FieldReader<bool, CAP0I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP0I` writer - Interrupt on CT32B0_CAP0 event: a CR0 load due to a CT32B0_CAP0 event will generate an interrupt."]
pub struct CAP0I_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0I_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP0I_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP0I_A::DISABLED)
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
#[doc = "Capture on CT32B0_CAP1 rising edge: a sequence of 0 then 1 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1RE_A {
    #[doc = "1: Enabled."]
    ENABLED = 1,
    #[doc = "0: Disabled."]
    DISABLED = 0,
}
impl From<CAP1RE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1RE` reader - Capture on CT32B0_CAP1 rising edge: a sequence of 0 then 1 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC."]
pub struct CAP1RE_R(crate::FieldReader<bool, CAP1RE_A>);
impl CAP1RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP1RE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP1RE_A {
        match self.bits {
            true => CAP1RE_A::ENABLED,
            false => CAP1RE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAP1RE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAP1RE_A::DISABLED
    }
}
impl core::ops::Deref for CAP1RE_R {
    type Target = crate::FieldReader<bool, CAP1RE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP1RE` writer - Capture on CT32B0_CAP1 rising edge: a sequence of 0 then 1 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC."]
pub struct CAP1RE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1RE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP1RE_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP1RE_A::DISABLED)
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
#[doc = "Capture on CT32B0_CAP1 falling edge: a sequence of 1 then 0 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1FE_A {
    #[doc = "1: Enabled."]
    ENABLED = 1,
    #[doc = "0: Disabled."]
    DISABLED = 0,
}
impl From<CAP1FE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1FE` reader - Capture on CT32B0_CAP1 falling edge: a sequence of 1 then 0 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC."]
pub struct CAP1FE_R(crate::FieldReader<bool, CAP1FE_A>);
impl CAP1FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP1FE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP1FE_A {
        match self.bits {
            true => CAP1FE_A::ENABLED,
            false => CAP1FE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAP1FE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAP1FE_A::DISABLED
    }
}
impl core::ops::Deref for CAP1FE_R {
    type Target = crate::FieldReader<bool, CAP1FE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP1FE` writer - Capture on CT32B0_CAP1 falling edge: a sequence of 1 then 0 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC."]
pub struct CAP1FE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1FE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1FE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP1FE_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP1FE_A::DISABLED)
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
#[doc = "Interrupt on CT32B0_CAP1 event: a CR1 load due to a CT32B0_CAP1 event will generate an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1I_A {
    #[doc = "1: Enabled."]
    ENABLED = 1,
    #[doc = "0: Disabled."]
    DISABLED = 0,
}
impl From<CAP1I_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1I` reader - Interrupt on CT32B0_CAP1 event: a CR1 load due to a CT32B0_CAP1 event will generate an interrupt."]
pub struct CAP1I_R(crate::FieldReader<bool, CAP1I_A>);
impl CAP1I_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP1I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP1I_A {
        match self.bits {
            true => CAP1I_A::ENABLED,
            false => CAP1I_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAP1I_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAP1I_A::DISABLED
    }
}
impl core::ops::Deref for CAP1I_R {
    type Target = crate::FieldReader<bool, CAP1I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP1I` writer - Interrupt on CT32B0_CAP1 event: a CR1 load due to a CT32B0_CAP1 event will generate an interrupt."]
pub struct CAP1I_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1I_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP1I_A::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP1I_A::DISABLED)
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
    #[doc = "Bit 0 - Capture on CT32B0_CAP0 rising edge: a sequence of 0 then 1 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap0re(&self) -> CAP0RE_R {
        CAP0RE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture on CT32B0_CAP0 falling edge: a sequence of 1 then 0 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap0fe(&self) -> CAP0FE_R {
        CAP0FE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt on CT32B0_CAP0 event: a CR0 load due to a CT32B0_CAP0 event will generate an interrupt."]
    #[inline(always)]
    pub fn cap0i(&self) -> CAP0I_R {
        CAP0I_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Capture on CT32B0_CAP1 rising edge: a sequence of 0 then 1 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap1re(&self) -> CAP1RE_R {
        CAP1RE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Capture on CT32B0_CAP1 falling edge: a sequence of 1 then 0 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap1fe(&self) -> CAP1FE_R {
        CAP1FE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt on CT32B0_CAP1 event: a CR1 load due to a CT32B0_CAP1 event will generate an interrupt."]
    #[inline(always)]
    pub fn cap1i(&self) -> CAP1I_R {
        CAP1I_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture on CT32B0_CAP0 rising edge: a sequence of 0 then 1 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap0re(&mut self) -> CAP0RE_W {
        CAP0RE_W { w: self }
    }
    #[doc = "Bit 1 - Capture on CT32B0_CAP0 falling edge: a sequence of 1 then 0 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap0fe(&mut self) -> CAP0FE_W {
        CAP0FE_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt on CT32B0_CAP0 event: a CR0 load due to a CT32B0_CAP0 event will generate an interrupt."]
    #[inline(always)]
    pub fn cap0i(&mut self) -> CAP0I_W {
        CAP0I_W { w: self }
    }
    #[doc = "Bit 6 - Capture on CT32B0_CAP1 rising edge: a sequence of 0 then 1 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap1re(&mut self) -> CAP1RE_W {
        CAP1RE_W { w: self }
    }
    #[doc = "Bit 7 - Capture on CT32B0_CAP1 falling edge: a sequence of 1 then 0 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap1fe(&mut self) -> CAP1FE_W {
        CAP1FE_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt on CT32B0_CAP1 event: a CR1 load due to a CT32B0_CAP1 event will generate an interrupt."]
    #[inline(always)]
    pub fn cap1i(&mut self) -> CAP1I_W {
        CAP1I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
