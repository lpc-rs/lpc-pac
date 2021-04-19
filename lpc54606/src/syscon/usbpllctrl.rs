#[doc = "Register `USBPLLCTRL` reader"]
pub struct R(crate::R<USBPLLCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<USBPLLCTRL_SPEC>> for R {
    fn from(reader: crate::R<USBPLLCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPLLCTRL` writer"]
pub struct W(crate::W<USBPLLCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPLLCTRL_SPEC>;
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
impl core::convert::From<crate::W<USBPLLCTRL_SPEC>> for W {
    fn from(writer: crate::W<USBPLLCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSEL` reader - PLL feedback Divider value."]
pub struct MSEL_R(crate::FieldReader<u8, u8>);
impl MSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSEL` writer - PLL feedback Divider value."]
pub struct MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PSEL` reader - PLL Divider value."]
pub struct PSEL_R(crate::FieldReader<u8, u8>);
impl PSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEL` writer - PLL Divider value."]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `NSEL` reader - PLL feedback Divider value."]
pub struct NSEL_R(crate::FieldReader<u8, u8>);
impl NSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        NSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSEL` writer - PLL feedback Divider value."]
pub struct NSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Direct CCO clock output control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECT_A {
    #[doc = "0: CCO Clock signal goes through post divider."]
    DISABLED = 0,
    #[doc = "1: CCO Clock signal goes directly to output(s).."]
    ENABLED = 1,
}
impl From<DIRECT_A> for bool {
    #[inline(always)]
    fn from(variant: DIRECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRECT` reader - Direct CCO clock output control."]
pub struct DIRECT_R(crate::FieldReader<bool, DIRECT_A>);
impl DIRECT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRECT_A {
        match self.bits {
            false => DIRECT_A::DISABLED,
            true => DIRECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DIRECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DIRECT_A::ENABLED
    }
}
impl core::ops::Deref for DIRECT_R {
    type Target = crate::FieldReader<bool, DIRECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRECT` writer - Direct CCO clock output control."]
pub struct DIRECT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CCO Clock signal goes through post divider."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIRECT_A::DISABLED)
    }
    #[doc = "CCO Clock signal goes directly to output(s).."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIRECT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Input clock bypass control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: CCO clock is sent to post dividers.."]
    DISABLED = 0,
    #[doc = "1: PLL input clock is sent to post dividers.."]
    ENABLED = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Input clock bypass control."]
pub struct BYPASS_R(crate::FieldReader<bool, BYPASS_A>);
impl BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLED,
            true => BYPASS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BYPASS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BYPASS_A::ENABLED
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool, BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS` writer - Input clock bypass control."]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CCO clock is sent to post dividers.."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLED)
    }
    #[doc = "PLL input clock is sent to post dividers.."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `FBSEL` reader - Feedback divider input clock control."]
pub struct FBSEL_R(crate::FieldReader<bool, bool>);
impl FBSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FBSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBSEL` writer - Feedback divider input clock control."]
pub struct FBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FBSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PLL feedback Divider value."]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - PLL Divider value."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - PLL feedback Divider value."]
    #[inline(always)]
    pub fn nsel(&self) -> NSEL_R {
        NSEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Direct CCO clock output control."]
    #[inline(always)]
    pub fn direct(&self) -> DIRECT_R {
        DIRECT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Input clock bypass control."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Feedback divider input clock control."]
    #[inline(always)]
    pub fn fbsel(&self) -> FBSEL_R {
        FBSEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PLL feedback Divider value."]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W { w: self }
    }
    #[doc = "Bits 8:9 - PLL Divider value."]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Bits 10:11 - PLL feedback Divider value."]
    #[inline(always)]
    pub fn nsel(&mut self) -> NSEL_W {
        NSEL_W { w: self }
    }
    #[doc = "Bit 12 - Direct CCO clock output control."]
    #[inline(always)]
    pub fn direct(&mut self) -> DIRECT_W {
        DIRECT_W { w: self }
    }
    #[doc = "Bit 13 - Input clock bypass control."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 14 - Feedback divider input clock control."]
    #[inline(always)]
    pub fn fbsel(&mut self) -> FBSEL_W {
        FBSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PLL control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllctrl](index.html) module"]
pub struct USBPLLCTRL_SPEC;
impl crate::RegisterSpec for USBPLLCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbpllctrl::R](R) reader structure"]
impl crate::Readable for USBPLLCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbpllctrl::W](W) writer structure"]
impl crate::Writable for USBPLLCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPLLCTRL to value 0"]
impl crate::Resettable for USBPLLCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
