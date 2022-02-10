///Register `HCRHDESCRIPTORB` reader
pub struct R(crate::R<HCRHDESCRIPTORB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHDESCRIPTORB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCRHDESCRIPTORB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCRHDESCRIPTORB_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HCRHDESCRIPTORB` writer
pub struct W(crate::W<HCRHDESCRIPTORB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHDESCRIPTORB_SPEC>;
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
impl From<crate::W<HCRHDESCRIPTORB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCRHDESCRIPTORB_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DR` reader - DeviceRemovable Each bit is dedicated to a port of the Root Hub.
pub struct DR_R(crate::FieldReader<u16, u16>);
impl DR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DR` writer - DeviceRemovable Each bit is dedicated to a port of the Root Hub.
pub struct DR_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
///Field `PPCM` reader - PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set.
pub struct PPCM_R(crate::FieldReader<u16, u16>);
impl PPCM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PPCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PPCM` writer - PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set.
pub struct PPCM_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:15 - DeviceRemovable Each bit is dedicated to a port of the Root Hub.
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set.
    #[inline(always)]
    pub fn ppcm(&self) -> PPCM_R {
        PPCM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - DeviceRemovable Each bit is dedicated to a port of the Root Hub.
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W {
        DR_W { w: self }
    }
    ///Bits 16:31 - PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set.
    #[inline(always)]
    pub fn ppcm(&mut self) -> PPCM_W {
        PPCM_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Second of the two registers which describes the characteristics of the Root Hub
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcrhdescriptorb](index.html) module
pub struct HCRHDESCRIPTORB_SPEC;
impl crate::RegisterSpec for HCRHDESCRIPTORB_SPEC {
    type Ux = u32;
}
///`read()` method returns [hcrhdescriptorb::R](R) reader structure
impl crate::Readable for HCRHDESCRIPTORB_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hcrhdescriptorb::W](W) writer structure
impl crate::Writable for HCRHDESCRIPTORB_SPEC {
    type Writer = W;
}
///`reset()` method sets HCRHDESCRIPTORB to value 0
impl crate::Resettable for HCRHDESCRIPTORB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
