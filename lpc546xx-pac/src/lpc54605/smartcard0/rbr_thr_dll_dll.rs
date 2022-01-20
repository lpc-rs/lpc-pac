#[doc = "Register `DLL` reader"]
pub struct R(crate::R<RBR_THR_DLL_DLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBR_THR_DLL_DLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBR_THR_DLL_DLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBR_THR_DLL_DLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLL` writer"]
pub struct W(crate::W<RBR_THR_DLL_DLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBR_THR_DLL_DLL_SPEC>;
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
impl From<crate::W<RBR_THR_DLL_DLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBR_THR_DLL_DLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLLSB` reader - The SCIn Divisor Latch LSB Register, along with the SCInDLM register, determines the baud rate of the SCIn."]
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
#[doc = "Field `DLLSB` writer - The SCIn Divisor Latch LSB Register, along with the SCInDLM register, determines the baud rate of the SCIn."]
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
    #[doc = "Bits 0:7 - The SCIn Divisor Latch LSB Register, along with the SCInDLM register, determines the baud rate of the SCIn."]
    #[inline(always)]
    pub fn dllsb(&self) -> DLLSB_R {
        DLLSB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The SCIn Divisor Latch LSB Register, along with the SCInDLM register, determines the baud rate of the SCIn."]
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
#[doc = "Divisor Latch LSB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbr_thr_dll_dll](index.html) module"]
pub struct RBR_THR_DLL_DLL_SPEC;
impl crate::RegisterSpec for RBR_THR_DLL_DLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbr_thr_dll_dll::R](R) reader structure"]
impl crate::Readable for RBR_THR_DLL_DLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbr_thr_dll_dll::W](W) writer structure"]
impl crate::Writable for RBR_THR_DLL_DLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLL to value 0x01"]
impl crate::Resettable for RBR_THR_DLL_DLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
