///Register `FCTR` reader
pub struct R(crate::R<FCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FCTR` writer
pub struct W(crate::W<FCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTR_SPEC>;
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
impl From<crate::W<FCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FS_RD0` reader - Value must be 0 for signature generation.
pub struct FS_RD0_R(crate::FieldReader<bool, bool>);
impl FS_RD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FS_RD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FS_RD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FS_RD0` writer - Value must be 0 for signature generation.
pub struct FS_RD0_W<'a> {
    w: &'a mut W,
}
impl<'a> FS_RD0_W<'a> {
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
///Field `FS_RD1` reader - Value must be 1 for signature generation.
pub struct FS_RD1_R(crate::FieldReader<bool, bool>);
impl FS_RD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FS_RD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FS_RD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FS_RD1` writer - Value must be 1 for signature generation.
pub struct FS_RD1_W<'a> {
    w: &'a mut W,
}
impl<'a> FS_RD1_W<'a> {
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
    ///Bit 3 - Value must be 0 for signature generation.
    #[inline(always)]
    pub fn fs_rd0(&self) -> FS_RD0_R {
        FS_RD0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Value must be 1 for signature generation.
    #[inline(always)]
    pub fn fs_rd1(&self) -> FS_RD1_R {
        FS_RD1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    ///Bit 3 - Value must be 0 for signature generation.
    #[inline(always)]
    pub fn fs_rd0(&mut self) -> FS_RD0_W {
        FS_RD0_W { w: self }
    }
    ///Bit 4 - Value must be 1 for signature generation.
    #[inline(always)]
    pub fn fs_rd1(&mut self) -> FS_RD1_W {
        FS_RD1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fctr](index.html) module
pub struct FCTR_SPEC;
impl crate::RegisterSpec for FCTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fctr::R](R) reader structure
impl crate::Readable for FCTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fctr::W](W) writer structure
impl crate::Writable for FCTR_SPEC {
    type Writer = W;
}
///`reset()` method sets FCTR to value 0x0020_0005
impl crate::Resettable for FCTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_0005
    }
}
