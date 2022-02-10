///Register `HFSR` reader
pub struct R(crate::R<HFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HFSR` writer
pub struct W(crate::W<HFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFSR_SPEC>;
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
impl From<crate::W<HFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFSR_SPEC>) -> Self {
        W(writer)
    }
}
///no description available
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTTBL_A {
    ///0: no BusFault on vector table read
    VECTTBL_0 = 0,
    ///1: BusFault on vector table read
    VECTTBL_1 = 1,
}
impl From<VECTTBL_A> for bool {
    #[inline(always)]
    fn from(variant: VECTTBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VECTTBL` reader - no description available
pub struct VECTTBL_R(crate::FieldReader<bool, VECTTBL_A>);
impl VECTTBL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VECTTBL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VECTTBL_A {
        match self.bits {
            false => VECTTBL_A::VECTTBL_0,
            true => VECTTBL_A::VECTTBL_1,
        }
    }
    ///Checks if the value of the field is `VECTTBL_0`
    #[inline(always)]
    pub fn is_vecttbl_0(&self) -> bool {
        **self == VECTTBL_A::VECTTBL_0
    }
    ///Checks if the value of the field is `VECTTBL_1`
    #[inline(always)]
    pub fn is_vecttbl_1(&self) -> bool {
        **self == VECTTBL_A::VECTTBL_1
    }
}
impl core::ops::Deref for VECTTBL_R {
    type Target = crate::FieldReader<bool, VECTTBL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `VECTTBL` writer - no description available
pub struct VECTTBL_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTTBL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: VECTTBL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///no BusFault on vector table read
    #[inline(always)]
    pub fn vecttbl_0(self) -> &'a mut W {
        self.variant(VECTTBL_A::VECTTBL_0)
    }
    ///BusFault on vector table read
    #[inline(always)]
    pub fn vecttbl_1(self) -> &'a mut W {
        self.variant(VECTTBL_A::VECTTBL_1)
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
pub enum FORCED_A {
    ///0: no forced HardFault
    FORCED_0 = 0,
    ///1: forced HardFault
    FORCED_1 = 1,
}
impl From<FORCED_A> for bool {
    #[inline(always)]
    fn from(variant: FORCED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FORCED` reader - no description available
pub struct FORCED_R(crate::FieldReader<bool, FORCED_A>);
impl FORCED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCED_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FORCED_A {
        match self.bits {
            false => FORCED_A::FORCED_0,
            true => FORCED_A::FORCED_1,
        }
    }
    ///Checks if the value of the field is `FORCED_0`
    #[inline(always)]
    pub fn is_forced_0(&self) -> bool {
        **self == FORCED_A::FORCED_0
    }
    ///Checks if the value of the field is `FORCED_1`
    #[inline(always)]
    pub fn is_forced_1(&self) -> bool {
        **self == FORCED_A::FORCED_1
    }
}
impl core::ops::Deref for FORCED_R {
    type Target = crate::FieldReader<bool, FORCED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FORCED` writer - no description available
pub struct FORCED_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCED_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FORCED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///no forced HardFault
    #[inline(always)]
    pub fn forced_0(self) -> &'a mut W {
        self.variant(FORCED_A::FORCED_0)
    }
    ///forced HardFault
    #[inline(always)]
    pub fn forced_1(self) -> &'a mut W {
        self.variant(FORCED_A::FORCED_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
///Field `DEBUGEVT` reader - no description available
pub struct DEBUGEVT_R(crate::FieldReader<bool, bool>);
impl DEBUGEVT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEBUGEVT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUGEVT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DEBUGEVT` writer - no description available
pub struct DEBUGEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGEVT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bit 1 - no description available
    #[inline(always)]
    pub fn vecttbl(&self) -> VECTTBL_R {
        VECTTBL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 30 - no description available
    #[inline(always)]
    pub fn forced(&self) -> FORCED_R {
        FORCED_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - no description available
    #[inline(always)]
    pub fn debugevt(&self) -> DEBUGEVT_R {
        DEBUGEVT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - no description available
    #[inline(always)]
    pub fn vecttbl(&mut self) -> VECTTBL_W {
        VECTTBL_W { w: self }
    }
    ///Bit 30 - no description available
    #[inline(always)]
    pub fn forced(&mut self) -> FORCED_W {
        FORCED_W { w: self }
    }
    ///Bit 31 - no description available
    #[inline(always)]
    pub fn debugevt(&mut self) -> DEBUGEVT_W {
        DEBUGEVT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HardFault Status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hfsr](index.html) module
pub struct HFSR_SPEC;
impl crate::RegisterSpec for HFSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hfsr::R](R) reader structure
impl crate::Readable for HFSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hfsr::W](W) writer structure
impl crate::Writable for HFSR_SPEC {
    type Writer = W;
}
///`reset()` method sets HFSR to value 0
impl crate::Resettable for HFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
