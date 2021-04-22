#[doc = "Register `FREQMEAS_REF` reader"]
pub struct R(crate::R<FREQMEAS_REF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQMEAS_REF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FREQMEAS_REF_SPEC>> for R {
    fn from(reader: crate::R<FREQMEAS_REF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQMEAS_REF` writer"]
pub struct W(crate::W<FREQMEAS_REF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQMEAS_REF_SPEC>;
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
impl core::convert::From<crate::W<FREQMEAS_REF_SPEC>> for W {
    fn from(writer: crate::W<FREQMEAS_REF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKIN` reader - Clock source number (decimal value) for frequency measure function target clock: 0 = CLK_IN 1 = FRO 12 MHz oscillator 2 = Watchdog oscillator 3 = 32 kHz RTC oscillator 4 = Main clock (see Section 4.5.23) 5 = PIO0_4 6 = PIO0_20 7 = PIO0_24 8 = PIO1_4"]
pub struct CLKIN_R(crate::FieldReader<u8, u8>);
impl CLKIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKIN` writer - Clock source number (decimal value) for frequency measure function target clock: 0 = CLK_IN 1 = FRO 12 MHz oscillator 2 = Watchdog oscillator 3 = 32 kHz RTC oscillator 4 = Main clock (see Section 4.5.23) 5 = PIO0_4 6 = PIO0_20 7 = PIO0_24 8 = PIO1_4"]
pub struct CLKIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Clock source number (decimal value) for frequency measure function target clock: 0 = CLK_IN 1 = FRO 12 MHz oscillator 2 = Watchdog oscillator 3 = 32 kHz RTC oscillator 4 = Main clock (see Section 4.5.23) 5 = PIO0_4 6 = PIO0_20 7 = PIO0_24 8 = PIO1_4"]
    #[inline(always)]
    pub fn clkin(&self) -> CLKIN_R {
        CLKIN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clock source number (decimal value) for frequency measure function target clock: 0 = CLK_IN 1 = FRO 12 MHz oscillator 2 = Watchdog oscillator 3 = 32 kHz RTC oscillator 4 = Main clock (see Section 4.5.23) 5 = PIO0_4 6 = PIO0_20 7 = PIO0_24 8 = PIO1_4"]
    #[inline(always)]
    pub fn clkin(&mut self) -> CLKIN_W {
        CLKIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selection for frequency measurement reference clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqmeas_ref](index.html) module"]
pub struct FREQMEAS_REF_SPEC;
impl crate::RegisterSpec for FREQMEAS_REF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freqmeas_ref::R](R) reader structure"]
impl crate::Readable for FREQMEAS_REF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freqmeas_ref::W](W) writer structure"]
impl crate::Writable for FREQMEAS_REF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQMEAS_REF to value 0x1f"]
impl crate::Resettable for FREQMEAS_REF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
