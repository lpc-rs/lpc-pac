///Register `MTL_TXQx_HI_CRDT` reader
pub struct R(crate::R<MTL_TXQX_HI_CRDT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_TXQX_HI_CRDT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_TXQX_HI_CRDT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_TXQX_HI_CRDT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MTL_TXQx_HI_CRDT` writer
pub struct W(crate::W<MTL_TXQX_HI_CRDT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_TXQX_HI_CRDT_SPEC>;
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
impl From<crate::W<MTL_TXQX_HI_CRDT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_TXQX_HI_CRDT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HC` reader - hiCredit.
pub struct HC_R(crate::FieldReader<u32, u32>);
impl HC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HC` writer - hiCredit.
pub struct HC_W<'a> {
    w: &'a mut W,
}
impl<'a> HC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:28 - hiCredit.
    #[inline(always)]
    pub fn hc(&self) -> HC_R {
        HC_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:28 - hiCredit.
    #[inline(always)]
    pub fn hc(&mut self) -> HC_W {
        HC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MTL TxQx hiCredit register, only TxQ1 support
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mtl_txqx_hi_crdt](index.html) module
pub struct MTL_TXQX_HI_CRDT_SPEC;
impl crate::RegisterSpec for MTL_TXQX_HI_CRDT_SPEC {
    type Ux = u32;
}
///`read()` method returns [mtl_txqx_hi_crdt::R](R) reader structure
impl crate::Readable for MTL_TXQX_HI_CRDT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mtl_txqx_hi_crdt::W](W) writer structure
impl crate::Writable for MTL_TXQX_HI_CRDT_SPEC {
    type Writer = W;
}
///`reset()` method sets MTL_TXQx_HI_CRDT to value 0
impl crate::Resettable for MTL_TXQX_HI_CRDT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
