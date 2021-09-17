#[doc = "Register `SCT_INMUX[%s]` reader"]
pub struct R(crate::R<SCT_INMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCT_INMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCT_INMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCT_INMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCT_INMUX[%s]` writer"]
pub struct W(crate::W<SCT_INMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCT_INMUX_SPEC>;
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
impl From<crate::W<SCT_INMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCT_INMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INP_N` reader - Input mux register for SCT input n (n = 0 to 3). 0 = sct input 0 1=sct input 1 2= sct input 2 3= sct gpio input 3 4= adc_thcmp_irq 5 = comparator out 6 = timer ct32b0 match2 7=gpio_int_bmatch 8=arm_txev 9=debug_halted"]
pub struct INP_N_R(crate::FieldReader<u8, u8>);
impl INP_N_R {
    pub(crate) fn new(bits: u8) -> Self {
        INP_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_N` writer - Input mux register for SCT input n (n = 0 to 3). 0 = sct input 0 1=sct input 1 2= sct input 2 3= sct gpio input 3 4= adc_thcmp_irq 5 = comparator out 6 = timer ct32b0 match2 7=gpio_int_bmatch 8=arm_txev 9=debug_halted"]
pub struct INP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Input mux register for SCT input n (n = 0 to 3). 0 = sct input 0 1=sct input 1 2= sct input 2 3= sct gpio input 3 4= adc_thcmp_irq 5 = comparator out 6 = timer ct32b0 match2 7=gpio_int_bmatch 8=arm_txev 9=debug_halted"]
    #[inline(always)]
    pub fn inp_n(&self) -> INP_N_R {
        INP_N_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input mux register for SCT input n (n = 0 to 3). 0 = sct input 0 1=sct input 1 2= sct input 2 3= sct gpio input 3 4= adc_thcmp_irq 5 = comparator out 6 = timer ct32b0 match2 7=gpio_int_bmatch 8=arm_txev 9=debug_halted"]
    #[inline(always)]
    pub fn inp_n(&mut self) -> INP_N_W {
        INP_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "input select register for SCT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sct_inmux](index.html) module"]
pub struct SCT_INMUX_SPEC;
impl crate::RegisterSpec for SCT_INMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sct_inmux::R](R) reader structure"]
impl crate::Readable for SCT_INMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sct_inmux::W](W) writer structure"]
impl crate::Writable for SCT_INMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCT_INMUX[%s]
to value 0x0f"]
impl crate::Resettable for SCT_INMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
