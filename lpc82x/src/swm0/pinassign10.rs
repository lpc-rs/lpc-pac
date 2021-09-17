#[doc = "Register `PINASSIGN10` reader"]
pub struct R(crate::R<PINASSIGN10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN10` writer"]
pub struct W(crate::W<PINASSIGN10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN10_SPEC>;
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
impl From<crate::W<PINASSIGN10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C2_SCL_IO` reader - I2C1_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct I2C2_SCL_IO_R(crate::FieldReader<u8, u8>);
impl I2C2_SCL_IO_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C2_SCL_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2_SCL_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2_SCL_IO` writer - I2C1_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct I2C2_SCL_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_SCL_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `I2C3_SDA_IO` reader - I2C3_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct I2C3_SDA_IO_R(crate::FieldReader<u8, u8>);
impl I2C3_SDA_IO_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C3_SDA_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3_SDA_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3_SDA_IO` writer - I2C3_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct I2C3_SDA_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_SDA_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `I2C3_SCL_IO` reader - I2C3_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct I2C3_SCL_IO_R(crate::FieldReader<u8, u8>);
impl I2C3_SCL_IO_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C3_SCL_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3_SCL_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3_SCL_IO` writer - I2C3_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct I2C3_SCL_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_SCL_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ADC_PINTRIG0_I` reader - ADC_PINTRIG0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct ADC_PINTRIG0_I_R(crate::FieldReader<u8, u8>);
impl ADC_PINTRIG0_I_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_PINTRIG0_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_PINTRIG0_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_PINTRIG0_I` writer - ADC_PINTRIG0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct ADC_PINTRIG0_I_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PINTRIG0_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C1_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn i2c2_scl_io(&self) -> I2C2_SCL_IO_R {
        I2C2_SCL_IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2C3_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn i2c3_sda_io(&self) -> I2C3_SDA_IO_R {
        I2C3_SDA_IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I2C3_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn i2c3_scl_io(&self) -> I2C3_SCL_IO_R {
        I2C3_SCL_IO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ADC_PINTRIG0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn adc_pintrig0_i(&self) -> ADC_PINTRIG0_I_R {
        ADC_PINTRIG0_I_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C1_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn i2c2_scl_io(&mut self) -> I2C2_SCL_IO_W {
        I2C2_SCL_IO_W { w: self }
    }
    #[doc = "Bits 8:15 - I2C3_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn i2c3_sda_io(&mut self) -> I2C3_SDA_IO_W {
        I2C3_SDA_IO_W { w: self }
    }
    #[doc = "Bits 16:23 - I2C3_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn i2c3_scl_io(&mut self) -> I2C3_SCL_IO_W {
        I2C3_SCL_IO_W { w: self }
    }
    #[doc = "Bits 24:31 - ADC_PINTRIG0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn adc_pintrig0_i(&mut self) -> ADC_PINTRIG0_I_W {
        ADC_PINTRIG0_I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 10. Assign movable functions, I2C2_SCL, I2C3_SDA, I2C3_SCL, ADC_PINTRIG0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign10](index.html) module"]
pub struct PINASSIGN10_SPEC;
impl crate::RegisterSpec for PINASSIGN10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign10::R](R) reader structure"]
impl crate::Readable for PINASSIGN10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign10::W](W) writer structure"]
impl crate::Writable for PINASSIGN10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN10 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
