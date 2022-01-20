#[doc = "Register `W0_0` reader"]
pub struct R(crate::R<W0_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W0_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W0_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W0_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W0_0` writer"]
pub struct W(crate::W<W0_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W0_0_SPEC>;
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
impl From<crate::W<W0_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W0_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWORD` reader - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub struct PWORD_R(crate::FieldReader<u32, u32>);
impl PWORD_R {
    #[inline(always)]
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
#[doc = "Field `PWORD` writer - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub struct PWORD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pword(&self) -> PWORD_R {
        PWORD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pword(&mut self) -> PWORD_W {
        PWORD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_0](index.html) module"]
pub struct W0_0_SPEC;
impl crate::RegisterSpec for W0_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w0_0::R](R) reader structure"]
impl crate::Readable for W0_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w0_0::W](W) writer structure"]
impl crate::Writable for W0_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets W0_0 to value 0"]
impl crate::Resettable for W0_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
