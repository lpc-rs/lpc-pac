#[doc = "Register `PINASSIGN7` reader"]
pub struct R(crate::R<PINASSIGN7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PINASSIGN7_SPEC>> for R {
    fn from(reader: crate::R<PINASSIGN7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN7` writer"]
pub struct W(crate::W<PINASSIGN7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN7_SPEC>;
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
impl core::convert::From<crate::W<PINASSIGN7_SPEC>> for W {
    fn from(writer: crate::W<PINASSIGN7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCT0_GPIO_IN_B_I` reader - SCT0_GPIO_IN_B function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SCT0_GPIO_IN_B_I_R(crate::FieldReader<u8, u8>);
impl SCT0_GPIO_IN_B_I_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCT0_GPIO_IN_B_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT0_GPIO_IN_B_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT0_GPIO_IN_B_I` writer - SCT0_GPIO_IN_B function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SCT0_GPIO_IN_B_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_GPIO_IN_B_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SCT0_GPIO_IN_C_I` reader - SCT0_GPIO_IN_C function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SCT0_GPIO_IN_C_I_R(crate::FieldReader<u8, u8>);
impl SCT0_GPIO_IN_C_I_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCT0_GPIO_IN_C_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT0_GPIO_IN_C_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT0_GPIO_IN_C_I` writer - SCT0_GPIO_IN_C function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SCT0_GPIO_IN_C_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_GPIO_IN_C_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SCT0_GPIO_IN_D_I` reader - SCT0_GPIO_IN_D function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SCT0_GPIO_IN_D_I_R(crate::FieldReader<u8, u8>);
impl SCT0_GPIO_IN_D_I_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCT0_GPIO_IN_D_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT0_GPIO_IN_D_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT0_GPIO_IN_D_I` writer - SCT0_GPIO_IN_D function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SCT0_GPIO_IN_D_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_GPIO_IN_D_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SCT_OUT0_O` reader - SCT_OUT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SCT_OUT0_O_R(crate::FieldReader<u8, u8>);
impl SCT_OUT0_O_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCT_OUT0_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT_OUT0_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT_OUT0_O` writer - SCT_OUT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SCT_OUT0_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT0_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SCT0_GPIO_IN_B function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_b_i(&self) -> SCT0_GPIO_IN_B_I_R {
        SCT0_GPIO_IN_B_I_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCT0_GPIO_IN_C function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_c_i(&self) -> SCT0_GPIO_IN_C_I_R {
        SCT0_GPIO_IN_C_I_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCT0_GPIO_IN_D function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_d_i(&self) -> SCT0_GPIO_IN_D_I_R {
        SCT0_GPIO_IN_D_I_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCT_OUT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out0_o(&self) -> SCT_OUT0_O_R {
        SCT_OUT0_O_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCT0_GPIO_IN_B function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_b_i(&mut self) -> SCT0_GPIO_IN_B_I_W {
        SCT0_GPIO_IN_B_I_W { w: self }
    }
    #[doc = "Bits 8:15 - SCT0_GPIO_IN_C function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_c_i(&mut self) -> SCT0_GPIO_IN_C_I_W {
        SCT0_GPIO_IN_C_I_W { w: self }
    }
    #[doc = "Bits 16:23 - SCT0_GPIO_IN_D function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_d_i(&mut self) -> SCT0_GPIO_IN_D_I_W {
        SCT0_GPIO_IN_D_I_W { w: self }
    }
    #[doc = "Bits 24:31 - SCT_OUT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out0_o(&mut self) -> SCT_OUT0_O_W {
        SCT_OUT0_O_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 7. Assign movable functions SCT_IN1, SCT_IN2, SCT_IN3, SCT_OUT0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign7](index.html) module"]
pub struct PINASSIGN7_SPEC;
impl crate::RegisterSpec for PINASSIGN7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign7::R](R) reader structure"]
impl crate::Readable for PINASSIGN7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign7::W](W) writer structure"]
impl crate::Writable for PINASSIGN7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN7 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
