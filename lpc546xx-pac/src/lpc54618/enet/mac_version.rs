///Register `MAC_VERSION` reader
pub struct R(crate::R<MAC_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAC_VERSION` writer
pub struct W(crate::W<MAC_VERSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_VERSION_SPEC>;
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
impl From<crate::W<MAC_VERSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_VERSION_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SNPVER` reader - NXP defined version.
pub struct SNPVER_R(crate::FieldReader<u8, u8>);
impl SNPVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SNPVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNPVER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SNPVER` writer - NXP defined version.
pub struct SNPVER_W<'a> {
    w: &'a mut W,
}
impl<'a> SNPVER_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///Field `USERVER` reader - User defined version.
pub struct USERVER_R(crate::FieldReader<u8, u8>);
impl USERVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USERVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USERVER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USERVER` writer - User defined version.
pub struct USERVER_W<'a> {
    w: &'a mut W,
}
impl<'a> USERVER_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    ///Bits 0:7 - NXP defined version.
    #[inline(always)]
    pub fn snpver(&self) -> SNPVER_R {
        SNPVER_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - User defined version.
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - NXP defined version.
    #[inline(always)]
    pub fn snpver(&mut self) -> SNPVER_W {
        SNPVER_W { w: self }
    }
    ///Bits 8:15 - User defined version.
    #[inline(always)]
    pub fn userver(&mut self) -> USERVER_W {
        USERVER_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MAC version register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mac_version](index.html) module
pub struct MAC_VERSION_SPEC;
impl crate::RegisterSpec for MAC_VERSION_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_version::R](R) reader structure
impl crate::Readable for MAC_VERSION_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mac_version::W](W) writer structure
impl crate::Writable for MAC_VERSION_SPEC {
    type Writer = W;
}
///`reset()` method sets MAC_VERSION to value 0
impl crate::Resettable for MAC_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
