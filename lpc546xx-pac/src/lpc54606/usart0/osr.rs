///Register `OSR` reader
pub struct R(crate::R<OSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OSR` writer
pub struct W(crate::W<OSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSR_SPEC>;
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
impl From<crate::W<OSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OSRVAL` reader - Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit.
pub struct OSRVAL_R(crate::FieldReader<u8, u8>);
impl OSRVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSRVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSRVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OSRVAL` writer - Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit.
pub struct OSRVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSRVAL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit.
    #[inline(always)]
    pub fn osrval(&self) -> OSRVAL_R {
        OSRVAL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit.
    #[inline(always)]
    pub fn osrval(&mut self) -> OSRVAL_W {
        OSRVAL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Oversample selection register for asynchronous communication.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [osr](index.html) module
pub struct OSR_SPEC;
impl crate::RegisterSpec for OSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [osr::R](R) reader structure
impl crate::Readable for OSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [osr::W](W) writer structure
impl crate::Writable for OSR_SPEC {
    type Writer = W;
}
///`reset()` method sets OSR to value 0x0f
impl crate::Resettable for OSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
