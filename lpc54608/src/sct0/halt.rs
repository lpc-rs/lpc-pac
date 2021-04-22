#[doc = "Register `HALT` reader"]
pub struct R(crate::R<HALT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HALT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HALT_SPEC>> for R {
    fn from(reader: crate::R<HALT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HALT` writer"]
pub struct W(crate::W<HALT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HALT_SPEC>;
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
impl core::convert::From<crate::W<HALT_SPEC>> for W {
    fn from(writer: crate::W<HALT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALTMSK_L` reader - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub struct HALTMSK_L_R(crate::FieldReader<u16, u16>);
impl HALTMSK_L_R {
    pub(crate) fn new(bits: u16) -> Self {
        HALTMSK_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALTMSK_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALTMSK_L` writer - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub struct HALTMSK_L_W<'a> {
    w: &'a mut W,
}
impl<'a> HALTMSK_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `HALTMSK_H` reader - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub struct HALTMSK_H_R(crate::FieldReader<u16, u16>);
impl HALTMSK_H_R {
    pub(crate) fn new(bits: u16) -> Self {
        HALTMSK_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALTMSK_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALTMSK_H` writer - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub struct HALTMSK_H_W<'a> {
    w: &'a mut W,
}
impl<'a> HALTMSK_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_l(&self) -> HALTMSK_L_R {
        HALTMSK_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_h(&self) -> HALTMSK_H_R {
        HALTMSK_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_l(&mut self) -> HALTMSK_L_W {
        HALTMSK_L_W { w: self }
    }
    #[doc = "Bits 16:31 - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_h(&mut self) -> HALTMSK_H_W {
        HALTMSK_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT halt event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [halt](index.html) module"]
pub struct HALT_SPEC;
impl crate::RegisterSpec for HALT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [halt::R](R) reader structure"]
impl crate::Readable for HALT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [halt::W](W) writer structure"]
impl crate::Writable for HALT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HALT to value 0"]
impl crate::Resettable for HALT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
