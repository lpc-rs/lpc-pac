///Register `PORT_ENA[%s]` reader
pub struct R(crate::R<PORT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PORT_ENA[%s]` writer
pub struct W(crate::W<PORT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_ENA_SPEC>;
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
impl From<crate::W<PORT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ENA` reader - Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt.
pub struct ENA_R(crate::FieldReader<u32, u32>);
impl ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ENA` writer - Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt.
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt.
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt.
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO grouped interrupt port 0 enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [port_ena](index.html) module
pub struct PORT_ENA_SPEC;
impl crate::RegisterSpec for PORT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [port_ena::R](R) reader structure
impl crate::Readable for PORT_ENA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [port_ena::W](W) writer structure
impl crate::Writable for PORT_ENA_SPEC {
    type Writer = W;
}
///`reset()` method sets PORT_ENA[%s]
///to value 0
impl crate::Resettable for PORT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
