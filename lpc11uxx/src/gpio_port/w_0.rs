#[doc = "Register `W_0%s` reader"]
pub struct R(crate::R<W_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<W_0_SPEC>> for R {
    fn from(reader: crate::R<W_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W_0%s` writer"]
pub struct W(crate::W<W_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W_0_SPEC>;
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
impl core::convert::From<crate::W<W_0_SPEC>> for W {
    fn from(writer: crate::W<W_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWORD` reader - Read 0: pin is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit."]
pub struct PWORD_R(crate::FieldReader<u32, u32>);
impl PWORD_R {
    pub(crate) fn new(bits: u32) -> Self {
        PWORD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWORD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWORD` writer - Read 0: pin is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit."]
pub struct PWORD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Read 0: pin is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit."]
    #[inline(always)]
    pub fn pword(&self) -> PWORD_R {
        PWORD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read 0: pin is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit."]
    #[inline(always)]
    pub fn pword(&mut self) -> PWORD_W {
        PWORD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Word pin registers port 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w_0](index.html) module"]
pub struct W_0_SPEC;
impl crate::RegisterSpec for W_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w_0::R](R) reader structure"]
impl crate::Readable for W_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w_0::W](W) writer structure"]
impl crate::Writable for W_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets W_0%s to value 0"]
impl crate::Resettable for W_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
