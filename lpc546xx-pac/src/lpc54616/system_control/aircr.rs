///Register `AIRCR` reader
pub struct R(crate::R<AIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIRCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AIRCR` writer
pub struct W(crate::W<AIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIRCR_SPEC>;
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
impl From<crate::W<AIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIRCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VECTRESET` writer - no description available
pub struct VECTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTRESET_W<'a> {
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
///Field `VECTCLRACTIVE` writer - no description available
pub struct VECTCLRACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTCLRACTIVE_W<'a> {
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
pub enum SYSRESETREQ_AW {
    ///0: no system reset request
    SYSRESETREQ_0 = 0,
    ///1: asserts a signal to the outer system that requests a reset
    SYSRESETREQ_1 = 1,
}
impl From<SYSRESETREQ_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSRESETREQ_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSRESETREQ` writer - no description available
pub struct SYSRESETREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRESETREQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SYSRESETREQ_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///no system reset request
    #[inline(always)]
    pub fn sysresetreq_0(self) -> &'a mut W {
        self.variant(SYSRESETREQ_AW::SYSRESETREQ_0)
    }
    ///asserts a signal to the outer system that requests a reset
    #[inline(always)]
    pub fn sysresetreq_1(self) -> &'a mut W {
        self.variant(SYSRESETREQ_AW::SYSRESETREQ_1)
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
///Field `PRIGROUP` reader - Interrupt priority grouping field. This field determines the split of group priority from subpriority.
pub struct PRIGROUP_R(crate::FieldReader<u8, u8>);
impl PRIGROUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRIGROUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIGROUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PRIGROUP` writer - Interrupt priority grouping field. This field determines the split of group priority from subpriority.
pub struct PRIGROUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIGROUP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
///no description available
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANNESS_A {
    ///0: Little-endian
    ENDIANNESS_0 = 0,
    ///1: Big-endian
    ENDIANNESS_1 = 1,
}
impl From<ENDIANNESS_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIANNESS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENDIANNESS` reader - no description available
pub struct ENDIANNESS_R(crate::FieldReader<bool, ENDIANNESS_A>);
impl ENDIANNESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDIANNESS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENDIANNESS_A {
        match self.bits {
            false => ENDIANNESS_A::ENDIANNESS_0,
            true => ENDIANNESS_A::ENDIANNESS_1,
        }
    }
    ///Checks if the value of the field is `ENDIANNESS_0`
    #[inline(always)]
    pub fn is_endianness_0(&self) -> bool {
        **self == ENDIANNESS_A::ENDIANNESS_0
    }
    ///Checks if the value of the field is `ENDIANNESS_1`
    #[inline(always)]
    pub fn is_endianness_1(&self) -> bool {
        **self == ENDIANNESS_A::ENDIANNESS_1
    }
}
impl core::ops::Deref for ENDIANNESS_R {
    type Target = crate::FieldReader<bool, ENDIANNESS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `VECTKEY` reader - Register key
pub struct VECTKEY_R(crate::FieldReader<u16, u16>);
impl VECTKEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        VECTKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VECTKEY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `VECTKEY` writer - Register key
pub struct VECTKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTKEY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 8:10 - Interrupt priority grouping field. This field determines the split of group priority from subpriority.
    #[inline(always)]
    pub fn prigroup(&self) -> PRIGROUP_R {
        PRIGROUP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bit 15 - no description available
    #[inline(always)]
    pub fn endianness(&self) -> ENDIANNESS_R {
        ENDIANNESS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 16:31 - Register key
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bit 0 - no description available
    #[inline(always)]
    pub fn vectreset(&mut self) -> VECTRESET_W {
        VECTRESET_W { w: self }
    }
    ///Bit 1 - no description available
    #[inline(always)]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W {
        VECTCLRACTIVE_W { w: self }
    }
    ///Bit 2 - no description available
    #[inline(always)]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W {
        SYSRESETREQ_W { w: self }
    }
    ///Bits 8:10 - Interrupt priority grouping field. This field determines the split of group priority from subpriority.
    #[inline(always)]
    pub fn prigroup(&mut self) -> PRIGROUP_W {
        PRIGROUP_W { w: self }
    }
    ///Bits 16:31 - Register key
    #[inline(always)]
    pub fn vectkey(&mut self) -> VECTKEY_W {
        VECTKEY_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Application Interrupt and Reset Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aircr](index.html) module
pub struct AIRCR_SPEC;
impl crate::RegisterSpec for AIRCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [aircr::R](R) reader structure
impl crate::Readable for AIRCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [aircr::W](W) writer structure
impl crate::Writable for AIRCR_SPEC {
    type Writer = W;
}
///`reset()` method sets AIRCR to value 0xfa05_0000
impl crate::Resettable for AIRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfa05_0000
    }
}
