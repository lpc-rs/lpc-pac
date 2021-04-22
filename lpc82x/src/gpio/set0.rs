#[doc = "Register `SET0` reader"]
pub struct R(crate::R<SET0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SET0_SPEC>> for R {
    fn from(reader: crate::R<SET0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SET0` writer"]
pub struct W(crate::W<SET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET0_SPEC>;
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
impl core::convert::From<crate::W<SET0_SPEC>> for W {
    fn from(writer: crate::W<SET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETP` reader - Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP_R(crate::FieldReader<u32, u32>);
impl SETP_R {
    pub(crate) fn new(bits: u32) -> Self {
        SETP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP` writer - Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp(&self) -> SETP_R {
        SETP_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp(&mut self) -> SETP_W {
        SETP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write: Set register for port Read: output bits for port\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set0](index.html) module"]
pub struct SET0_SPEC;
impl crate::RegisterSpec for SET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [set0::R](R) reader structure"]
impl crate::Readable for SET0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [set0::W](W) writer structure"]
impl crate::Writable for SET0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SET0 to value 0"]
impl crate::Resettable for SET0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
