///Register `MAC_RXQ_CTRL2` reader
pub struct R(crate::R<MAC_RXQ_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_RXQ_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_RXQ_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_RXQ_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAC_RXQ_CTRL2` writer
pub struct W(crate::W<MAC_RXQ_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_RXQ_CTRL2_SPEC>;
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
impl From<crate::W<MAC_RXQ_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_RXQ_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PSRQ0` reader - Priorities Selected in the Receive Queue 0.
pub struct PSRQ0_R(crate::FieldReader<u8, u8>);
impl PSRQ0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSRQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSRQ0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PSRQ0` writer - Priorities Selected in the Receive Queue 0.
pub struct PSRQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRQ0_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///Field `PSRQ1` reader - Priorities Selected in the Receive Queue 1.
pub struct PSRQ1_R(crate::FieldReader<u8, u8>);
impl PSRQ1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSRQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSRQ1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PSRQ1` writer - Priorities Selected in the Receive Queue 1.
pub struct PSRQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRQ1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
///Field `PSRQ2` reader - Priorities Selected in the Receive Queue 2.
pub struct PSRQ2_R(crate::FieldReader<u8, u8>);
impl PSRQ2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSRQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSRQ2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PSRQ2` writer - Priorities Selected in the Receive Queue 2.
pub struct PSRQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRQ2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
///Field `PSRQ3` reader - Priorities Selected in the Receive Queue 3.
pub struct PSRQ3_R(crate::FieldReader<u8, u8>);
impl PSRQ3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSRQ3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSRQ3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PSRQ3` writer - Priorities Selected in the Receive Queue 3.
pub struct PSRQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRQ3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Priorities Selected in the Receive Queue 0.
    #[inline(always)]
    pub fn psrq0(&self) -> PSRQ0_R {
        PSRQ0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Priorities Selected in the Receive Queue 1.
    #[inline(always)]
    pub fn psrq1(&self) -> PSRQ1_R {
        PSRQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Priorities Selected in the Receive Queue 2.
    #[inline(always)]
    pub fn psrq2(&self) -> PSRQ2_R {
        PSRQ2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Priorities Selected in the Receive Queue 3.
    #[inline(always)]
    pub fn psrq3(&self) -> PSRQ3_R {
        PSRQ3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Priorities Selected in the Receive Queue 0.
    #[inline(always)]
    pub fn psrq0(&mut self) -> PSRQ0_W {
        PSRQ0_W { w: self }
    }
    ///Bits 8:15 - Priorities Selected in the Receive Queue 1.
    #[inline(always)]
    pub fn psrq1(&mut self) -> PSRQ1_W {
        PSRQ1_W { w: self }
    }
    ///Bits 16:23 - Priorities Selected in the Receive Queue 2.
    #[inline(always)]
    pub fn psrq2(&mut self) -> PSRQ2_W {
        PSRQ2_W { w: self }
    }
    ///Bits 24:31 - Priorities Selected in the Receive Queue 3.
    #[inline(always)]
    pub fn psrq3(&mut self) -> PSRQ3_W {
        PSRQ3_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Receive Queue Control 0 register 0x0000
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mac_rxq_ctrl2](index.html) module
pub struct MAC_RXQ_CTRL2_SPEC;
impl crate::RegisterSpec for MAC_RXQ_CTRL2_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_rxq_ctrl2::R](R) reader structure
impl crate::Readable for MAC_RXQ_CTRL2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mac_rxq_ctrl2::W](W) writer structure
impl crate::Writable for MAC_RXQ_CTRL2_SPEC {
    type Writer = W;
}
///`reset()` method sets MAC_RXQ_CTRL2 to value 0
impl crate::Resettable for MAC_RXQ_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
