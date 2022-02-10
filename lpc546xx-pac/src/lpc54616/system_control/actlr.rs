///Register `ACTLR` reader
pub struct R(crate::R<ACTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTLR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ACTLR` writer
pub struct W(crate::W<ACTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTLR_SPEC>;
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
impl From<crate::W<ACTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DISMCYCINT` reader - Disables interruption of multi-cycle instructions.
pub struct DISMCYCINT_R(crate::FieldReader<bool, bool>);
impl DISMCYCINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISMCYCINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISMCYCINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DISMCYCINT` writer - Disables interruption of multi-cycle instructions.
pub struct DISMCYCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISMCYCINT_W<'a> {
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
///Field `DISDEFWBUF` reader - Disables write buffer use during default memory map accesses.
pub struct DISDEFWBUF_R(crate::FieldReader<bool, bool>);
impl DISDEFWBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISDEFWBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISDEFWBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DISDEFWBUF` writer - Disables write buffer use during default memory map accesses.
pub struct DISDEFWBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DISDEFWBUF_W<'a> {
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
///Field `DISFOLD` reader - Disables folding of IT instructions.
pub struct DISFOLD_R(crate::FieldReader<bool, bool>);
impl DISFOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISFOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISFOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DISFOLD` writer - Disables folding of IT instructions.
pub struct DISFOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFOLD_W<'a> {
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
impl R {
    ///Bit 0 - Disables interruption of multi-cycle instructions.
    #[inline(always)]
    pub fn dismcycint(&self) -> DISMCYCINT_R {
        DISMCYCINT_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Disables write buffer use during default memory map accesses.
    #[inline(always)]
    pub fn disdefwbuf(&self) -> DISDEFWBUF_R {
        DISDEFWBUF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Disables folding of IT instructions.
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Disables interruption of multi-cycle instructions.
    #[inline(always)]
    pub fn dismcycint(&mut self) -> DISMCYCINT_W {
        DISMCYCINT_W { w: self }
    }
    ///Bit 1 - Disables write buffer use during default memory map accesses.
    #[inline(always)]
    pub fn disdefwbuf(&mut self) -> DISDEFWBUF_W {
        DISDEFWBUF_W { w: self }
    }
    ///Bit 2 - Disables folding of IT instructions.
    #[inline(always)]
    pub fn disfold(&mut self) -> DISFOLD_W {
        DISFOLD_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Auxiliary Control Register,
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [actlr](index.html) module
pub struct ACTLR_SPEC;
impl crate::RegisterSpec for ACTLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [actlr::R](R) reader structure
impl crate::Readable for ACTLR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [actlr::W](W) writer structure
impl crate::Writable for ACTLR_SPEC {
    type Writer = W;
}
///`reset()` method sets ACTLR to value 0
impl crate::Resettable for ACTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
