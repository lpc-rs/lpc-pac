///Register `TCR` reader
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TCR` writer
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
///Counter enable.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN_A {
    ///0: Disabled.The counters are disabled.
    DISABLED = 0,
    ///1: Enabled. The Timer Counter and Prescale Counter are enabled.
    ENABLED = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CEN` reader - Counter enable.
pub struct CEN_R(crate::FieldReader<bool, CEN_A>);
impl CEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::DISABLED,
            true => CEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CEN_A::ENABLED
    }
}
impl core::ops::Deref for CEN_R {
    type Target = crate::FieldReader<bool, CEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CEN` writer - Counter enable.
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled.The counters are disabled.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEN_A::DISABLED)
    }
    ///Enabled. The Timer Counter and Prescale Counter are enabled.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Counter reset.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRST_A {
    ///0: Disabled. Do nothing.
    DISABLED = 0,
    ///1: Enabled. The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of the APB bus clock. The counters remain reset until TCR\[1\]
    ///is returned to zero.
    ENABLED = 1,
}
impl From<CRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CRST` reader - Counter reset.
pub struct CRST_R(crate::FieldReader<bool, CRST_A>);
impl CRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRST_A {
        match self.bits {
            false => CRST_A::DISABLED,
            true => CRST_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CRST_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CRST_A::ENABLED
    }
}
impl core::ops::Deref for CRST_R {
    type Target = crate::FieldReader<bool, CRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CRST` writer - Counter reset.
pub struct CRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. Do nothing.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRST_A::DISABLED)
    }
    ///Enabled. The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of the APB bus clock. The counters remain reset until TCR\[1\]
    ///is returned to zero.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRST_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    ///Bit 0 - Counter enable.
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Counter reset.
    #[inline(always)]
    pub fn crst(&self) -> CRST_R {
        CRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Counter enable.
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    ///Bit 1 - Counter reset.
    #[inline(always)]
    pub fn crst(&mut self) -> CRST_W {
        CRST_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tcr](index.html) module
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tcr::R](R) reader structure
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tcr::W](W) writer structure
impl crate::Writable for TCR_SPEC {
    type Writer = W;
}
///`reset()` method sets TCR to value 0
impl crate::Resettable for TCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
