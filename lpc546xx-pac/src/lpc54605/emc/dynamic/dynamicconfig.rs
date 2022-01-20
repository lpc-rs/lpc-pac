#[doc = "Register `DYNAMICCONFIG` reader"]
pub struct R(crate::R<DYNAMICCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICCONFIG` writer"]
pub struct W(crate::W<DYNAMICCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICCONFIG_SPEC>;
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
impl From<crate::W<DYNAMICCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD` reader - Memory device."]
pub struct MD_R(crate::FieldReader<u8, u8>);
impl MD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MD` writer - Memory device."]
pub struct MD_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `AM0` reader - See Table 933."]
pub struct AM0_R(crate::FieldReader<u8, u8>);
impl AM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AM0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AM0` writer - See Table 933."]
pub struct AM0_W<'a> {
    w: &'a mut W,
}
impl<'a> AM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | ((value as u32 & 0x3f) << 7);
        self.w
    }
}
#[doc = "Field `AM1` reader - See Table 933."]
pub struct AM1_R(crate::FieldReader<bool, bool>);
impl AM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AM1` writer - See Table 933."]
pub struct AM1_W<'a> {
    w: &'a mut W,
}
impl<'a> AM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `B` reader - Buffer enable."]
pub struct B_R(crate::FieldReader<bool, bool>);
impl B_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B` writer - Buffer enable."]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `P` reader - Write protect."]
pub struct P_R(crate::FieldReader<bool, bool>);
impl P_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P` writer - Write protect."]
pub struct P_W<'a> {
    w: &'a mut W,
}
impl<'a> P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:4 - Memory device."]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 7:12 - See Table 933."]
    #[inline(always)]
    pub fn am0(&self) -> AM0_R {
        AM0_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - See Table 933."]
    #[inline(always)]
    pub fn am1(&self) -> AM1_R {
        AM1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:4 - Memory device."]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W {
        MD_W { w: self }
    }
    #[doc = "Bits 7:12 - See Table 933."]
    #[inline(always)]
    pub fn am0(&mut self) -> AM0_W {
        AM0_W { w: self }
    }
    #[doc = "Bit 14 - See Table 933."]
    #[inline(always)]
    pub fn am1(&mut self) -> AM1_W {
        AM1_W { w: self }
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&mut self) -> P_W {
        P_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration information for EMC_DYCSx\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicconfig](index.html) module"]
pub struct DYNAMICCONFIG_SPEC;
impl crate::RegisterSpec for DYNAMICCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicconfig::R](R) reader structure"]
impl crate::Readable for DYNAMICCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicconfig::W](W) writer structure"]
impl crate::Writable for DYNAMICCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICCONFIG to value 0"]
impl crate::Resettable for DYNAMICCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
