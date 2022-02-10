///Register `DIRCLR[%s]` writer
pub struct W(crate::W<DIRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRCLR_SPEC>;
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
impl From<crate::W<DIRCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRCLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIRCLRP` writer - Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit.
pub struct DIRCLRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLRP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
impl W {
    ///Bits 0:28 - Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit.
    #[inline(always)]
    pub fn dirclrp(&mut self) -> DIRCLRP_W {
        DIRCLRP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clear pin direction bits for port
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dirclr](index.html) module
pub struct DIRCLR_SPEC;
impl crate::RegisterSpec for DIRCLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [dirclr::W](W) writer structure
impl crate::Writable for DIRCLR_SPEC {
    type Writer = W;
}
///`reset()` method sets DIRCLR[%s]
///to value 0
impl crate::Resettable for DIRCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
