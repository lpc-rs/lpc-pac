#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTSTAT_SPEC>> for R {
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTAT` writer"]
pub struct W(crate::W<INTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTAT_SPEC>;
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
impl core::convert::From<crate::W<INTSTAT_SPEC>> for W {
    fn from(writer: crate::W<INTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP0OUT` reader - Interrupt status register bit for the Control EP0 OUT direction."]
pub struct EP0OUT_R(crate::FieldReader<bool, bool>);
impl EP0OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP0OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0OUT` writer - Interrupt status register bit for the Control EP0 OUT direction."]
pub struct EP0OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0OUT_W<'a> {
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
#[doc = "Field `EP0IN` reader - Interrupt status register bit for the Control EP0 IN direction."]
pub struct EP0IN_R(crate::FieldReader<bool, bool>);
impl EP0IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP0IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0IN` writer - Interrupt status register bit for the Control EP0 IN direction."]
pub struct EP0IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0IN_W<'a> {
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
#[doc = "Field `EP1OUT` reader - Interrupt status register bit for the EP1 OUT direction."]
pub struct EP1OUT_R(crate::FieldReader<bool, bool>);
impl EP1OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP1OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1OUT` writer - Interrupt status register bit for the EP1 OUT direction."]
pub struct EP1OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1OUT_W<'a> {
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
#[doc = "Field `EP1IN` reader - Interrupt status register bit for the EP1 IN direction."]
pub struct EP1IN_R(crate::FieldReader<bool, bool>);
impl EP1IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP1IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1IN` writer - Interrupt status register bit for the EP1 IN direction."]
pub struct EP1IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1IN_W<'a> {
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
#[doc = "Field `EP2OUT` reader - Interrupt status register bit for the EP2 OUT direction."]
pub struct EP2OUT_R(crate::FieldReader<bool, bool>);
impl EP2OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP2OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2OUT` writer - Interrupt status register bit for the EP2 OUT direction."]
pub struct EP2OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2OUT_W<'a> {
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
#[doc = "Field `EP2IN` reader - Interrupt status register bit for the EP2 IN direction."]
pub struct EP2IN_R(crate::FieldReader<bool, bool>);
impl EP2IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP2IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2IN` writer - Interrupt status register bit for the EP2 IN direction."]
pub struct EP2IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2IN_W<'a> {
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
#[doc = "Field `EP3OUT` reader - Interrupt status register bit for the EP3 OUT direction."]
pub struct EP3OUT_R(crate::FieldReader<bool, bool>);
impl EP3OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP3OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP3OUT` writer - Interrupt status register bit for the EP3 OUT direction."]
pub struct EP3OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3OUT_W<'a> {
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
#[doc = "Field `EP3IN` reader - Interrupt status register bit for the EP3 IN direction."]
pub struct EP3IN_R(crate::FieldReader<bool, bool>);
impl EP3IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP3IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP3IN` writer - Interrupt status register bit for the EP3 IN direction."]
pub struct EP3IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3IN_W<'a> {
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
#[doc = "Field `EP4OUT` reader - Interrupt status register bit for the EP4 OUT direction."]
pub struct EP4OUT_R(crate::FieldReader<bool, bool>);
impl EP4OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP4OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP4OUT` writer - Interrupt status register bit for the EP4 OUT direction."]
pub struct EP4OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `EP4IN` reader - Interrupt status register bit for the EP4 IN direction."]
pub struct EP4IN_R(crate::FieldReader<bool, bool>);
impl EP4IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP4IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP4IN` writer - Interrupt status register bit for the EP4 IN direction."]
pub struct EP4IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4IN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `EP5OUT` reader - Interrupt status register bit for the EP5 OUT direction."]
pub struct EP5OUT_R(crate::FieldReader<bool, bool>);
impl EP5OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP5OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP5OUT` writer - Interrupt status register bit for the EP5 OUT direction."]
pub struct EP5OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `EP5IN` reader - Interrupt status register bit for the EP5 IN direction."]
pub struct EP5IN_R(crate::FieldReader<bool, bool>);
impl EP5IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP5IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP5IN` writer - Interrupt status register bit for the EP5 IN direction."]
pub struct EP5IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5IN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `FRAME_INT` reader - Frame interrupt."]
pub struct FRAME_INT_R(crate::FieldReader<bool, bool>);
impl FRAME_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAME_INT` writer - Frame interrupt."]
pub struct FRAME_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_INT_W<'a> {
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
#[doc = "Field `DEV_INT` reader - Device status interrupt."]
pub struct DEV_INT_R(crate::FieldReader<bool, bool>);
impl DEV_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEV_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_INT` writer - Device status interrupt."]
pub struct DEV_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_INT_W<'a> {
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
    #[doc = "Bit 0 - Interrupt status register bit for the Control EP0 OUT direction."]
    #[inline(always)]
    pub fn ep0out(&self) -> EP0OUT_R {
        EP0OUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt status register bit for the Control EP0 IN direction."]
    #[inline(always)]
    pub fn ep0in(&self) -> EP0IN_R {
        EP0IN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt status register bit for the EP1 OUT direction."]
    #[inline(always)]
    pub fn ep1out(&self) -> EP1OUT_R {
        EP1OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt status register bit for the EP1 IN direction."]
    #[inline(always)]
    pub fn ep1in(&self) -> EP1IN_R {
        EP1IN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt status register bit for the EP2 OUT direction."]
    #[inline(always)]
    pub fn ep2out(&self) -> EP2OUT_R {
        EP2OUT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt status register bit for the EP2 IN direction."]
    #[inline(always)]
    pub fn ep2in(&self) -> EP2IN_R {
        EP2IN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt status register bit for the EP3 OUT direction."]
    #[inline(always)]
    pub fn ep3out(&self) -> EP3OUT_R {
        EP3OUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt status register bit for the EP3 IN direction."]
    #[inline(always)]
    pub fn ep3in(&self) -> EP3IN_R {
        EP3IN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt status register bit for the EP4 OUT direction."]
    #[inline(always)]
    pub fn ep4out(&self) -> EP4OUT_R {
        EP4OUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt status register bit for the EP4 IN direction."]
    #[inline(always)]
    pub fn ep4in(&self) -> EP4IN_R {
        EP4IN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt status register bit for the EP5 OUT direction."]
    #[inline(always)]
    pub fn ep5out(&self) -> EP5OUT_R {
        EP5OUT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt status register bit for the EP5 IN direction."]
    #[inline(always)]
    pub fn ep5in(&self) -> EP5IN_R {
        EP5IN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Frame interrupt."]
    #[inline(always)]
    pub fn frame_int(&self) -> FRAME_INT_R {
        FRAME_INT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Device status interrupt."]
    #[inline(always)]
    pub fn dev_int(&self) -> DEV_INT_R {
        DEV_INT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status register bit for the Control EP0 OUT direction."]
    #[inline(always)]
    pub fn ep0out(&mut self) -> EP0OUT_W {
        EP0OUT_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt status register bit for the Control EP0 IN direction."]
    #[inline(always)]
    pub fn ep0in(&mut self) -> EP0IN_W {
        EP0IN_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt status register bit for the EP1 OUT direction."]
    #[inline(always)]
    pub fn ep1out(&mut self) -> EP1OUT_W {
        EP1OUT_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt status register bit for the EP1 IN direction."]
    #[inline(always)]
    pub fn ep1in(&mut self) -> EP1IN_W {
        EP1IN_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt status register bit for the EP2 OUT direction."]
    #[inline(always)]
    pub fn ep2out(&mut self) -> EP2OUT_W {
        EP2OUT_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt status register bit for the EP2 IN direction."]
    #[inline(always)]
    pub fn ep2in(&mut self) -> EP2IN_W {
        EP2IN_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt status register bit for the EP3 OUT direction."]
    #[inline(always)]
    pub fn ep3out(&mut self) -> EP3OUT_W {
        EP3OUT_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt status register bit for the EP3 IN direction."]
    #[inline(always)]
    pub fn ep3in(&mut self) -> EP3IN_W {
        EP3IN_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt status register bit for the EP4 OUT direction."]
    #[inline(always)]
    pub fn ep4out(&mut self) -> EP4OUT_W {
        EP4OUT_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt status register bit for the EP4 IN direction."]
    #[inline(always)]
    pub fn ep4in(&mut self) -> EP4IN_W {
        EP4IN_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt status register bit for the EP5 OUT direction."]
    #[inline(always)]
    pub fn ep5out(&mut self) -> EP5OUT_W {
        EP5OUT_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt status register bit for the EP5 IN direction."]
    #[inline(always)]
    pub fn ep5in(&mut self) -> EP5IN_W {
        EP5IN_W { w: self }
    }
    #[doc = "Bit 30 - Frame interrupt."]
    #[inline(always)]
    pub fn frame_int(&mut self) -> FRAME_INT_W {
        FRAME_INT_W { w: self }
    }
    #[doc = "Bit 31 - Device status interrupt."]
    #[inline(always)]
    pub fn dev_int(&mut self) -> DEV_INT_W {
        DEV_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intstat::W](W) writer structure"]
impl crate::Writable for INTSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
