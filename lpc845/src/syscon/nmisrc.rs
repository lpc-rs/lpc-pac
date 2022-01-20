#[doc = "Register `NMISRC` reader"]
pub struct R(crate::R<NMISRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMISRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMISRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMISRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMISRC` writer"]
pub struct W(crate::W<NMISRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMISRC_SPEC>;
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
impl From<crate::W<NMISRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMISRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQN` reader - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 is 1"]
pub struct IRQN_R(crate::FieldReader<u8, u8>);
impl IRQN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IRQN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQN` writer - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 is 1"]
pub struct IRQN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `NMIEN` reader - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
pub struct NMIEN_R(crate::FieldReader<bool, bool>);
impl NMIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NMIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIEN` writer - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
pub struct NMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIEN_W<'a> {
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
    #[doc = "Bits 0:4 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 is 1"]
    #[inline(always)]
    pub fn irqn(&self) -> IRQN_R {
        IRQN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
    #[inline(always)]
    pub fn nmien(&self) -> NMIEN_R {
        NMIEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) if bit 31 is 1"]
    #[inline(always)]
    pub fn irqn(&mut self) -> IRQN_W {
        IRQN_W { w: self }
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by bits 4:0."]
    #[inline(always)]
    pub fn nmien(&mut self) -> NMIEN_W {
        NMIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI source selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmisrc](index.html) module"]
pub struct NMISRC_SPEC;
impl crate::RegisterSpec for NMISRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmisrc::R](R) reader structure"]
impl crate::Readable for NMISRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmisrc::W](W) writer structure"]
impl crate::Writable for NMISRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMISRC to value 0"]
impl crate::Resettable for NMISRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
