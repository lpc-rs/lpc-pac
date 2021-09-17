#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RITINT` reader - Interrupt flag."]
pub struct RITINT_R(crate::FieldReader<bool, bool>);
impl RITINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RITINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RITINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RITINT` writer - Interrupt flag."]
pub struct RITINT_W<'a> {
    w: &'a mut W,
}
impl<'a> RITINT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RITENCLR` reader - Timer enable clear."]
pub struct RITENCLR_R(crate::FieldReader<bool, bool>);
impl RITENCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RITENCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RITENCLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RITENCLR` writer - Timer enable clear."]
pub struct RITENCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RITENCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RITENBR` reader - Timer enable for debug."]
pub struct RITENBR_R(crate::FieldReader<bool, bool>);
impl RITENBR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RITENBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RITENBR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RITENBR` writer - Timer enable for debug."]
pub struct RITENBR_W<'a> {
    w: &'a mut W,
}
impl<'a> RITENBR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RITEN` reader - Timer enable."]
pub struct RITEN_R(crate::FieldReader<bool, bool>);
impl RITEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RITEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RITEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RITEN` writer - Timer enable."]
pub struct RITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RITEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt flag."]
    #[inline(always)]
    pub fn ritint(&self) -> RITINT_R {
        RITINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer enable clear."]
    #[inline(always)]
    pub fn ritenclr(&self) -> RITENCLR_R {
        RITENCLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer enable for debug."]
    #[inline(always)]
    pub fn ritenbr(&self) -> RITENBR_R {
        RITENBR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline(always)]
    pub fn riten(&self) -> RITEN_R {
        RITEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag."]
    #[inline(always)]
    pub fn ritint(&mut self) -> RITINT_W {
        RITINT_W { w: self }
    }
    #[doc = "Bit 1 - Timer enable clear."]
    #[inline(always)]
    pub fn ritenclr(&mut self) -> RITENCLR_W {
        RITENCLR_W { w: self }
    }
    #[doc = "Bit 2 - Timer enable for debug."]
    #[inline(always)]
    pub fn ritenbr(&mut self) -> RITENBR_W {
        RITENBR_W { w: self }
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline(always)]
    pub fn riten(&mut self) -> RITEN_W {
        RITEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x0c"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
