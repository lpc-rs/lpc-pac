#[doc = "Register `CONFLAG` reader"]
pub struct R(crate::R<CONFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFLAG` writer"]
pub struct W(crate::W<CONFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFLAG_SPEC>;
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
impl From<crate::W<CONFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCFLAG` reader - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub struct NCFLAG_R(crate::FieldReader<u16, u16>);
impl NCFLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        NCFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCFLAG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCFLAG` writer - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub struct NCFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> NCFLAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `BUSERRL` reader - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
pub struct BUSERRL_R(crate::FieldReader<bool, bool>);
impl BUSERRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSERRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSERRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSERRL` writer - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
pub struct BUSERRL_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSERRL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `BUSERRH` reader - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
pub struct BUSERRH_R(crate::FieldReader<bool, bool>);
impl BUSERRH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSERRH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSERRH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSERRH` writer - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
pub struct BUSERRH_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSERRH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncflag(&self) -> NCFLAG_R {
        NCFLAG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[inline(always)]
    pub fn buserrl(&self) -> BUSERRL_R {
        BUSERRL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[inline(always)]
    pub fn buserrh(&self) -> BUSERRH_R {
        BUSERRH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncflag(&mut self) -> NCFLAG_W {
        NCFLAG_W { w: self }
    }
    #[doc = "Bit 30 - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[inline(always)]
    pub fn buserrl(&mut self) -> BUSERRL_W {
        BUSERRL_W { w: self }
    }
    #[doc = "Bit 31 - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[inline(always)]
    pub fn buserrh(&mut self) -> BUSERRH_W {
        BUSERRH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT conflict flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conflag](index.html) module"]
pub struct CONFLAG_SPEC;
impl crate::RegisterSpec for CONFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conflag::R](R) reader structure"]
impl crate::Readable for CONFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conflag::W](W) writer structure"]
impl crate::Writable for CONFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFLAG to value 0"]
impl crate::Resettable for CONFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
