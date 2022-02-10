///Register `DFSR` reader
pub struct R(crate::R<DFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DFSR` writer
pub struct W(crate::W<DFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSR_SPEC>;
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
impl From<crate::W<DFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSR_SPEC>) -> Self {
        W(writer)
    }
}
///no description available
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALTED_A {
    ///0: No active halt request debug event
    HALTED_0 = 0,
    ///1: Halt request debug event active
    HALTED_1 = 1,
}
impl From<HALTED_A> for bool {
    #[inline(always)]
    fn from(variant: HALTED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HALTED` reader - no description available
pub struct HALTED_R(crate::FieldReader<bool, HALTED_A>);
impl HALTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALTED_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HALTED_A {
        match self.bits {
            false => HALTED_A::HALTED_0,
            true => HALTED_A::HALTED_1,
        }
    }
    ///Checks if the value of the field is `HALTED_0`
    #[inline(always)]
    pub fn is_halted_0(&self) -> bool {
        **self == HALTED_A::HALTED_0
    }
    ///Checks if the value of the field is `HALTED_1`
    #[inline(always)]
    pub fn is_halted_1(&self) -> bool {
        **self == HALTED_A::HALTED_1
    }
}
impl core::ops::Deref for HALTED_R {
    type Target = crate::FieldReader<bool, HALTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HALTED` writer - no description available
pub struct HALTED_W<'a> {
    w: &'a mut W,
}
impl<'a> HALTED_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HALTED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No active halt request debug event
    #[inline(always)]
    pub fn halted_0(self) -> &'a mut W {
        self.variant(HALTED_A::HALTED_0)
    }
    ///Halt request debug event active
    #[inline(always)]
    pub fn halted_1(self) -> &'a mut W {
        self.variant(HALTED_A::HALTED_1)
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
///no description available
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKPT_A {
    ///0: No current breakpoint debug event
    BKPT_0 = 0,
    ///1: At least one current breakpoint debug event
    BKPT_1 = 1,
}
impl From<BKPT_A> for bool {
    #[inline(always)]
    fn from(variant: BKPT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BKPT` reader - no description available
pub struct BKPT_R(crate::FieldReader<bool, BKPT_A>);
impl BKPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BKPT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKPT_A {
        match self.bits {
            false => BKPT_A::BKPT_0,
            true => BKPT_A::BKPT_1,
        }
    }
    ///Checks if the value of the field is `BKPT_0`
    #[inline(always)]
    pub fn is_bkpt_0(&self) -> bool {
        **self == BKPT_A::BKPT_0
    }
    ///Checks if the value of the field is `BKPT_1`
    #[inline(always)]
    pub fn is_bkpt_1(&self) -> bool {
        **self == BKPT_A::BKPT_1
    }
}
impl core::ops::Deref for BKPT_R {
    type Target = crate::FieldReader<bool, BKPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKPT` writer - no description available
pub struct BKPT_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKPT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No current breakpoint debug event
    #[inline(always)]
    pub fn bkpt_0(self) -> &'a mut W {
        self.variant(BKPT_A::BKPT_0)
    }
    ///At least one current breakpoint debug event
    #[inline(always)]
    pub fn bkpt_1(self) -> &'a mut W {
        self.variant(BKPT_A::BKPT_1)
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
///no description available
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWTTRAP_A {
    ///0: No current debug events generated by the DWT
    DWTTRAP_0 = 0,
    ///1: At least one current debug event generated by the DWT
    DWTTRAP_1 = 1,
}
impl From<DWTTRAP_A> for bool {
    #[inline(always)]
    fn from(variant: DWTTRAP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DWTTRAP` reader - no description available
pub struct DWTTRAP_R(crate::FieldReader<bool, DWTTRAP_A>);
impl DWTTRAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DWTTRAP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DWTTRAP_A {
        match self.bits {
            false => DWTTRAP_A::DWTTRAP_0,
            true => DWTTRAP_A::DWTTRAP_1,
        }
    }
    ///Checks if the value of the field is `DWTTRAP_0`
    #[inline(always)]
    pub fn is_dwttrap_0(&self) -> bool {
        **self == DWTTRAP_A::DWTTRAP_0
    }
    ///Checks if the value of the field is `DWTTRAP_1`
    #[inline(always)]
    pub fn is_dwttrap_1(&self) -> bool {
        **self == DWTTRAP_A::DWTTRAP_1
    }
}
impl core::ops::Deref for DWTTRAP_R {
    type Target = crate::FieldReader<bool, DWTTRAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DWTTRAP` writer - no description available
pub struct DWTTRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DWTTRAP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DWTTRAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No current debug events generated by the DWT
    #[inline(always)]
    pub fn dwttrap_0(self) -> &'a mut W {
        self.variant(DWTTRAP_A::DWTTRAP_0)
    }
    ///At least one current debug event generated by the DWT
    #[inline(always)]
    pub fn dwttrap_1(self) -> &'a mut W {
        self.variant(DWTTRAP_A::DWTTRAP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///no description available
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCATCH_A {
    ///0: No Vector catch triggered
    VCATCH_0 = 0,
    ///1: Vector catch triggered
    VCATCH_1 = 1,
}
impl From<VCATCH_A> for bool {
    #[inline(always)]
    fn from(variant: VCATCH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VCATCH` reader - no description available
pub struct VCATCH_R(crate::FieldReader<bool, VCATCH_A>);
impl VCATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCATCH_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VCATCH_A {
        match self.bits {
            false => VCATCH_A::VCATCH_0,
            true => VCATCH_A::VCATCH_1,
        }
    }
    ///Checks if the value of the field is `VCATCH_0`
    #[inline(always)]
    pub fn is_vcatch_0(&self) -> bool {
        **self == VCATCH_A::VCATCH_0
    }
    ///Checks if the value of the field is `VCATCH_1`
    #[inline(always)]
    pub fn is_vcatch_1(&self) -> bool {
        **self == VCATCH_A::VCATCH_1
    }
}
impl core::ops::Deref for VCATCH_R {
    type Target = crate::FieldReader<bool, VCATCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `VCATCH` writer - no description available
pub struct VCATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> VCATCH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: VCATCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No Vector catch triggered
    #[inline(always)]
    pub fn vcatch_0(self) -> &'a mut W {
        self.variant(VCATCH_A::VCATCH_0)
    }
    ///Vector catch triggered
    #[inline(always)]
    pub fn vcatch_1(self) -> &'a mut W {
        self.variant(VCATCH_A::VCATCH_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///no description available
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTERNAL_A {
    ///0: No EDBGRQ debug event
    EXTERNAL_0 = 0,
    ///1: EDBGRQ debug event
    EXTERNAL_1 = 1,
}
impl From<EXTERNAL_A> for bool {
    #[inline(always)]
    fn from(variant: EXTERNAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTERNAL` reader - no description available
pub struct EXTERNAL_R(crate::FieldReader<bool, EXTERNAL_A>);
impl EXTERNAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTERNAL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTERNAL_A {
        match self.bits {
            false => EXTERNAL_A::EXTERNAL_0,
            true => EXTERNAL_A::EXTERNAL_1,
        }
    }
    ///Checks if the value of the field is `EXTERNAL_0`
    #[inline(always)]
    pub fn is_external_0(&self) -> bool {
        **self == EXTERNAL_A::EXTERNAL_0
    }
    ///Checks if the value of the field is `EXTERNAL_1`
    #[inline(always)]
    pub fn is_external_1(&self) -> bool {
        **self == EXTERNAL_A::EXTERNAL_1
    }
}
impl core::ops::Deref for EXTERNAL_R {
    type Target = crate::FieldReader<bool, EXTERNAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTERNAL` writer - no description available
pub struct EXTERNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTERNAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No EDBGRQ debug event
    #[inline(always)]
    pub fn external_0(self) -> &'a mut W {
        self.variant(EXTERNAL_A::EXTERNAL_0)
    }
    ///EDBGRQ debug event
    #[inline(always)]
    pub fn external_1(self) -> &'a mut W {
        self.variant(EXTERNAL_A::EXTERNAL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    ///Bit 0 - no description available
    #[inline(always)]
    pub fn halted(&self) -> HALTED_R {
        HALTED_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - no description available
    #[inline(always)]
    pub fn bkpt(&self) -> BKPT_R {
        BKPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - no description available
    #[inline(always)]
    pub fn dwttrap(&self) -> DWTTRAP_R {
        DWTTRAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - no description available
    #[inline(always)]
    pub fn vcatch(&self) -> VCATCH_R {
        VCATCH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - no description available
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - no description available
    #[inline(always)]
    pub fn halted(&mut self) -> HALTED_W {
        HALTED_W { w: self }
    }
    ///Bit 1 - no description available
    #[inline(always)]
    pub fn bkpt(&mut self) -> BKPT_W {
        BKPT_W { w: self }
    }
    ///Bit 2 - no description available
    #[inline(always)]
    pub fn dwttrap(&mut self) -> DWTTRAP_W {
        DWTTRAP_W { w: self }
    }
    ///Bit 3 - no description available
    #[inline(always)]
    pub fn vcatch(&mut self) -> VCATCH_W {
        VCATCH_W { w: self }
    }
    ///Bit 4 - no description available
    #[inline(always)]
    pub fn external(&mut self) -> EXTERNAL_W {
        EXTERNAL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Debug Fault Status Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsr](index.html) module
pub struct DFSR_SPEC;
impl crate::RegisterSpec for DFSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsr::R](R) reader structure
impl crate::Readable for DFSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dfsr::W](W) writer structure
impl crate::Writable for DFSR_SPEC {
    type Writer = W;
}
///`reset()` method sets DFSR to value 0
impl crate::Resettable for DFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
