///Register `PSTAT` reader
pub struct R(crate::R<PSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSTAT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PSTAT` writer
pub struct W(crate::W<PSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSTAT_SPEC>;
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
impl From<crate::W<PSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSTAT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BUSY` reader - Busy status for this channel pair.
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BUSY` writer - Busy status for this channel pair.
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Field `SLVFRMERR` reader - Save Frame Error flag.
pub struct SLVFRMERR_R(crate::FieldReader<bool, bool>);
impl SLVFRMERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLVFRMERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLVFRMERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SLVFRMERR` writer - Save Frame Error flag.
pub struct SLVFRMERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVFRMERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Field `LR` reader - Left/Right indication.
pub struct LR_R(crate::FieldReader<bool, bool>);
impl LR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LR` writer - Left/Right indication.
pub struct LR_W<'a> {
    w: &'a mut W,
}
impl<'a> LR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Field `DATAPAUSED` reader - Data Paused status flag.
pub struct DATAPAUSED_R(crate::FieldReader<bool, bool>);
impl DATAPAUSED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAPAUSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAPAUSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - Busy status for this channel pair.
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Save Frame Error flag.
    #[inline(always)]
    pub fn slvfrmerr(&self) -> SLVFRMERR_R {
        SLVFRMERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Left/Right indication.
    #[inline(always)]
    pub fn lr(&self) -> LR_R {
        LR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Data Paused status flag.
    #[inline(always)]
    pub fn datapaused(&self) -> DATAPAUSED_R {
        DATAPAUSED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Busy status for this channel pair.
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    ///Bit 1 - Save Frame Error flag.
    #[inline(always)]
    pub fn slvfrmerr(&mut self) -> SLVFRMERR_W {
        SLVFRMERR_W { w: self }
    }
    ///Bit 2 - Left/Right indication.
    #[inline(always)]
    pub fn lr(&mut self) -> LR_W {
        LR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Status register for channel pair
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pstat](index.html) module
pub struct PSTAT_SPEC;
impl crate::RegisterSpec for PSTAT_SPEC {
    type Ux = u32;
}
///`read()` method returns [pstat::R](R) reader structure
impl crate::Readable for PSTAT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pstat::W](W) writer structure
impl crate::Writable for PSTAT_SPEC {
    type Writer = W;
}
///`reset()` method sets PSTAT to value 0
impl crate::Resettable for PSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
