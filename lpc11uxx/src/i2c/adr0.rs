#[doc = "Register `ADR0` reader"]
pub struct R(crate::R<ADR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADR0` writer"]
pub struct W(crate::W<ADR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADR0_SPEC>;
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
impl From<crate::W<ADR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GC` reader - General Call enable bit."]
pub struct GC_R(crate::FieldReader<bool, bool>);
impl GC_R {
    pub(crate) fn new(bits: bool) -> Self {
        GC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GC` writer - General Call enable bit."]
pub struct GC_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_W<'a> {
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
#[doc = "Field `Address` reader - The I2C device address for slave mode."]
pub struct ADDRESS_R(crate::FieldReader<u8, u8>);
impl ADDRESS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRESS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Address` writer - The I2C device address for slave mode."]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - General Call enable bit."]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - The I2C device address for slave mode."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - General Call enable bit."]
    #[inline(always)]
    pub fn gc(&mut self) -> GC_W {
        GC_W { w: self }
    }
    #[doc = "Bits 1:7 - The I2C device address for slave mode."]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Address Register 0. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adr0](index.html) module"]
pub struct ADR0_SPEC;
impl crate::RegisterSpec for ADR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adr0::R](R) reader structure"]
impl crate::Readable for ADR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adr0::W](W) writer structure"]
impl crate::Writable for ADR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADR0 to value 0"]
impl crate::Resettable for ADR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
