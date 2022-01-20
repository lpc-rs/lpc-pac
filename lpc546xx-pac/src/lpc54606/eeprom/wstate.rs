#[doc = "Register `WSTATE` reader"]
pub struct R(crate::R<WSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WSTATE` writer"]
pub struct W(crate::W<WSTATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WSTATE_SPEC>;
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
impl From<crate::W<WSTATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WSTATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHASE3` reader - Wait states for phase 3 (minus 1 encoded)."]
pub struct PHASE3_R(crate::FieldReader<u8, u8>);
impl PHASE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PHASE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHASE3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHASE3` writer - Wait states for phase 3 (minus 1 encoded)."]
pub struct PHASE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PHASE2` reader - Wait states for phase 2 (minus 1 encoded)."]
pub struct PHASE2_R(crate::FieldReader<u8, u8>);
impl PHASE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PHASE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHASE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHASE2` writer - Wait states for phase 2 (minus 1 encoded)."]
pub struct PHASE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `PHASE1` reader - Wait states for phase 1 (minus 1 encoded)."]
pub struct PHASE1_R(crate::FieldReader<u8, u8>);
impl PHASE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PHASE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHASE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHASE1` writer - Wait states for phase 1 (minus 1 encoded)."]
pub struct PHASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `LCK_PARWEP` reader - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access."]
pub struct LCK_PARWEP_R(crate::FieldReader<bool, bool>);
impl LCK_PARWEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCK_PARWEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK_PARWEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK_PARWEP` writer - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access."]
pub struct LCK_PARWEP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK_PARWEP_W<'a> {
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
    #[doc = "Bits 0:7 - Wait states for phase 3 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase3(&self) -> PHASE3_R {
        PHASE3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Wait states for phase 2 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase2(&self) -> PHASE2_R {
        PHASE2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Wait states for phase 1 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase1(&self) -> PHASE1_R {
        PHASE1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access."]
    #[inline(always)]
    pub fn lck_parwep(&self) -> LCK_PARWEP_R {
        LCK_PARWEP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait states for phase 3 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase3(&mut self) -> PHASE3_W {
        PHASE3_W { w: self }
    }
    #[doc = "Bits 8:15 - Wait states for phase 2 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase2(&mut self) -> PHASE2_W {
        PHASE2_W { w: self }
    }
    #[doc = "Bits 16:23 - Wait states for phase 1 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase1(&mut self) -> PHASE1_W {
        PHASE1_W { w: self }
    }
    #[doc = "Bit 31 - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access."]
    #[inline(always)]
    pub fn lck_parwep(&mut self) -> LCK_PARWEP_W {
        LCK_PARWEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM wait state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wstate](index.html) module"]
pub struct WSTATE_SPEC;
impl crate::RegisterSpec for WSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wstate::R](R) reader structure"]
impl crate::Readable for WSTATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wstate::W](W) writer structure"]
impl crate::Writable for WSTATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WSTATE to value 0x0004_0802"]
impl crate::Resettable for WSTATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0802
    }
}
