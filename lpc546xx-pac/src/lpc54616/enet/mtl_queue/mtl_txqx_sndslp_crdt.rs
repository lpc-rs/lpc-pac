///Register `MTL_TXQx_SNDSLP_CRDT` reader
pub struct R(crate::R<MTL_TXQX_SNDSLP_CRDT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_TXQX_SNDSLP_CRDT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_TXQX_SNDSLP_CRDT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_TXQX_SNDSLP_CRDT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MTL_TXQx_SNDSLP_CRDT` writer
pub struct W(crate::W<MTL_TXQX_SNDSLP_CRDT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_TXQX_SNDSLP_CRDT_SPEC>;
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
impl From<crate::W<MTL_TXQX_SNDSLP_CRDT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_TXQX_SNDSLP_CRDT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SSC` reader - sendSlopeCredit.
pub struct SSC_R(crate::FieldReader<u16, u16>);
impl SSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SSC` writer - sendSlopeCredit.
pub struct SSC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    ///Bits 0:13 - sendSlopeCredit.
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 0:13 - sendSlopeCredit.
    #[inline(always)]
    pub fn ssc(&mut self) -> SSC_W {
        SSC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MTL TxQx SendSlopCredit register, only TxQ1 support
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mtl_txqx_sndslp_crdt](index.html) module
pub struct MTL_TXQX_SNDSLP_CRDT_SPEC;
impl crate::RegisterSpec for MTL_TXQX_SNDSLP_CRDT_SPEC {
    type Ux = u32;
}
///`read()` method returns [mtl_txqx_sndslp_crdt::R](R) reader structure
impl crate::Readable for MTL_TXQX_SNDSLP_CRDT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mtl_txqx_sndslp_crdt::W](W) writer structure
impl crate::Writable for MTL_TXQX_SNDSLP_CRDT_SPEC {
    type Writer = W;
}
///`reset()` method sets MTL_TXQx_SNDSLP_CRDT to value 0
impl crate::Resettable for MTL_TXQX_SNDSLP_CRDT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
