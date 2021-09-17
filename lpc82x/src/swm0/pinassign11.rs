#[doc = "Register `PINASSIGN11` reader"]
pub struct R(crate::R<PINASSIGN11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN11` writer"]
pub struct W(crate::W<PINASSIGN11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN11_SPEC>;
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
impl From<crate::W<PINASSIGN11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_PINTRIG1_I` reader - ADC_PINTRIG1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct ADC_PINTRIG1_I_R(crate::FieldReader<u8, u8>);
impl ADC_PINTRIG1_I_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_PINTRIG1_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_PINTRIG1_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_PINTRIG1_I` writer - ADC_PINTRIG1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct ADC_PINTRIG1_I_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PINTRIG1_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `ACMP_O_O` reader - ACMP_O function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct ACMP_O_O_R(crate::FieldReader<u8, u8>);
impl ACMP_O_O_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACMP_O_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACMP_O_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP_O_O` writer - ACMP_O function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct ACMP_O_O_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_O_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CLKOUT_O` reader - CLKOUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct CLKOUT_O_R(crate::FieldReader<u8, u8>);
impl CLKOUT_O_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKOUT_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKOUT_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOUT_O` writer - CLKOUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct CLKOUT_O_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `GPIO_INT_BMAT_O` reader - GPIO_INT_BMAT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct GPIO_INT_BMAT_O_R(crate::FieldReader<u8, u8>);
impl GPIO_INT_BMAT_O_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_INT_BMAT_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_INT_BMAT_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_INT_BMAT_O` writer - GPIO_INT_BMAT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct GPIO_INT_BMAT_O_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_BMAT_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADC_PINTRIG1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn adc_pintrig1_i(&self) -> ADC_PINTRIG1_I_R {
        ADC_PINTRIG1_I_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ACMP_O function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn acmp_o_o(&self) -> ACMP_O_O_R {
        ACMP_O_O_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CLKOUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn clkout_o(&self) -> CLKOUT_O_R {
        CLKOUT_O_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO_INT_BMAT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn gpio_int_bmat_o(&self) -> GPIO_INT_BMAT_O_R {
        GPIO_INT_BMAT_O_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC_PINTRIG1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn adc_pintrig1_i(&mut self) -> ADC_PINTRIG1_I_W {
        ADC_PINTRIG1_I_W { w: self }
    }
    #[doc = "Bits 8:15 - ACMP_O function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn acmp_o_o(&mut self) -> ACMP_O_O_W {
        ACMP_O_O_W { w: self }
    }
    #[doc = "Bits 16:23 - CLKOUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn clkout_o(&mut self) -> CLKOUT_O_W {
        CLKOUT_O_W { w: self }
    }
    #[doc = "Bits 24:31 - GPIO_INT_BMAT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn gpio_int_bmat_o(&mut self) -> GPIO_INT_BMAT_O_W {
        GPIO_INT_BMAT_O_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 11. Assign movable functions ADC_PINTRIG1, ACMP_O, CLKOUT, GPIO_INT_BMAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign11](index.html) module"]
pub struct PINASSIGN11_SPEC;
impl crate::RegisterSpec for PINASSIGN11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign11::R](R) reader structure"]
impl crate::Readable for PINASSIGN11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign11::W](W) writer structure"]
impl crate::Writable for PINASSIGN11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN11 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
