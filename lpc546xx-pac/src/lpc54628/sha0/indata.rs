///Register `INDATA` reader
pub struct R(crate::R<INDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INDATA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `INDATA` writer
pub struct W(crate::W<INDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDATA_SPEC>;
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
impl From<crate::W<INDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDATA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATA` reader - In this field the next word is written in little-endian format.
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DATA` writer - In this field the next word is written in little-endian format.
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - In this field the next word is written in little-endian format.
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - In this field the next word is written in little-endian format.
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Input Data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [indata](index.html) module
pub struct INDATA_SPEC;
impl crate::RegisterSpec for INDATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [indata::R](R) reader structure
impl crate::Readable for INDATA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [indata::W](W) writer structure
impl crate::Writable for INDATA_SPEC {
    type Writer = W;
}
///`reset()` method sets INDATA to value 0
impl crate::Resettable for INDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
