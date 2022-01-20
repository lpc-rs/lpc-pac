#[doc = "Register `B0_26` reader"]
pub struct R(crate::R<B0_26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B0_26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B0_26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B0_26_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `B0_26` writer"]
pub struct W(crate::W<B0_26_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<B0_26_SPEC>;
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
impl From<crate::W<B0_26_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<B0_26_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBYTE` reader - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub struct PBYTE_R(crate::FieldReader<bool, bool>);
impl PBYTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBYTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBYTE` writer - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub struct PBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PBYTE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pbyte(&self) -> PBYTE_R {
        PBYTE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pbyte(&mut self) -> PBYTE_W {
        PBYTE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_26](index.html) module"]
pub struct B0_26_SPEC;
impl crate::RegisterSpec for B0_26_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [b0_26::R](R) reader structure"]
impl crate::Readable for B0_26_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [b0_26::W](W) writer structure"]
impl crate::Writable for B0_26_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets B0_26 to value 0"]
impl crate::Resettable for B0_26_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
