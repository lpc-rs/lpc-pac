#[doc = "Register `WINDOW` reader"]
pub struct R(crate::R<WINDOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINDOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINDOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINDOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINDOW` writer"]
pub struct W(crate::W<WINDOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINDOW_SPEC>;
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
impl From<crate::W<WINDOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINDOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINDOW` reader - Watchdog window value."]
pub struct WINDOW_R(crate::FieldReader<u32, u32>);
impl WINDOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WINDOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINDOW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINDOW` writer - Watchdog window value."]
pub struct WINDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> WINDOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Watchdog window value."]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Watchdog window value."]
    #[inline(always)]
    pub fn window(&mut self) -> WINDOW_W {
        WINDOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Window compare value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [window](index.html) module"]
pub struct WINDOW_SPEC;
impl crate::RegisterSpec for WINDOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [window::R](R) reader structure"]
impl crate::Readable for WINDOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [window::W](W) writer structure"]
impl crate::Writable for WINDOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WINDOW to value 0x00ff_ffff"]
impl crate::Resettable for WINDOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_ffff
    }
}
