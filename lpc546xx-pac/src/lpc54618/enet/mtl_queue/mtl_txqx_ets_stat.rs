///Register `MTL_TXQx_ETS_STAT` reader
pub struct R(crate::R<MTL_TXQX_ETS_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_TXQX_ETS_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_TXQX_ETS_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_TXQX_ETS_STAT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MTL_TXQx_ETS_STAT` writer
pub struct W(crate::W<MTL_TXQX_ETS_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_TXQX_ETS_STAT_SPEC>;
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
impl From<crate::W<MTL_TXQX_ETS_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_TXQX_ETS_STAT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ABS` reader - Average Bits per Slot.
pub struct ABS_R(crate::FieldReader<u32, u32>);
impl ABS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ABS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ABS` writer - Average Bits per Slot.
pub struct ABS_W<'a> {
    w: &'a mut W,
}
impl<'a> ABS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:23 - Average Bits per Slot.
    #[inline(always)]
    pub fn abs(&self) -> ABS_R {
        ABS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:23 - Average Bits per Slot.
    #[inline(always)]
    pub fn abs(&mut self) -> ABS_W {
        ABS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MTL TxQx ETS Status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mtl_txqx_ets_stat](index.html) module
pub struct MTL_TXQX_ETS_STAT_SPEC;
impl crate::RegisterSpec for MTL_TXQX_ETS_STAT_SPEC {
    type Ux = u32;
}
///`read()` method returns [mtl_txqx_ets_stat::R](R) reader structure
impl crate::Readable for MTL_TXQX_ETS_STAT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mtl_txqx_ets_stat::W](W) writer structure
impl crate::Writable for MTL_TXQX_ETS_STAT_SPEC {
    type Writer = W;
}
///`reset()` method sets MTL_TXQx_ETS_STAT to value 0
impl crate::Resettable for MTL_TXQX_ETS_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
