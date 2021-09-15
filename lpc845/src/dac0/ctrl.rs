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
#[doc = "DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_DMA_REQ_A {
    #[doc = "0: This bit is cleared on any write to the DACR register."]
    CLEAR = 0,
    #[doc = "1: This bit is set by hardware when the timer times out."]
    SET = 1,
}
impl From<INT_DMA_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: INT_DMA_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_DMA_REQ` reader - DMA request"]
pub struct INT_DMA_REQ_R(crate::FieldReader<bool, INT_DMA_REQ_A>);
impl INT_DMA_REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_DMA_REQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_DMA_REQ_A {
        match self.bits {
            false => INT_DMA_REQ_A::CLEAR,
            true => INT_DMA_REQ_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == INT_DMA_REQ_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == INT_DMA_REQ_A::SET
    }
}
impl core::ops::Deref for INT_DMA_REQ_R {
    type Target = crate::FieldReader<bool, INT_DMA_REQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_DMA_REQ` writer - DMA request"]
pub struct INT_DMA_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_DMA_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_DMA_REQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This bit is cleared on any write to the DACR register."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(INT_DMA_REQ_A::CLEAR)
    }
    #[doc = "This bit is set by hardware when the timer times out."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(INT_DMA_REQ_A::SET)
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
#[doc = "dacr double buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLBUF_ENA_A {
    #[doc = "0: DACR double-buffering is disabled."]
    DISABLED = 0,
    #[doc = "1: When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    ENABLED = 1,
}
impl From<DBLBUF_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: DBLBUF_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBLBUF_ENA` reader - dacr double buffer"]
pub struct DBLBUF_ENA_R(crate::FieldReader<bool, DBLBUF_ENA_A>);
impl DBLBUF_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBLBUF_ENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLBUF_ENA_A {
        match self.bits {
            false => DBLBUF_ENA_A::DISABLED,
            true => DBLBUF_ENA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DBLBUF_ENA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DBLBUF_ENA_A::ENABLED
    }
}
impl core::ops::Deref for DBLBUF_ENA_R {
    type Target = crate::FieldReader<bool, DBLBUF_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBLBUF_ENA` writer - dacr double buffer"]
pub struct DBLBUF_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DBLBUF_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBLBUF_ENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DACR double-buffering is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBLBUF_ENA_A::DISABLED)
    }
    #[doc = "When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBLBUF_ENA_A::ENABLED)
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
#[doc = "time-out counter operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNT_ENA_A {
    #[doc = "0: Time-out counter operation is disabled."]
    DISABLED = 0,
    #[doc = "1: Time-out counter operation is enabled."]
    ENABLED = 1,
}
impl From<CNT_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: CNT_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNT_ENA` reader - time-out counter operation"]
pub struct CNT_ENA_R(crate::FieldReader<bool, CNT_ENA_A>);
impl CNT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNT_ENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNT_ENA_A {
        match self.bits {
            false => CNT_ENA_A::DISABLED,
            true => CNT_ENA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CNT_ENA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CNT_ENA_A::ENABLED
    }
}
impl core::ops::Deref for CNT_ENA_R {
    type Target = crate::FieldReader<bool, CNT_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_ENA` writer - time-out counter operation"]
pub struct CNT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNT_ENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Time-out counter operation is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CNT_ENA_A::DISABLED)
    }
    #[doc = "Time-out counter operation is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CNT_ENA_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "DMA access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ENA_A {
    #[doc = "0: DMA access is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA Burst Request Input 7 is enabled for the DAC"]
    ENABLED = 1,
}
impl From<DMA_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_ENA` reader - DMA access"]
pub struct DMA_ENA_R(crate::FieldReader<bool, DMA_ENA_A>);
impl DMA_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_ENA_A {
        match self.bits {
            false => DMA_ENA_A::DISABLED,
            true => DMA_ENA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMA_ENA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMA_ENA_A::ENABLED
    }
}
impl core::ops::Deref for DMA_ENA_R {
    type Target = crate::FieldReader<bool, DMA_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_ENA` writer - DMA access"]
pub struct DMA_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_ENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA access is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_ENA_A::DISABLED)
    }
    #[doc = "DMA Burst Request Input 7 is enabled for the DAC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_ENA_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA request"]
    #[inline(always)]
    pub fn int_dma_req(&self) -> INT_DMA_REQ_R {
        INT_DMA_REQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - dacr double buffer"]
    #[inline(always)]
    pub fn dblbuf_ena(&self) -> DBLBUF_ENA_R {
        DBLBUF_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - time-out counter operation"]
    #[inline(always)]
    pub fn cnt_ena(&self) -> CNT_ENA_R {
        CNT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA access"]
    #[inline(always)]
    pub fn dma_ena(&self) -> DMA_ENA_R {
        DMA_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA request"]
    #[inline(always)]
    pub fn int_dma_req(&mut self) -> INT_DMA_REQ_W {
        INT_DMA_REQ_W { w: self }
    }
    #[doc = "Bit 1 - dacr double buffer"]
    #[inline(always)]
    pub fn dblbuf_ena(&mut self) -> DBLBUF_ENA_W {
        DBLBUF_ENA_W { w: self }
    }
    #[doc = "Bit 2 - time-out counter operation"]
    #[inline(always)]
    pub fn cnt_ena(&mut self) -> CNT_ENA_W {
        CNT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - DMA access"]
    #[inline(always)]
    pub fn dma_ena(&mut self) -> DMA_ENA_W {
        DMA_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Control register. This register controls DMA and timer operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
