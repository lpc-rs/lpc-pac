#[doc = "Register `DLL` reader"]
pub struct R(crate::R<DLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLL` writer"]
pub struct W(crate::W<DLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLL_SPEC>;
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
impl From<crate::W<DLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLLSB` reader - The USART Divisor Latch LSB Register, along with the DLM register, determines the baud rate of the USART."]
pub struct DLLSB_R(crate::FieldReader<u8, u8>);
impl DLLSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLLSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLLSB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLLSB` writer - The USART Divisor Latch LSB Register, along with the DLM register, determines the baud rate of the USART."]
pub struct DLLSB_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The USART Divisor Latch LSB Register, along with the DLM register, determines the baud rate of the USART."]
    #[inline(always)]
    pub fn dllsb(&self) -> DLLSB_R {
        DLLSB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The USART Divisor Latch LSB Register, along with the DLM register, determines the baud rate of the USART."]
    #[inline(always)]
    pub fn dllsb(&mut self) -> DLLSB_W {
        DLLSB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dll](index.html) module"]
pub struct DLL_SPEC;
impl crate::RegisterSpec for DLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dll::R](R) reader structure"]
impl crate::Readable for DLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dll::W](W) writer structure"]
impl crate::Writable for DLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLL to value 0x01"]
impl crate::Resettable for DLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
