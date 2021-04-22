#[doc = "Register `USBINTR` reader"]
pub struct R(crate::R<USBINTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBINTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<USBINTR_SPEC>> for R {
    fn from(reader: crate::R<USBINTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBINTR` writer"]
pub struct W(crate::W<USBINTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBINTR_SPEC>;
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
impl core::convert::From<crate::W<USBINTR_SPEC>> for W {
    fn from(writer: crate::W<USBINTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCDE` reader - Port Change Detect Interrupt Enable: 1: enable 0: disable."]
pub struct PCDE_R(crate::FieldReader<bool, bool>);
impl PCDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCDE` writer - Port Change Detect Interrupt Enable: 1: enable 0: disable."]
pub struct PCDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCDE_W<'a> {
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
#[doc = "Field `FLRE` reader - Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
pub struct FLRE_R(crate::FieldReader<bool, bool>);
impl FLRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLRE` writer - Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
pub struct FLRE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLRE_W<'a> {
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
#[doc = "Field `ATL_IRQ_E` reader - ATL IRQ Enable bit: 1: enable 0: disable."]
pub struct ATL_IRQ_E_R(crate::FieldReader<bool, bool>);
impl ATL_IRQ_E_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATL_IRQ_E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATL_IRQ_E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATL_IRQ_E` writer - ATL IRQ Enable bit: 1: enable 0: disable."]
pub struct ATL_IRQ_E_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_IRQ_E_W<'a> {
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
#[doc = "Field `ISO_IRQ_E` reader - ISO IRQ Enable bit: 1: enable 0: disable."]
pub struct ISO_IRQ_E_R(crate::FieldReader<bool, bool>);
impl ISO_IRQ_E_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISO_IRQ_E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISO_IRQ_E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISO_IRQ_E` writer - ISO IRQ Enable bit: 1: enable 0: disable."]
pub struct ISO_IRQ_E_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_IRQ_E_W<'a> {
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
#[doc = "Field `INT_IRQ_E` reader - INT IRQ Enable bit: 1: enable 0: disable."]
pub struct INT_IRQ_E_R(crate::FieldReader<bool, bool>);
impl INT_IRQ_E_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_IRQ_E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_IRQ_E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_IRQ_E` writer - INT IRQ Enable bit: 1: enable 0: disable."]
pub struct INT_IRQ_E_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_IRQ_E_W<'a> {
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
#[doc = "Field `SOF_E` reader - SOF Interrupt Enable bit: 1: enable 0: disable."]
pub struct SOF_E_R(crate::FieldReader<bool, bool>);
impl SOF_E_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF_E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF_E` writer - SOF Interrupt Enable bit: 1: enable 0: disable."]
pub struct SOF_E_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_E_W<'a> {
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
    #[doc = "Bit 2 - Port Change Detect Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn pcde(&self) -> PCDE_R {
        PCDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn flre(&self) -> FLRE_R {
        FLRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ATL IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn atl_irq_e(&self) -> ATL_IRQ_E_R {
        ATL_IRQ_E_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ISO IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn iso_irq_e(&self) -> ISO_IRQ_E_R {
        ISO_IRQ_E_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - INT IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn int_irq_e(&self) -> INT_IRQ_E_R {
        INT_IRQ_E_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SOF Interrupt Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn sof_e(&self) -> SOF_E_R {
        SOF_E_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Port Change Detect Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn pcde(&mut self) -> PCDE_W {
        PCDE_W { w: self }
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn flre(&mut self) -> FLRE_W {
        FLRE_W { w: self }
    }
    #[doc = "Bit 16 - ATL IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn atl_irq_e(&mut self) -> ATL_IRQ_E_W {
        ATL_IRQ_E_W { w: self }
    }
    #[doc = "Bit 17 - ISO IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn iso_irq_e(&mut self) -> ISO_IRQ_E_W {
        ISO_IRQ_E_W { w: self }
    }
    #[doc = "Bit 18 - INT IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn int_irq_e(&mut self) -> INT_IRQ_E_W {
        INT_IRQ_E_W { w: self }
    }
    #[doc = "Bit 19 - SOF Interrupt Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn sof_e(&mut self) -> SOF_E_W {
        SOF_E_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbintr](index.html) module"]
pub struct USBINTR_SPEC;
impl crate::RegisterSpec for USBINTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbintr::R](R) reader structure"]
impl crate::Readable for USBINTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbintr::W](W) writer structure"]
impl crate::Writable for USBINTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBINTR to value 0"]
impl crate::Resettable for USBINTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
