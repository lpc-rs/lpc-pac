#[doc = "Register `CLKDIV` reader"]
pub struct R(crate::R<CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLKDIV_SPEC>> for R {
    fn from(reader: crate::R<CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV` writer"]
pub struct W(crate::W<CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV_SPEC>;
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
impl core::convert::From<crate::W<CLKDIV_SPEC>> for W {
    fn from(writer: crate::W<CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVVAL` reader - This field controls how the Flexcomm clock (FCLK) is used by the I2C functions that need an internal clock in order to operate. 0x0000 = FCLK is used directly by the I2C. 0x0001 = FCLK is divided by 2 before use. 0x0002 = FCLK is divided by 3 before use. 0xFFFF = FCLK is divided by 65,536 before use."]
pub struct DIVVAL_R(crate::FieldReader<u16, u16>);
impl DIVVAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIVVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVVAL` writer - This field controls how the Flexcomm clock (FCLK) is used by the I2C functions that need an internal clock in order to operate. 0x0000 = FCLK is used directly by the I2C. 0x0001 = FCLK is divided by 2 before use. 0x0002 = FCLK is divided by 3 before use. 0xFFFF = FCLK is divided by 65,536 before use."]
pub struct DIVVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field controls how the Flexcomm clock (FCLK) is used by the I2C functions that need an internal clock in order to operate. 0x0000 = FCLK is used directly by the I2C. 0x0001 = FCLK is divided by 2 before use. 0x0002 = FCLK is divided by 3 before use. 0xFFFF = FCLK is divided by 65,536 before use."]
    #[inline(always)]
    pub fn divval(&self) -> DIVVAL_R {
        DIVVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field controls how the Flexcomm clock (FCLK) is used by the I2C functions that need an internal clock in order to operate. 0x0000 = FCLK is used directly by the I2C. 0x0001 = FCLK is divided by 2 before use. 0x0002 = FCLK is divided by 3 before use. 0xFFFF = FCLK is divided by 65,536 before use."]
    #[inline(always)]
    pub fn divval(&mut self) -> DIVVAL_W {
        DIVVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv](index.html) module"]
pub struct CLKDIV_SPEC;
impl crate::RegisterSpec for CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv::R](R) reader structure"]
impl crate::Readable for CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](W) writer structure"]
impl crate::Writable for CLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for CLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
