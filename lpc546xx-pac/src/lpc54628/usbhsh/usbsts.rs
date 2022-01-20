#[doc = "Register `USBSTS` reader"]
pub struct R(crate::R<USBSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBSTS` writer"]
pub struct W(crate::W<USBSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBSTS_SPEC>;
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
impl From<crate::W<USBSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCD` reader - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
pub struct PCD_R(crate::FieldReader<bool, bool>);
impl PCD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCD` writer - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
pub struct PCD_W<'a> {
    w: &'a mut W,
}
impl<'a> PCD_W<'a> {
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
#[doc = "Field `FLR` reader - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
pub struct FLR_R(crate::FieldReader<bool, bool>);
impl FLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLR` writer - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
pub struct FLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLR_W<'a> {
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
#[doc = "Field `ATL_IRQ` reader - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
pub struct ATL_IRQ_R(crate::FieldReader<bool, bool>);
impl ATL_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ATL_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATL_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATL_IRQ` writer - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
pub struct ATL_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `ISO_IRQ` reader - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
pub struct ISO_IRQ_R(crate::FieldReader<bool, bool>);
impl ISO_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISO_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISO_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISO_IRQ` writer - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
pub struct ISO_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `INT_IRQ` reader - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
pub struct INT_IRQ_R(crate::FieldReader<bool, bool>);
impl INT_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_IRQ` writer - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
pub struct INT_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `SOF_IRQ` reader - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
pub struct SOF_IRQ_R(crate::FieldReader<bool, bool>);
impl SOF_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOF_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF_IRQ` writer - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
pub struct SOF_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_IRQ_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
    #[inline(always)]
    pub fn pcd(&self) -> PCD_R {
        PCD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[inline(always)]
    pub fn flr(&self) -> FLR_R {
        FLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn atl_irq(&self) -> ATL_IRQ_R {
        ATL_IRQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn iso_irq(&self) -> ISO_IRQ_R {
        ISO_IRQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn int_irq(&self) -> INT_IRQ_R {
        INT_IRQ_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[inline(always)]
    pub fn sof_irq(&self) -> SOF_IRQ_R {
        SOF_IRQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
    #[inline(always)]
    pub fn pcd(&mut self) -> PCD_W {
        PCD_W { w: self }
    }
    #[doc = "Bit 3 - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[inline(always)]
    pub fn flr(&mut self) -> FLR_W {
        FLR_W { w: self }
    }
    #[doc = "Bit 16 - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn atl_irq(&mut self) -> ATL_IRQ_W {
        ATL_IRQ_W { w: self }
    }
    #[doc = "Bit 17 - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn iso_irq(&mut self) -> ISO_IRQ_W {
        ISO_IRQ_W { w: self }
    }
    #[doc = "Bit 18 - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn int_irq(&mut self) -> INT_IRQ_W {
        INT_IRQ_W { w: self }
    }
    #[doc = "Bit 19 - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[inline(always)]
    pub fn sof_irq(&mut self) -> SOF_IRQ_W {
        SOF_IRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsts](index.html) module"]
pub struct USBSTS_SPEC;
impl crate::RegisterSpec for USBSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbsts::R](R) reader structure"]
impl crate::Readable for USBSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbsts::W](W) writer structure"]
impl crate::Writable for USBSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBSTS to value 0"]
impl crate::Resettable for USBSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
