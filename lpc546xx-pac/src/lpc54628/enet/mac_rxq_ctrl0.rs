///Register `MAC_RXQ_CTRL0` reader
pub struct R(crate::R<MAC_RXQ_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_RXQ_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_RXQ_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_RXQ_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAC_RXQ_CTRL0` writer
pub struct W(crate::W<MAC_RXQ_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_RXQ_CTRL0_SPEC>;
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
impl From<crate::W<MAC_RXQ_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_RXQ_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXQ0EN` reader - Receive Queue 0 Enable.
pub struct RXQ0EN_R(crate::FieldReader<u8, u8>);
impl RXQ0EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXQ0EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXQ0EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXQ0EN` writer - Receive Queue 0 Enable.
pub struct RXQ0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXQ0EN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
///Field `RXQ1EN` reader - Receive Queue 1 Enable.
pub struct RXQ1EN_R(crate::FieldReader<u8, u8>);
impl RXQ1EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXQ1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXQ1EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXQ1EN` writer - Receive Queue 1 Enable.
pub struct RXQ1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXQ1EN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
impl R {
    ///Bits 0:1 - Receive Queue 0 Enable.
    #[inline(always)]
    pub fn rxq0en(&self) -> RXQ0EN_R {
        RXQ0EN_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 2:3 - Receive Queue 1 Enable.
    #[inline(always)]
    pub fn rxq1en(&self) -> RXQ1EN_R {
        RXQ1EN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:1 - Receive Queue 0 Enable.
    #[inline(always)]
    pub fn rxq0en(&mut self) -> RXQ0EN_W {
        RXQ0EN_W { w: self }
    }
    ///Bits 2:3 - Receive Queue 1 Enable.
    #[inline(always)]
    pub fn rxq1en(&mut self) -> RXQ1EN_W {
        RXQ1EN_W { w: self }
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
///For information about available fields see [mac_rxq_ctrl0](index.html) module
pub struct MAC_RXQ_CTRL0_SPEC;
impl crate::RegisterSpec for MAC_RXQ_CTRL0_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_rxq_ctrl0::R](R) reader structure
impl crate::Readable for MAC_RXQ_CTRL0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mac_rxq_ctrl0::W](W) writer structure
impl crate::Writable for MAC_RXQ_CTRL0_SPEC {
    type Writer = W;
}
///`reset()` method sets MAC_RXQ_CTRL0 to value 0
impl crate::Resettable for MAC_RXQ_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
