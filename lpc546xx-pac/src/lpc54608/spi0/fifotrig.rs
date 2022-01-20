#[doc = "Register `FIFOTRIG` reader"]
pub struct R(crate::R<FIFOTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOTRIG` writer"]
pub struct W(crate::W<FIFOTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOTRIG_SPEC>;
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
impl From<crate::W<FIFOTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLVLENA_A {
    #[doc = "0: Transmit FIFO level does not generate a FIFO level trigger."]
    DISABLED = 0,
    #[doc = "1: An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    ENABLED = 1,
}
impl From<TXLVLENA_A> for bool {
    #[inline(always)]
    fn from(variant: TXLVLENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXLVLENA` reader - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
pub struct TXLVLENA_R(crate::FieldReader<bool, TXLVLENA_A>);
impl TXLVLENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXLVLENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXLVLENA_A {
        match self.bits {
            false => TXLVLENA_A::DISABLED,
            true => TXLVLENA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXLVLENA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXLVLENA_A::ENABLED
    }
}
impl core::ops::Deref for TXLVLENA_R {
    type Target = crate::FieldReader<bool, TXLVLENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXLVLENA` writer - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
pub struct TXLVLENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLVLENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXLVLENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit FIFO level does not generate a FIFO level trigger."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXLVLENA_A::DISABLED)
    }
    #[doc = "An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXLVLENA_A::ENABLED)
    }
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
#[doc = "Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLVLENA_A {
    #[doc = "0: Receive FIFO level does not generate a FIFO level trigger."]
    DISABLED = 0,
    #[doc = "1: An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    ENABLED = 1,
}
impl From<RXLVLENA_A> for bool {
    #[inline(always)]
    fn from(variant: RXLVLENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXLVLENA` reader - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
pub struct RXLVLENA_R(crate::FieldReader<bool, RXLVLENA_A>);
impl RXLVLENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXLVLENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXLVLENA_A {
        match self.bits {
            false => RXLVLENA_A::DISABLED,
            true => RXLVLENA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXLVLENA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXLVLENA_A::ENABLED
    }
}
impl core::ops::Deref for RXLVLENA_R {
    type Target = crate::FieldReader<bool, RXLVLENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLVLENA` writer - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
pub struct RXLVLENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLVLENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXLVLENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive FIFO level does not generate a FIFO level trigger."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXLVLENA_A::DISABLED)
    }
    #[doc = "An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXLVLENA_A::ENABLED)
    }
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
#[doc = "Field `TXLVL` reader - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
pub struct TXLVL_R(crate::FieldReader<u8, u8>);
impl TXLVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXLVL` writer - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
pub struct TXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RXLVL` reader - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
pub struct RXLVL_R(crate::FieldReader<u8, u8>);
impl RXLVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXLVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLVL` writer - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
pub struct RXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline(always)]
    pub fn txlvlena(&self) -> TXLVLENA_R {
        TXLVLENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline(always)]
    pub fn rxlvlena(&self) -> RXLVLENA_R {
        RXLVLENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline(always)]
    pub fn txlvlena(&mut self) -> TXLVLENA_W {
        TXLVLENA_W { w: self }
    }
    #[doc = "Bit 1 - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline(always)]
    pub fn rxlvlena(&mut self) -> RXLVLENA_W {
        RXLVLENA_W { w: self }
    }
    #[doc = "Bits 8:11 - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    pub fn txlvl(&mut self) -> TXLVL_W {
        TXLVL_W { w: self }
    }
    #[doc = "Bits 16:19 - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn rxlvl(&mut self) -> RXLVL_W {
        RXLVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO trigger settings for interrupt and DMA request.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifotrig](index.html) module"]
pub struct FIFOTRIG_SPEC;
impl crate::RegisterSpec for FIFOTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifotrig::R](R) reader structure"]
impl crate::Readable for FIFOTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifotrig::W](W) writer structure"]
impl crate::Writable for FIFOTRIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOTRIG to value 0"]
impl crate::Resettable for FIFOTRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
