#[doc = "Register `STATE` reader"]
pub struct R(crate::R<STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STATE_SPEC>> for R {
    fn from(reader: crate::R<STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATE` writer"]
pub struct W(crate::W<STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATE_SPEC>;
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
impl core::convert::From<crate::W<STATE_SPEC>> for W {
    fn from(writer: crate::W<STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATEMSKn` reader - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
pub struct STATEMSKN_R(crate::FieldReader<u8, u8>);
impl STATEMSKN_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATEMSKN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATEMSKN_R {
    type Target = crate::FieldReader<u8, u8>;
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
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[inline(always)]
    pub fn statemskn(&self) -> STATEMSKN_R {
        STATEMSKN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[inline(always)]
    pub fn statemskn(&mut self) -> STATEMSKN_W {
        STATEMSKN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT event state register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](index.html) module"]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state::R](R) reader structure"]
impl crate::Readable for STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [state::W](W) writer structure"]
impl crate::Writable for STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
