#[doc = "Register `STARTERSET[%s]` writer"]
pub struct W(crate::W<STARTERSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTERSET_SPEC>;
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
impl From<crate::W<STARTERSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTERSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_SET` writer - Writing ones to this register sets the corresponding bit or bits in the STARTER0 register, if they are implemented."]
pub struct START_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> START_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register sets the corresponding bit or bits in the STARTER0 register, if they are implemented."]
    #[inline(always)]
    pub fn start_set(&mut self) -> START_SET_W {
        START_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set bits in STARTER\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterset](index.html) module"]
pub struct STARTERSET_SPEC;
impl crate::RegisterSpec for STARTERSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [starterset::W](W) writer structure"]
impl crate::Writable for STARTERSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTERSET[%s]
to value 0"]
impl crate::Resettable for STARTERSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
