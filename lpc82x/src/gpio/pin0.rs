#[doc = "Register `PIN0` reader"]
pub struct R(crate::R<PIN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN0` writer"]
pub struct W(crate::W<PIN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN0_SPEC>;
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
impl From<crate::W<PIN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORT` reader - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub struct PORT_R(crate::FieldReader<u32, u32>);
impl PORT_R {
    pub(crate) fn new(bits: u32) -> Self {
        PORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT` writer - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub struct PORT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port pin register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin0](index.html) module"]
pub struct PIN0_SPEC;
impl crate::RegisterSpec for PIN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin0::R](R) reader structure"]
impl crate::Readable for PIN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin0::W](W) writer structure"]
impl crate::Writable for PIN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIN0 to value 0"]
impl crate::Resettable for PIN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
