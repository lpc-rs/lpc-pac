#[doc = "Register `TXDAT` reader"]
pub struct R(crate::R<TXDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDAT` writer"]
pub struct W(crate::W<TXDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDAT_SPEC>;
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
impl From<crate::W<TXDAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDAT` reader - Writing to the USART Transmit Data Register causes the data to be transmitted as soon as the transmit shift register is available and any conditions for transmitting data are met: CTS low (if CTSEN bit = 1), TXDIS bit = 0."]
pub struct TXDAT_R(crate::FieldReader<u16, u16>);
impl TXDAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TXDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDAT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDAT` writer - Writing to the USART Transmit Data Register causes the data to be transmitted as soon as the transmit shift register is available and any conditions for transmitting data are met: CTS low (if CTSEN bit = 1), TXDIS bit = 0."]
pub struct TXDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Writing to the USART Transmit Data Register causes the data to be transmitted as soon as the transmit shift register is available and any conditions for transmitting data are met: CTS low (if CTSEN bit = 1), TXDIS bit = 0."]
    #[inline(always)]
    pub fn txdat(&self) -> TXDAT_R {
        TXDAT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Writing to the USART Transmit Data Register causes the data to be transmitted as soon as the transmit shift register is available and any conditions for transmitting data are met: CTS low (if CTSEN bit = 1), TXDIS bit = 0."]
    #[inline(always)]
    pub fn txdat(&mut self) -> TXDAT_W {
        TXDAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Data register. Data to be transmitted is written here.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdat](index.html) module"]
pub struct TXDAT_SPEC;
impl crate::RegisterSpec for TXDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdat::R](R) reader structure"]
impl crate::Readable for TXDAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdat::W](W) writer structure"]
impl crate::Writable for TXDAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDAT to value 0"]
impl crate::Resettable for TXDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
