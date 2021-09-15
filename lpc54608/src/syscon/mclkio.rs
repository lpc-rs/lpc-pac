#[doc = "Register `MCLKIO` reader"]
pub struct R(crate::R<MCLKIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKIO` writer"]
pub struct W(crate::W<MCLKIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKIO_SPEC>;
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
impl From<crate::W<MCLKIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - MCLK direction control."]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - MCLK direction control."]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MCLK direction control."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCLK direction control."]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK input/output control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkio](index.html) module"]
pub struct MCLKIO_SPEC;
impl crate::RegisterSpec for MCLKIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mclkio::R](R) reader structure"]
impl crate::Readable for MCLKIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkio::W](W) writer structure"]
impl crate::Writable for MCLKIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCLKIO to value 0"]
impl crate::Resettable for MCLKIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
