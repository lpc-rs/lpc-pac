#[doc = "Register `PIOPORCAP0` reader"]
pub struct R(crate::R<PIOPORCAP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIOPORCAP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIOPORCAP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIOPORCAP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIOPORCAP0` writer"]
pub struct W(crate::W<PIOPORCAP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIOPORCAP0_SPEC>;
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
impl From<crate::W<PIOPORCAP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIOPORCAP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIOSTAT` reader - State of PIO0_17 through PIO0_0 at power-on reset"]
pub struct PIOSTAT_R(crate::FieldReader<u32, u32>);
impl PIOSTAT_R {
    pub(crate) fn new(bits: u32) -> Self {
        PIOSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIOSTAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - State of PIO0_17 through PIO0_0 at power-on reset"]
    #[inline(always)]
    pub fn piostat(&self) -> PIOSTAT_R {
        PIOSTAT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POR captured PIO status 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pioporcap0](index.html) module"]
pub struct PIOPORCAP0_SPEC;
impl crate::RegisterSpec for PIOPORCAP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pioporcap0::R](R) reader structure"]
impl crate::Readable for PIOPORCAP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pioporcap0::W](W) writer structure"]
impl crate::Writable for PIOPORCAP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIOPORCAP0 to value 0"]
impl crate::Resettable for PIOPORCAP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
