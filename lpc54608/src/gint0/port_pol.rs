#[doc = "Register `PORT_POL[%s]` reader"]
pub struct R(crate::R<PORT_POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PORT_POL_SPEC>> for R {
    fn from(reader: crate::R<PORT_POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_POL[%s]` writer"]
pub struct W(crate::W<PORT_POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_POL_SPEC>;
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
impl core::convert::From<crate::W<PORT_POL_SPEC>> for W {
    fn from(writer: crate::W<PORT_POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POL` reader - Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_R(crate::FieldReader<u32, u32>);
impl POL_R {
    pub(crate) fn new(bits: u32) -> Self {
        POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL` writer - Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO grouped interrupt port 0 polarity register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_pol](index.html) module"]
pub struct PORT_POL_SPEC;
impl crate::RegisterSpec for PORT_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_pol::R](R) reader structure"]
impl crate::Readable for PORT_POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_pol::W](W) writer structure"]
impl crate::Writable for PORT_POL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORT_POL[%s]
to value 0xffff_ffff"]
impl crate::Resettable for PORT_POL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
