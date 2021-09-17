#[doc = "Register `ICSR` reader"]
pub struct R(crate::R<ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSR` writer"]
pub struct W(crate::W<ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSR_SPEC>;
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
impl From<crate::W<ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTACTIVE` reader - Active exception number"]
pub struct VECTACTIVE_R(crate::FieldReader<u16, u16>);
impl VECTACTIVE_R {
    pub(crate) fn new(bits: u16) -> Self {
        VECTACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VECTACTIVE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETTOBASE_A {
    #[doc = "0: there are preempted active exceptions to execute"]
    RETTOBASE_0 = 0,
    #[doc = "1: there are no active exceptions, or the currently-executing exception is the only active exception"]
    RETTOBASE_1 = 1,
}
impl From<RETTOBASE_A> for bool {
    #[inline(always)]
    fn from(variant: RETTOBASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETTOBASE` reader - no description available"]
pub struct RETTOBASE_R(crate::FieldReader<bool, RETTOBASE_A>);
impl RETTOBASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RETTOBASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETTOBASE_A {
        match self.bits {
            false => RETTOBASE_A::RETTOBASE_0,
            true => RETTOBASE_A::RETTOBASE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RETTOBASE_0`"]
    #[inline(always)]
    pub fn is_rettobase_0(&self) -> bool {
        **self == RETTOBASE_A::RETTOBASE_0
    }
    #[doc = "Checks if the value of the field is `RETTOBASE_1`"]
    #[inline(always)]
    pub fn is_rettobase_1(&self) -> bool {
        **self == RETTOBASE_A::RETTOBASE_1
    }
}
impl core::ops::Deref for RETTOBASE_R {
    type Target = crate::FieldReader<bool, RETTOBASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VECTPENDING` reader - Exception number of the highest priority pending enabled exception"]
pub struct VECTPENDING_R(crate::FieldReader<u8, u8>);
impl VECTPENDING_R {
    pub(crate) fn new(bits: u8) -> Self {
        VECTPENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VECTPENDING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISRPENDING` reader - no description available"]
pub struct ISRPENDING_R(crate::FieldReader<bool, bool>);
impl ISRPENDING_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISRPENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISRPENDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISRPREEMPT_A {
    #[doc = "0: Will not service"]
    ISRPREEMPT_0 = 0,
    #[doc = "1: Will service a pending exception"]
    ISRPREEMPT_1 = 1,
}
impl From<ISRPREEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: ISRPREEMPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISRPREEMPT` reader - no description available"]
pub struct ISRPREEMPT_R(crate::FieldReader<bool, ISRPREEMPT_A>);
impl ISRPREEMPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISRPREEMPT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISRPREEMPT_A {
        match self.bits {
            false => ISRPREEMPT_A::ISRPREEMPT_0,
            true => ISRPREEMPT_A::ISRPREEMPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ISRPREEMPT_0`"]
    #[inline(always)]
    pub fn is_isrpreempt_0(&self) -> bool {
        **self == ISRPREEMPT_A::ISRPREEMPT_0
    }
    #[doc = "Checks if the value of the field is `ISRPREEMPT_1`"]
    #[inline(always)]
    pub fn is_isrpreempt_1(&self) -> bool {
        **self == ISRPREEMPT_A::ISRPREEMPT_1
    }
}
impl core::ops::Deref for ISRPREEMPT_R {
    type Target = crate::FieldReader<bool, ISRPREEMPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTCLR_AW {
    #[doc = "0: no effect"]
    PENDSTCLR_0 = 0,
    #[doc = "1: removes the pending state from the SysTick exception"]
    PENDSTCLR_1 = 1,
}
impl From<PENDSTCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: PENDSTCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTCLR` writer - no description available"]
pub struct PENDSTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTCLR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn pendstclr_0(self) -> &'a mut W {
        self.variant(PENDSTCLR_AW::PENDSTCLR_0)
    }
    #[doc = "removes the pending state from the SysTick exception"]
    #[inline(always)]
    pub fn pendstclr_1(self) -> &'a mut W {
        self.variant(PENDSTCLR_AW::PENDSTCLR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSET_A {
    #[doc = "0: write: no effect; read: SysTick exception is not pending"]
    PENDSTSET_0 = 0,
    #[doc = "1: write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    PENDSTSET_1 = 1,
}
impl From<PENDSTSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSTSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTSET` reader - no description available"]
pub struct PENDSTSET_R(crate::FieldReader<bool, PENDSTSET_A>);
impl PENDSTSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PENDSTSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTSET_A {
        match self.bits {
            false => PENDSTSET_A::PENDSTSET_0,
            true => PENDSTSET_A::PENDSTSET_1,
        }
    }
    #[doc = "Checks if the value of the field is `PENDSTSET_0`"]
    #[inline(always)]
    pub fn is_pendstset_0(&self) -> bool {
        **self == PENDSTSET_A::PENDSTSET_0
    }
    #[doc = "Checks if the value of the field is `PENDSTSET_1`"]
    #[inline(always)]
    pub fn is_pendstset_1(&self) -> bool {
        **self == PENDSTSET_A::PENDSTSET_1
    }
}
impl core::ops::Deref for PENDSTSET_R {
    type Target = crate::FieldReader<bool, PENDSTSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSTSET` writer - no description available"]
pub struct PENDSTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    #[inline(always)]
    pub fn pendstset_0(self) -> &'a mut W {
        self.variant(PENDSTSET_A::PENDSTSET_0)
    }
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline(always)]
    pub fn pendstset_1(self) -> &'a mut W {
        self.variant(PENDSTSET_A::PENDSTSET_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVCLR_AW {
    #[doc = "0: no effect"]
    PENDSVCLR_0 = 0,
    #[doc = "1: removes the pending state from the PendSV exception"]
    PENDSVCLR_1 = 1,
}
impl From<PENDSVCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: PENDSVCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVCLR` writer - no description available"]
pub struct PENDSVCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVCLR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn pendsvclr_0(self) -> &'a mut W {
        self.variant(PENDSVCLR_AW::PENDSVCLR_0)
    }
    #[doc = "removes the pending state from the PendSV exception"]
    #[inline(always)]
    pub fn pendsvclr_1(self) -> &'a mut W {
        self.variant(PENDSVCLR_AW::PENDSVCLR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSET_A {
    #[doc = "0: write: no effect; read: PendSV exception is not pending"]
    PENDSVSET_0 = 0,
    #[doc = "1: write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    PENDSVSET_1 = 1,
}
impl From<PENDSVSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVSET` reader - no description available"]
pub struct PENDSVSET_R(crate::FieldReader<bool, PENDSVSET_A>);
impl PENDSVSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PENDSVSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVSET_A {
        match self.bits {
            false => PENDSVSET_A::PENDSVSET_0,
            true => PENDSVSET_A::PENDSVSET_1,
        }
    }
    #[doc = "Checks if the value of the field is `PENDSVSET_0`"]
    #[inline(always)]
    pub fn is_pendsvset_0(&self) -> bool {
        **self == PENDSVSET_A::PENDSVSET_0
    }
    #[doc = "Checks if the value of the field is `PENDSVSET_1`"]
    #[inline(always)]
    pub fn is_pendsvset_1(&self) -> bool {
        **self == PENDSVSET_A::PENDSVSET_1
    }
}
impl core::ops::Deref for PENDSVSET_R {
    type Target = crate::FieldReader<bool, PENDSVSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSVSET` writer - no description available"]
pub struct PENDSVSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    #[inline(always)]
    pub fn pendsvset_0(self) -> &'a mut W {
        self.variant(PENDSVSET_A::PENDSVSET_0)
    }
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline(always)]
    pub fn pendsvset_1(self) -> &'a mut W {
        self.variant(PENDSVSET_A::PENDSVSET_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIPENDSET_A {
    #[doc = "0: write: no effect; read: NMI exception is not pending"]
    NMIPENDSET_0 = 0,
    #[doc = "1: write: changes NMI exception state to pending; read: NMI exception is pending"]
    NMIPENDSET_1 = 1,
}
impl From<NMIPENDSET_A> for bool {
    #[inline(always)]
    fn from(variant: NMIPENDSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIPENDSET` reader - no description available"]
pub struct NMIPENDSET_R(crate::FieldReader<bool, NMIPENDSET_A>);
impl NMIPENDSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        NMIPENDSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIPENDSET_A {
        match self.bits {
            false => NMIPENDSET_A::NMIPENDSET_0,
            true => NMIPENDSET_A::NMIPENDSET_1,
        }
    }
    #[doc = "Checks if the value of the field is `NMIPENDSET_0`"]
    #[inline(always)]
    pub fn is_nmipendset_0(&self) -> bool {
        **self == NMIPENDSET_A::NMIPENDSET_0
    }
    #[doc = "Checks if the value of the field is `NMIPENDSET_1`"]
    #[inline(always)]
    pub fn is_nmipendset_1(&self) -> bool {
        **self == NMIPENDSET_A::NMIPENDSET_1
    }
}
impl core::ops::Deref for NMIPENDSET_R {
    type Target = crate::FieldReader<bool, NMIPENDSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIPENDSET` writer - no description available"]
pub struct NMIPENDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIPENDSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMIPENDSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "write: no effect; read: NMI exception is not pending"]
    #[inline(always)]
    pub fn nmipendset_0(self) -> &'a mut W {
        self.variant(NMIPENDSET_A::NMIPENDSET_0)
    }
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline(always)]
    pub fn nmipendset_1(self) -> &'a mut W {
        self.variant(NMIPENDSET_A::NMIPENDSET_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn rettobase(&self) -> RETTOBASE_R {
        RETTOBASE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:17 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn isrpreempt(&self) -> ISRPREEMPT_R {
        ISRPREEMPT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W {
        PENDSTCLR_W { w: self }
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn pendstset(&mut self) -> PENDSTSET_W {
        PENDSTSET_W { w: self }
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W {
        PENDSVCLR_W { w: self }
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    pub fn pendsvset(&mut self) -> PENDSVSET_W {
        PENDSVSET_W { w: self }
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W {
        NMIPENDSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](index.html) module"]
pub struct ICSR_SPEC;
impl crate::RegisterSpec for ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icsr::R](R) reader structure"]
impl crate::Readable for ICSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icsr::W](W) writer structure"]
impl crate::Writable for ICSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for ICSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
