///Register `START` reader
pub struct R(crate::R<START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<START_SPEC>) -> Self {
        R(reader)
    }
}
///Register `START` writer
pub struct W(crate::W<START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<START_SPEC>;
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
impl From<crate::W<START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<START_SPEC>) -> Self {
        W(writer)
    }
}
///Field `STARTMSK_L` reader - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT.
pub struct STARTMSK_L_R(crate::FieldReader<u16, u16>);
impl STARTMSK_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        STARTMSK_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTMSK_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `STARTMSK_L` writer - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT.
pub struct STARTMSK_L_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTMSK_L_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
///Field `STARTMSK_H` reader - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT.
pub struct STARTMSK_H_R(crate::FieldReader<u16, u16>);
impl STARTMSK_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        STARTMSK_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTMSK_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `STARTMSK_H` writer - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT.
pub struct STARTMSK_H_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTMSK_H_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:15 - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT.
    #[inline(always)]
    pub fn startmsk_l(&self) -> STARTMSK_L_R {
        STARTMSK_L_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT.
    #[inline(always)]
    pub fn startmsk_h(&self) -> STARTMSK_H_R {
        STARTMSK_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT.
    #[inline(always)]
    pub fn startmsk_l(&mut self) -> STARTMSK_L_W {
        STARTMSK_L_W { w: self }
    }
    ///Bits 16:31 - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT.
    #[inline(always)]
    pub fn startmsk_h(&mut self) -> STARTMSK_H_W {
        STARTMSK_H_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SCT start event select register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [start](index.html) module
pub struct START_SPEC;
impl crate::RegisterSpec for START_SPEC {
    type Ux = u32;
}
///`read()` method returns [start::R](R) reader structure
impl crate::Readable for START_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [start::W](W) writer structure
impl crate::Writable for START_SPEC {
    type Writer = W;
}
///`reset()` method sets START to value 0
impl crate::Resettable for START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
