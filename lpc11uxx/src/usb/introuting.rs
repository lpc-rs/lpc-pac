#[doc = "Register `INTROUTING` reader"]
pub struct R(crate::R<INTROUTING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTROUTING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTROUTING_SPEC>> for R {
    fn from(reader: crate::R<INTROUTING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTROUTING` writer"]
pub struct W(crate::W<INTROUTING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTROUTING_SPEC>;
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
impl core::convert::From<crate::W<INTROUTING_SPEC>> for W {
    fn from(writer: crate::W<INTROUTING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROUTE_INT9_0` reader - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
pub struct ROUTE_INT9_0_R(crate::FieldReader<u16, u16>);
impl ROUTE_INT9_0_R {
    pub(crate) fn new(bits: u16) -> Self {
        ROUTE_INT9_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROUTE_INT9_0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROUTE_INT9_0` writer - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
pub struct ROUTE_INT9_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ROUTE_INT9_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `ROUTE_INT30` reader - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
pub struct ROUTE_INT30_R(crate::FieldReader<bool, bool>);
impl ROUTE_INT30_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROUTE_INT30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROUTE_INT30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROUTE_INT30` writer - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
pub struct ROUTE_INT30_W<'a> {
    w: &'a mut W,
}
impl<'a> ROUTE_INT30_W<'a> {
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
#[doc = "Field `ROUTE_INT31` reader - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
pub struct ROUTE_INT31_R(crate::FieldReader<bool, bool>);
impl ROUTE_INT31_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROUTE_INT31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROUTE_INT31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROUTE_INT31` writer - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
pub struct ROUTE_INT31_W<'a> {
    w: &'a mut W,
}
impl<'a> ROUTE_INT31_W<'a> {
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
    #[doc = "Bits 0:9 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int9_0(&self) -> ROUTE_INT9_0_R {
        ROUTE_INT9_0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int30(&self) -> ROUTE_INT30_R {
        ROUTE_INT30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int31(&self) -> ROUTE_INT31_R {
        ROUTE_INT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int9_0(&mut self) -> ROUTE_INT9_0_W {
        ROUTE_INT9_0_W { w: self }
    }
    #[doc = "Bit 30 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int30(&mut self) -> ROUTE_INT30_W {
        ROUTE_INT30_W { w: self }
    }
    #[doc = "Bit 31 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int31(&mut self) -> ROUTE_INT31_W {
        ROUTE_INT31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt routing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [introuting](index.html) module"]
pub struct INTROUTING_SPEC;
impl crate::RegisterSpec for INTROUTING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [introuting::R](R) reader structure"]
impl crate::Readable for INTROUTING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [introuting::W](W) writer structure"]
impl crate::Writable for INTROUTING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTROUTING to value 0"]
impl crate::Resettable for INTROUTING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
