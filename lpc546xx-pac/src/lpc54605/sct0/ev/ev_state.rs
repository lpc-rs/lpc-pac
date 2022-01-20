#[doc = "Register `EV_STATE` reader"]
pub struct R(crate::R<EV_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EV_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EV_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EV_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EV_STATE` writer"]
pub struct W(crate::W<EV_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EV_STATE_SPEC>;
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
impl From<crate::W<EV_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EV_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATEMSKn` reader - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
pub struct STATEMSKN_R(crate::FieldReader<u16, u16>);
impl STATEMSKN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        STATEMSKN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATEMSKN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATEMSKn` writer - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
pub struct STATEMSKN_W<'a> {
    w: &'a mut W,
}
impl<'a> STATEMSKN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[inline(always)]
    pub fn statemskn(&self) -> STATEMSKN_R {
        STATEMSKN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[inline(always)]
    pub fn statemskn(&mut self) -> STATEMSKN_W {
        STATEMSKN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT event state register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_state](index.html) module"]
pub struct EV_STATE_SPEC;
impl crate::RegisterSpec for EV_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ev_state::R](R) reader structure"]
impl crate::Readable for EV_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ev_state::W](W) writer structure"]
impl crate::Writable for EV_STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EV_STATE to value 0"]
impl crate::Resettable for EV_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
