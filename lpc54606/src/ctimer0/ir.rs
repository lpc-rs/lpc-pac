#[doc = "Register `IR` reader"]
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IR_SPEC>> for R {
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IR` writer"]
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl core::convert::From<crate::W<IR_SPEC>> for W {
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR0INT` reader - Interrupt flag for match channel 0."]
pub struct MR0INT_R(crate::FieldReader<bool, bool>);
impl MR0INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR0INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR0INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR0INT` writer - Interrupt flag for match channel 0."]
pub struct MR0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0INT_W<'a> {
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
#[doc = "Field `MR1INT` reader - Interrupt flag for match channel 1."]
pub struct MR1INT_R(crate::FieldReader<bool, bool>);
impl MR1INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR1INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR1INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR1INT` writer - Interrupt flag for match channel 1."]
pub struct MR1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1INT_W<'a> {
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
#[doc = "Field `MR2INT` reader - Interrupt flag for match channel 2."]
pub struct MR2INT_R(crate::FieldReader<bool, bool>);
impl MR2INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR2INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR2INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR2INT` writer - Interrupt flag for match channel 2."]
pub struct MR2INT_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2INT_W<'a> {
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
#[doc = "Field `MR3INT` reader - Interrupt flag for match channel 3."]
pub struct MR3INT_R(crate::FieldReader<bool, bool>);
impl MR3INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR3INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR3INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR3INT` writer - Interrupt flag for match channel 3."]
pub struct MR3INT_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3INT_W<'a> {
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
#[doc = "Field `CR0INT` reader - Interrupt flag for capture channel 0 event."]
pub struct CR0INT_R(crate::FieldReader<bool, bool>);
impl CR0INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR0INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR0INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CR0INT` writer - Interrupt flag for capture channel 0 event."]
pub struct CR0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CR1INT` reader - Interrupt flag for capture channel 1 event."]
pub struct CR1INT_R(crate::FieldReader<bool, bool>);
impl CR1INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR1INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR1INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CR1INT` writer - Interrupt flag for capture channel 1 event."]
pub struct CR1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CR2INT` reader - Interrupt flag for capture channel 2 event."]
pub struct CR2INT_R(crate::FieldReader<bool, bool>);
impl CR2INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR2INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR2INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CR2INT` writer - Interrupt flag for capture channel 2 event."]
pub struct CR2INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR2INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CR3INT` reader - Interrupt flag for capture channel 3 event."]
pub struct CR3INT_R(crate::FieldReader<bool, bool>);
impl CR3INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR3INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR3INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CR3INT` writer - Interrupt flag for capture channel 3 event."]
pub struct CR3INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR3INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt flag for match channel 0."]
    #[inline(always)]
    pub fn mr0int(&self) -> MR0INT_R {
        MR0INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for match channel 1."]
    #[inline(always)]
    pub fn mr1int(&self) -> MR1INT_R {
        MR1INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for match channel 2."]
    #[inline(always)]
    pub fn mr2int(&self) -> MR2INT_R {
        MR2INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for match channel 3."]
    #[inline(always)]
    pub fn mr3int(&self) -> MR3INT_R {
        MR3INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    pub fn cr0int(&self) -> CR0INT_R {
        CR0INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    pub fn cr1int(&self) -> CR1INT_R {
        CR1INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt flag for capture channel 2 event."]
    #[inline(always)]
    pub fn cr2int(&self) -> CR2INT_R {
        CR2INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt flag for capture channel 3 event."]
    #[inline(always)]
    pub fn cr3int(&self) -> CR3INT_R {
        CR3INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag for match channel 0."]
    #[inline(always)]
    pub fn mr0int(&mut self) -> MR0INT_W {
        MR0INT_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt flag for match channel 1."]
    #[inline(always)]
    pub fn mr1int(&mut self) -> MR1INT_W {
        MR1INT_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt flag for match channel 2."]
    #[inline(always)]
    pub fn mr2int(&mut self) -> MR2INT_W {
        MR2INT_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt flag for match channel 3."]
    #[inline(always)]
    pub fn mr3int(&mut self) -> MR3INT_W {
        MR3INT_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    pub fn cr0int(&mut self) -> CR0INT_W {
        CR0INT_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    pub fn cr1int(&mut self) -> CR1INT_W {
        CR1INT_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt flag for capture channel 2 event."]
    #[inline(always)]
    pub fn cr2int(&mut self) -> CR2INT_W {
        CR2INT_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt flag for capture channel 3 event."]
    #[inline(always)]
    pub fn cr3int(&mut self) -> CR3INT_W {
        CR3INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](index.html) module"]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir::R](R) reader structure"]
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ir::W](W) writer structure"]
impl crate::Writable for IR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
