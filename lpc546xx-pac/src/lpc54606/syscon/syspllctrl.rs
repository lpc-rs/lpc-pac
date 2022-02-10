///Register `SYSPLLCTRL` reader
pub struct R(crate::R<SYSPLLCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLLCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLLCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLLCTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SYSPLLCTRL` writer
pub struct W(crate::W<SYSPLLCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLLCTRL_SPEC>;
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
impl From<crate::W<SYSPLLCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLLCTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SELR` reader - Bandwidth select R value.
pub struct SELR_R(crate::FieldReader<u8, u8>);
impl SELR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SELR` writer - Bandwidth select R value.
pub struct SELR_W<'a> {
    w: &'a mut W,
}
impl<'a> SELR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
///Field `SELI` reader - Bandwidth select I value.
pub struct SELI_R(crate::FieldReader<u8, u8>);
impl SELI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SELI` writer - Bandwidth select I value.
pub struct SELI_W<'a> {
    w: &'a mut W,
}
impl<'a> SELI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | ((value as u32 & 0x3f) << 4);
        self.w
    }
}
///Field `SELP` reader - Bandwidth select P value.
pub struct SELP_R(crate::FieldReader<u8, u8>);
impl SELP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SELP` writer - Bandwidth select P value.
pub struct SELP_W<'a> {
    w: &'a mut W,
}
impl<'a> SELP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
///PLL bypass control.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    ///0: Bypass disabled. PLL CCO is sent to the PLL post-dividers.
    DISABLED = 0,
    ///1: Bypass enabled. PLL input clock is sent directly to the PLL output (default).
    ENABLED = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BYPASS` reader - PLL bypass control.
pub struct BYPASS_R(crate::FieldReader<bool, BYPASS_A>);
impl BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLED,
            true => BYPASS_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BYPASS_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
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
///Field `BYPASS` writer - PLL bypass control.
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Bypass disabled. PLL CCO is sent to the PLL post-dividers.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLED)
    }
    ///Bypass enabled. PLL input clock is sent directly to the PLL output (default).
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///Field `UPLIMOFF` reader - Disable upper frequency limiter.
pub struct UPLIMOFF_R(crate::FieldReader<bool, bool>);
impl UPLIMOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPLIMOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPLIMOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UPLIMOFF` writer - Disable upper frequency limiter.
pub struct UPLIMOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> UPLIMOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///Field `DIRECTI` reader - PLL0 direct input enable.
pub struct DIRECTI_R(crate::FieldReader<bool, bool>);
impl DIRECTI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIRECTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRECTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DIRECTI` writer - PLL0 direct input enable.
pub struct DIRECTI_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECTI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///PLL0 direct output enable.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTO_A {
    ///0: Disabled. The PLL output divider (P divider) is used to create the PLL output.
    DISABLED = 0,
    ///1: Enabled. The PLL output divider (P divider) is bypassed, the PLL CCO output is used as the PLL output.
    ENABLED = 1,
}
impl From<DIRECTO_A> for bool {
    #[inline(always)]
    fn from(variant: DIRECTO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRECTO` reader - PLL0 direct output enable.
pub struct DIRECTO_R(crate::FieldReader<bool, DIRECTO_A>);
impl DIRECTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIRECTO_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DIRECTO_A {
        match self.bits {
            false => DIRECTO_A::DISABLED,
            true => DIRECTO_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DIRECTO_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DIRECTO_A::ENABLED
    }
}
impl core::ops::Deref for DIRECTO_R {
    type Target = crate::FieldReader<bool, DIRECTO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DIRECTO` writer - PLL0 direct output enable.
pub struct DIRECTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECTO_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DIRECTO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. The PLL output divider (P divider) is used to create the PLL output.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIRECTO_A::DISABLED)
    }
    ///Enabled. The PLL output divider (P divider) is bypassed, the PLL CCO output is used as the PLL output.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIRECTO_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Bandwidth select R value.
    #[inline(always)]
    pub fn selr(&self) -> SELR_R {
        SELR_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:9 - Bandwidth select I value.
    #[inline(always)]
    pub fn seli(&self) -> SELI_R {
        SELI_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    ///Bits 10:14 - Bandwidth select P value.
    #[inline(always)]
    pub fn selp(&self) -> SELP_R {
        SELP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bit 15 - PLL bypass control.
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 17 - Disable upper frequency limiter.
    #[inline(always)]
    pub fn uplimoff(&self) -> UPLIMOFF_R {
        UPLIMOFF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 19 - PLL0 direct input enable.
    #[inline(always)]
    pub fn directi(&self) -> DIRECTI_R {
        DIRECTI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - PLL0 direct output enable.
    #[inline(always)]
    pub fn directo(&self) -> DIRECTO_R {
        DIRECTO_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:3 - Bandwidth select R value.
    #[inline(always)]
    pub fn selr(&mut self) -> SELR_W {
        SELR_W { w: self }
    }
    ///Bits 4:9 - Bandwidth select I value.
    #[inline(always)]
    pub fn seli(&mut self) -> SELI_W {
        SELI_W { w: self }
    }
    ///Bits 10:14 - Bandwidth select P value.
    #[inline(always)]
    pub fn selp(&mut self) -> SELP_W {
        SELP_W { w: self }
    }
    ///Bit 15 - PLL bypass control.
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    ///Bit 17 - Disable upper frequency limiter.
    #[inline(always)]
    pub fn uplimoff(&mut self) -> UPLIMOFF_W {
        UPLIMOFF_W { w: self }
    }
    ///Bit 19 - PLL0 direct input enable.
    #[inline(always)]
    pub fn directi(&mut self) -> DIRECTI_W {
        DIRECTI_W { w: self }
    }
    ///Bit 20 - PLL0 direct output enable.
    #[inline(always)]
    pub fn directo(&mut self) -> DIRECTO_W {
        DIRECTO_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///System PLL control
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [syspllctrl](index.html) module
pub struct SYSPLLCTRL_SPEC;
impl crate::RegisterSpec for SYSPLLCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [syspllctrl::R](R) reader structure
impl crate::Readable for SYSPLLCTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [syspllctrl::W](W) writer structure
impl crate::Writable for SYSPLLCTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets SYSPLLCTRL to value 0
impl crate::Resettable for SYSPLLCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
