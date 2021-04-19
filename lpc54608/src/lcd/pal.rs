#[doc = "Register `PAL[%s]` reader"]
pub struct R(crate::R<PAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PAL_SPEC>> for R {
    fn from(reader: crate::R<PAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAL[%s]` writer"]
pub struct W(crate::W<PAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAL_SPEC>;
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
impl core::convert::From<crate::W<PAL_SPEC>> for W {
    fn from(writer: crate::W<PAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R04_0` reader - Red palette data."]
pub struct R04_0_R(crate::FieldReader<u8, u8>);
impl R04_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        R04_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R04_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R04_0` writer - Red palette data."]
pub struct R04_0_W<'a> {
    w: &'a mut W,
}
impl<'a> R04_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `G04_0` reader - Green palette data."]
pub struct G04_0_R(crate::FieldReader<u8, u8>);
impl G04_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        G04_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G04_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G04_0` writer - Green palette data."]
pub struct G04_0_W<'a> {
    w: &'a mut W,
}
impl<'a> G04_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `B04_0` reader - Blue palette data."]
pub struct B04_0_R(crate::FieldReader<u8, u8>);
impl B04_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        B04_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B04_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B04_0` writer - Blue palette data."]
pub struct B04_0_W<'a> {
    w: &'a mut W,
}
impl<'a> B04_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `I0` reader - Intensity / unused bit."]
pub struct I0_R(crate::FieldReader<bool, bool>);
impl I0_R {
    pub(crate) fn new(bits: bool) -> Self {
        I0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I0` writer - Intensity / unused bit."]
pub struct I0_W<'a> {
    w: &'a mut W,
}
impl<'a> I0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `R14_0` reader - Red palette data."]
pub struct R14_0_R(crate::FieldReader<u8, u8>);
impl R14_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        R14_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R14_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R14_0` writer - Red palette data."]
pub struct R14_0_W<'a> {
    w: &'a mut W,
}
impl<'a> R14_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `G14_0` reader - Green palette data."]
pub struct G14_0_R(crate::FieldReader<u8, u8>);
impl G14_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        G14_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G14_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G14_0` writer - Green palette data."]
pub struct G14_0_W<'a> {
    w: &'a mut W,
}
impl<'a> G14_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | ((value as u32 & 0x1f) << 21);
        self.w
    }
}
#[doc = "Field `B14_0` reader - Blue palette data."]
pub struct B14_0_R(crate::FieldReader<u8, u8>);
impl B14_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        B14_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B14_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B14_0` writer - Blue palette data."]
pub struct B14_0_W<'a> {
    w: &'a mut W,
}
impl<'a> B14_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | ((value as u32 & 0x1f) << 26);
        self.w
    }
}
#[doc = "Field `I1` reader - Intensity / unused bit."]
pub struct I1_R(crate::FieldReader<bool, bool>);
impl I1_R {
    pub(crate) fn new(bits: bool) -> Self {
        I1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I1` writer - Intensity / unused bit."]
pub struct I1_W<'a> {
    w: &'a mut W,
}
impl<'a> I1_W<'a> {
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
    #[doc = "Bits 0:4 - Red palette data."]
    #[inline(always)]
    pub fn r04_0(&self) -> R04_0_R {
        R04_0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Green palette data."]
    #[inline(always)]
    pub fn g04_0(&self) -> G04_0_R {
        G04_0_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Blue palette data."]
    #[inline(always)]
    pub fn b04_0(&self) -> B04_0_R {
        B04_0_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i0(&self) -> I0_R {
        I0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Red palette data."]
    #[inline(always)]
    pub fn r14_0(&self) -> R14_0_R {
        R14_0_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Green palette data."]
    #[inline(always)]
    pub fn g14_0(&self) -> G14_0_R {
        G14_0_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - Blue palette data."]
    #[inline(always)]
    pub fn b14_0(&self) -> B14_0_R {
        B14_0_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i1(&self) -> I1_R {
        I1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Red palette data."]
    #[inline(always)]
    pub fn r04_0(&mut self) -> R04_0_W {
        R04_0_W { w: self }
    }
    #[doc = "Bits 5:9 - Green palette data."]
    #[inline(always)]
    pub fn g04_0(&mut self) -> G04_0_W {
        G04_0_W { w: self }
    }
    #[doc = "Bits 10:14 - Blue palette data."]
    #[inline(always)]
    pub fn b04_0(&mut self) -> B04_0_W {
        B04_0_W { w: self }
    }
    #[doc = "Bit 15 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i0(&mut self) -> I0_W {
        I0_W { w: self }
    }
    #[doc = "Bits 16:20 - Red palette data."]
    #[inline(always)]
    pub fn r14_0(&mut self) -> R14_0_W {
        R14_0_W { w: self }
    }
    #[doc = "Bits 21:25 - Green palette data."]
    #[inline(always)]
    pub fn g14_0(&mut self) -> G14_0_W {
        G14_0_W { w: self }
    }
    #[doc = "Bits 26:30 - Blue palette data."]
    #[inline(always)]
    pub fn b14_0(&mut self) -> B14_0_W {
        B14_0_W { w: self }
    }
    #[doc = "Bit 31 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i1(&mut self) -> I1_W {
        I1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "256x16-bit Color Palette registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pal](index.html) module"]
pub struct PAL_SPEC;
impl crate::RegisterSpec for PAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pal::R](R) reader structure"]
impl crate::Readable for PAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pal::W](W) writer structure"]
impl crate::Writable for PAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAL[%s]
to value 0"]
impl crate::Resettable for PAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
