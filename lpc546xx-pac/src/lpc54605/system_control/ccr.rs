///Register `CCR` reader
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR` writer
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
///no description available
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NONBASETHRDENA_A {
    ///0: processor can enter Thread mode only when no exception is active
    NONBASETHRDENA_0 = 0,
    ///1: processor can enter Thread mode from any level under the control of an EXC_RETURN value
    NONBASETHRDENA_1 = 1,
}
impl From<NONBASETHRDENA_A> for bool {
    #[inline(always)]
    fn from(variant: NONBASETHRDENA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NONBASETHRDENA` reader - no description available
pub struct NONBASETHRDENA_R(crate::FieldReader<bool, NONBASETHRDENA_A>);
impl NONBASETHRDENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NONBASETHRDENA_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NONBASETHRDENA_A {
        match self.bits {
            false => NONBASETHRDENA_A::NONBASETHRDENA_0,
            true => NONBASETHRDENA_A::NONBASETHRDENA_1,
        }
    }
    ///Checks if the value of the field is `NONBASETHRDENA_0`
    #[inline(always)]
    pub fn is_nonbasethrdena_0(&self) -> bool {
        **self == NONBASETHRDENA_A::NONBASETHRDENA_0
    }
    ///Checks if the value of the field is `NONBASETHRDENA_1`
    #[inline(always)]
    pub fn is_nonbasethrdena_1(&self) -> bool {
        **self == NONBASETHRDENA_A::NONBASETHRDENA_1
    }
}
impl core::ops::Deref for NONBASETHRDENA_R {
    type Target = crate::FieldReader<bool, NONBASETHRDENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NONBASETHRDENA` writer - no description available
pub struct NONBASETHRDENA_W<'a> {
    w: &'a mut W,
}
impl<'a> NONBASETHRDENA_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NONBASETHRDENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///processor can enter Thread mode only when no exception is active
    #[inline(always)]
    pub fn nonbasethrdena_0(self) -> &'a mut W {
        self.variant(NONBASETHRDENA_A::NONBASETHRDENA_0)
    }
    ///processor can enter Thread mode from any level under the control of an EXC_RETURN value
    #[inline(always)]
    pub fn nonbasethrdena_1(self) -> &'a mut W {
        self.variant(NONBASETHRDENA_A::NONBASETHRDENA_1)
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
///Enables unprivileged software access to the STIR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERSETMPEND_A {
    ///0: disable
    USERSETMPEND_0 = 0,
    ///1: enable
    USERSETMPEND_1 = 1,
}
impl From<USERSETMPEND_A> for bool {
    #[inline(always)]
    fn from(variant: USERSETMPEND_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USERSETMPEND` reader - Enables unprivileged software access to the STIR
pub struct USERSETMPEND_R(crate::FieldReader<bool, USERSETMPEND_A>);
impl USERSETMPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USERSETMPEND_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USERSETMPEND_A {
        match self.bits {
            false => USERSETMPEND_A::USERSETMPEND_0,
            true => USERSETMPEND_A::USERSETMPEND_1,
        }
    }
    ///Checks if the value of the field is `USERSETMPEND_0`
    #[inline(always)]
    pub fn is_usersetmpend_0(&self) -> bool {
        **self == USERSETMPEND_A::USERSETMPEND_0
    }
    ///Checks if the value of the field is `USERSETMPEND_1`
    #[inline(always)]
    pub fn is_usersetmpend_1(&self) -> bool {
        **self == USERSETMPEND_A::USERSETMPEND_1
    }
}
impl core::ops::Deref for USERSETMPEND_R {
    type Target = crate::FieldReader<bool, USERSETMPEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USERSETMPEND` writer - Enables unprivileged software access to the STIR
pub struct USERSETMPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> USERSETMPEND_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USERSETMPEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///disable
    #[inline(always)]
    pub fn usersetmpend_0(self) -> &'a mut W {
        self.variant(USERSETMPEND_A::USERSETMPEND_0)
    }
    ///enable
    #[inline(always)]
    pub fn usersetmpend_1(self) -> &'a mut W {
        self.variant(USERSETMPEND_A::USERSETMPEND_1)
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
///Enables unaligned access traps
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGN_TRP_A {
    ///0: do not trap unaligned halfword and word accesses
    UNALIGN_TRP_0 = 0,
    ///1: trap unaligned halfword and word accesses
    UNALIGN_TRP_1 = 1,
}
impl From<UNALIGN_TRP_A> for bool {
    #[inline(always)]
    fn from(variant: UNALIGN_TRP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UNALIGN_TRP` reader - Enables unaligned access traps
pub struct UNALIGN_TRP_R(crate::FieldReader<bool, UNALIGN_TRP_A>);
impl UNALIGN_TRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNALIGN_TRP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UNALIGN_TRP_A {
        match self.bits {
            false => UNALIGN_TRP_A::UNALIGN_TRP_0,
            true => UNALIGN_TRP_A::UNALIGN_TRP_1,
        }
    }
    ///Checks if the value of the field is `UNALIGN_TRP_0`
    #[inline(always)]
    pub fn is_unalign_trp_0(&self) -> bool {
        **self == UNALIGN_TRP_A::UNALIGN_TRP_0
    }
    ///Checks if the value of the field is `UNALIGN_TRP_1`
    #[inline(always)]
    pub fn is_unalign_trp_1(&self) -> bool {
        **self == UNALIGN_TRP_A::UNALIGN_TRP_1
    }
}
impl core::ops::Deref for UNALIGN_TRP_R {
    type Target = crate::FieldReader<bool, UNALIGN_TRP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UNALIGN_TRP` writer - Enables unaligned access traps
pub struct UNALIGN_TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> UNALIGN_TRP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UNALIGN_TRP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///do not trap unaligned halfword and word accesses
    #[inline(always)]
    pub fn unalign_trp_0(self) -> &'a mut W {
        self.variant(UNALIGN_TRP_A::UNALIGN_TRP_0)
    }
    ///trap unaligned halfword and word accesses
    #[inline(always)]
    pub fn unalign_trp_1(self) -> &'a mut W {
        self.variant(UNALIGN_TRP_A::UNALIGN_TRP_1)
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
///Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV_0_TRP_A {
    ///0: do not trap divide by 0
    DIV_0_TRP_0 = 0,
    ///1: trap divide by 0
    DIV_0_TRP_1 = 1,
}
impl From<DIV_0_TRP_A> for bool {
    #[inline(always)]
    fn from(variant: DIV_0_TRP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIV_0_TRP` reader - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0
pub struct DIV_0_TRP_R(crate::FieldReader<bool, DIV_0_TRP_A>);
impl DIV_0_TRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIV_0_TRP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DIV_0_TRP_A {
        match self.bits {
            false => DIV_0_TRP_A::DIV_0_TRP_0,
            true => DIV_0_TRP_A::DIV_0_TRP_1,
        }
    }
    ///Checks if the value of the field is `DIV_0_TRP_0`
    #[inline(always)]
    pub fn is_div_0_trp_0(&self) -> bool {
        **self == DIV_0_TRP_A::DIV_0_TRP_0
    }
    ///Checks if the value of the field is `DIV_0_TRP_1`
    #[inline(always)]
    pub fn is_div_0_trp_1(&self) -> bool {
        **self == DIV_0_TRP_A::DIV_0_TRP_1
    }
}
impl core::ops::Deref for DIV_0_TRP_R {
    type Target = crate::FieldReader<bool, DIV_0_TRP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DIV_0_TRP` writer - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0
pub struct DIV_0_TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_0_TRP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DIV_0_TRP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///do not trap divide by 0
    #[inline(always)]
    pub fn div_0_trp_0(self) -> &'a mut W {
        self.variant(DIV_0_TRP_A::DIV_0_TRP_0)
    }
    ///trap divide by 0
    #[inline(always)]
    pub fn div_0_trp_1(self) -> &'a mut W {
        self.variant(DIV_0_TRP_A::DIV_0_TRP_1)
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
///Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFHFNMIGN_A {
    ///0: data bus faults caused by load and store instructions cause a lock-up
    BFHFNMIGN_0 = 0,
    ///1: handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions
    BFHFNMIGN_1 = 1,
}
impl From<BFHFNMIGN_A> for bool {
    #[inline(always)]
    fn from(variant: BFHFNMIGN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BFHFNMIGN` reader - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions.
pub struct BFHFNMIGN_R(crate::FieldReader<bool, BFHFNMIGN_A>);
impl BFHFNMIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BFHFNMIGN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BFHFNMIGN_A {
        match self.bits {
            false => BFHFNMIGN_A::BFHFNMIGN_0,
            true => BFHFNMIGN_A::BFHFNMIGN_1,
        }
    }
    ///Checks if the value of the field is `BFHFNMIGN_0`
    #[inline(always)]
    pub fn is_bfhfnmign_0(&self) -> bool {
        **self == BFHFNMIGN_A::BFHFNMIGN_0
    }
    ///Checks if the value of the field is `BFHFNMIGN_1`
    #[inline(always)]
    pub fn is_bfhfnmign_1(&self) -> bool {
        **self == BFHFNMIGN_A::BFHFNMIGN_1
    }
}
impl core::ops::Deref for BFHFNMIGN_R {
    type Target = crate::FieldReader<bool, BFHFNMIGN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BFHFNMIGN` writer - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions.
pub struct BFHFNMIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> BFHFNMIGN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BFHFNMIGN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///data bus faults caused by load and store instructions cause a lock-up
    #[inline(always)]
    pub fn bfhfnmign_0(self) -> &'a mut W {
        self.variant(BFHFNMIGN_A::BFHFNMIGN_0)
    }
    ///handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions
    #[inline(always)]
    pub fn bfhfnmign_1(self) -> &'a mut W {
        self.variant(BFHFNMIGN_A::BFHFNMIGN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Indicates stack alignment on exception entry
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKALIGN_A {
    ///0: 4-byte aligned
    STKALIGN_0 = 0,
    ///1: 8-byte aligned
    STKALIGN_1 = 1,
}
impl From<STKALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: STKALIGN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `STKALIGN` reader - Indicates stack alignment on exception entry
pub struct STKALIGN_R(crate::FieldReader<bool, STKALIGN_A>);
impl STKALIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STKALIGN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STKALIGN_A {
        match self.bits {
            false => STKALIGN_A::STKALIGN_0,
            true => STKALIGN_A::STKALIGN_1,
        }
    }
    ///Checks if the value of the field is `STKALIGN_0`
    #[inline(always)]
    pub fn is_stkalign_0(&self) -> bool {
        **self == STKALIGN_A::STKALIGN_0
    }
    ///Checks if the value of the field is `STKALIGN_1`
    #[inline(always)]
    pub fn is_stkalign_1(&self) -> bool {
        **self == STKALIGN_A::STKALIGN_1
    }
}
impl core::ops::Deref for STKALIGN_R {
    type Target = crate::FieldReader<bool, STKALIGN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `STKALIGN` writer - Indicates stack alignment on exception entry
pub struct STKALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> STKALIGN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STKALIGN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///4-byte aligned
    #[inline(always)]
    pub fn stkalign_0(self) -> &'a mut W {
        self.variant(STKALIGN_A::STKALIGN_0)
    }
    ///8-byte aligned
    #[inline(always)]
    pub fn stkalign_1(self) -> &'a mut W {
        self.variant(STKALIGN_A::STKALIGN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    ///Bit 0 - no description available
    #[inline(always)]
    pub fn nonbasethrdena(&self) -> NONBASETHRDENA_R {
        NONBASETHRDENA_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Enables unprivileged software access to the STIR
    #[inline(always)]
    pub fn usersetmpend(&self) -> USERSETMPEND_R {
        USERSETMPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 3 - Enables unaligned access traps
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0
    #[inline(always)]
    pub fn div_0_trp(&self) -> DIV_0_TRP_R {
        DIV_0_TRP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions.
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BFHFNMIGN_R {
        BFHFNMIGN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Indicates stack alignment on exception entry
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - no description available
    #[inline(always)]
    pub fn nonbasethrdena(&mut self) -> NONBASETHRDENA_W {
        NONBASETHRDENA_W { w: self }
    }
    ///Bit 1 - Enables unprivileged software access to the STIR
    #[inline(always)]
    pub fn usersetmpend(&mut self) -> USERSETMPEND_W {
        USERSETMPEND_W { w: self }
    }
    ///Bit 3 - Enables unaligned access traps
    #[inline(always)]
    pub fn unalign_trp(&mut self) -> UNALIGN_TRP_W {
        UNALIGN_TRP_W { w: self }
    }
    ///Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0
    #[inline(always)]
    pub fn div_0_trp(&mut self) -> DIV_0_TRP_W {
        DIV_0_TRP_W { w: self }
    }
    ///Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions.
    #[inline(always)]
    pub fn bfhfnmign(&mut self) -> BFHFNMIGN_W {
        BFHFNMIGN_W { w: self }
    }
    ///Bit 9 - Indicates stack alignment on exception entry
    #[inline(always)]
    pub fn stkalign(&mut self) -> STKALIGN_W {
        STKALIGN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Configuration and Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr](index.html) module
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr::R](R) reader structure
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr::W](W) writer structure
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
