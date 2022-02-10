///Register `MPIN[%s]` reader
pub struct R(crate::R<MPIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPIN_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MPIN[%s]` writer
pub struct W(crate::W<MPIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPIN_SPEC>;
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
impl From<crate::W<MPIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPIN_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MPORTP` reader - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0.
pub struct MPORTP_R(crate::FieldReader<u32, u32>);
impl MPORTP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MPORTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPORTP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MPORTP` writer - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0.
pub struct MPORTP_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0.
    #[inline(always)]
    pub fn mportp(&self) -> MPORTP_R {
        MPORTP_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0.
    #[inline(always)]
    pub fn mportp(&mut self) -> MPORTP_W {
        MPORTP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Masked port register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mpin](index.html) module
pub struct MPIN_SPEC;
impl crate::RegisterSpec for MPIN_SPEC {
    type Ux = u32;
}
///`read()` method returns [mpin::R](R) reader structure
impl crate::Readable for MPIN_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mpin::W](W) writer structure
impl crate::Writable for MPIN_SPEC {
    type Writer = W;
}
///`reset()` method sets MPIN[%s]
///to value 0
impl crate::Resettable for MPIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
