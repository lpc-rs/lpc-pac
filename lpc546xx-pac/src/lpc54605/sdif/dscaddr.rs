///Register `DSCADDR` reader
pub struct R(crate::R<DSCADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSCADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSCADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSCADDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DSCADDR` writer
pub struct W(crate::W<DSCADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSCADDR_SPEC>;
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
impl From<crate::W<DSCADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSCADDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HDA` reader - Host Descriptor Address Pointer.
pub struct HDA_R(crate::FieldReader<u32, u32>);
impl HDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HDA` writer - Host Descriptor Address Pointer.
pub struct HDA_W<'a> {
    w: &'a mut W,
}
impl<'a> HDA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Host Descriptor Address Pointer.
    #[inline(always)]
    pub fn hda(&self) -> HDA_R {
        HDA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Host Descriptor Address Pointer.
    #[inline(always)]
    pub fn hda(&mut self) -> HDA_W {
        HDA_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Current Host Descriptor Address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dscaddr](index.html) module
pub struct DSCADDR_SPEC;
impl crate::RegisterSpec for DSCADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dscaddr::R](R) reader structure
impl crate::Readable for DSCADDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dscaddr::W](W) writer structure
impl crate::Writable for DSCADDR_SPEC {
    type Writer = W;
}
///`reset()` method sets DSCADDR to value 0
impl crate::Resettable for DSCADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
