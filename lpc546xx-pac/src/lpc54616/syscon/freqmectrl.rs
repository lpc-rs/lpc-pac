///Register `FREQMECTRL` reader
pub struct R(crate::R<FREQMECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQMECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQMECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQMECTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FREQMECTRL` writer
pub struct W(crate::W<FREQMECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQMECTRL_SPEC>;
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
impl From<crate::W<FREQMECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQMECTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CAPVAL` reader - Stores the capture result which is used to calculate the frequency of the target clock. This field is read-only.
pub struct CAPVAL_R(crate::FieldReader<u16, u16>);
impl CAPVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CAPVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CAPVAL` writer - Stores the capture result which is used to calculate the frequency of the target clock. This field is read-only.
pub struct CAPVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPVAL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
///Field `PROG` reader - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 13:0).
pub struct PROG_R(crate::FieldReader<bool, bool>);
impl PROG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PROG` writer - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 13:0).
pub struct PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:13 - Stores the capture result which is used to calculate the frequency of the target clock. This field is read-only.
    #[inline(always)]
    pub fn capval(&self) -> CAPVAL_R {
        CAPVAL_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 31 - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 13:0).
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:13 - Stores the capture result which is used to calculate the frequency of the target clock. This field is read-only.
    #[inline(always)]
    pub fn capval(&mut self) -> CAPVAL_W {
        CAPVAL_W { w: self }
    }
    ///Bit 31 - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 13:0).
    #[inline(always)]
    pub fn prog(&mut self) -> PROG_W {
        PROG_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Frequency measure register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [freqmectrl](index.html) module
pub struct FREQMECTRL_SPEC;
impl crate::RegisterSpec for FREQMECTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [freqmectrl::R](R) reader structure
impl crate::Readable for FREQMECTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [freqmectrl::W](W) writer structure
impl crate::Writable for FREQMECTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets FREQMECTRL to value 0
impl crate::Resettable for FREQMECTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
