///Register `MMFAR` reader
pub struct R(crate::R<MMFAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMFAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMFAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMFAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MMFAR` writer
pub struct W(crate::W<MMFAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMFAR_SPEC>;
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
impl From<crate::W<MMFAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMFAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDRESS` reader - Address of MemManage fault location
pub struct ADDRESS_R(crate::FieldReader<u32, u32>);
impl ADDRESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRESS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ADDRESS` writer - Address of MemManage fault location
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Address of MemManage fault location
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Address of MemManage fault location
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MemManage Address Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmfar](index.html) module
pub struct MMFAR_SPEC;
impl crate::RegisterSpec for MMFAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmfar::R](R) reader structure
impl crate::Readable for MMFAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mmfar::W](W) writer structure
impl crate::Writable for MMFAR_SPEC {
    type Writer = W;
}
///`reset()` method sets MMFAR to value 0
impl crate::Resettable for MMFAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
