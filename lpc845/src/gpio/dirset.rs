#[doc = "Register `DIRSET[%s]` writer"]
pub struct W(crate::W<DIRSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRSET_SPEC>;
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
impl From<crate::W<DIRSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRSETP` writer - Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
pub struct DIRSETP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSETP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:28 - Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
    #[inline(always)]
    pub fn dirsetp(&mut self) -> DIRSETP_W {
        DIRSETP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set pin direction bits for port\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirset](index.html) module"]
pub struct DIRSET_SPEC;
impl crate::RegisterSpec for DIRSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dirset::W](W) writer structure"]
impl crate::Writable for DIRSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIRSET[%s]
to value 0"]
impl crate::Resettable for DIRSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
