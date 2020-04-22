#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA interrupt request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_DMA_REQ_A {
    #[doc = "0: Clear on any write to the DACR register."]
    CLEAR_ON_ANY_WRITE_T = 0,
    #[doc = "1: Set by hardware when the timer times out."]
    SET_BY_HARDWARE_WHEN = 1,
}
impl From<INT_DMA_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: INT_DMA_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT_DMA_REQ`"]
pub type INT_DMA_REQ_R = crate::R<bool, INT_DMA_REQ_A>;
impl INT_DMA_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_DMA_REQ_A {
        match self.bits {
            false => INT_DMA_REQ_A::CLEAR_ON_ANY_WRITE_T,
            true => INT_DMA_REQ_A::SET_BY_HARDWARE_WHEN,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_ON_ANY_WRITE_T`"]
    #[inline(always)]
    pub fn is_clear_on_any_write_t(&self) -> bool {
        *self == INT_DMA_REQ_A::CLEAR_ON_ANY_WRITE_T
    }
    #[doc = "Checks if the value of the field is `SET_BY_HARDWARE_WHEN`"]
    #[inline(always)]
    pub fn is_set_by_hardware_when(&self) -> bool {
        *self == INT_DMA_REQ_A::SET_BY_HARDWARE_WHEN
    }
}
#[doc = "Write proxy for field `INT_DMA_REQ`"]
pub struct INT_DMA_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_DMA_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_DMA_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear on any write to the DACR register."]
    #[inline(always)]
    pub fn clear_on_any_write_t(self) -> &'a mut W {
        self.variant(INT_DMA_REQ_A::CLEAR_ON_ANY_WRITE_T)
    }
    #[doc = "Set by hardware when the timer times out."]
    #[inline(always)]
    pub fn set_by_hardware_when(self) -> &'a mut W {
        self.variant(INT_DMA_REQ_A::SET_BY_HARDWARE_WHEN)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Double buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLBUF_ENA_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    ENABLE_WHEN_THIS_BI = 1,
}
impl From<DBLBUF_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: DBLBUF_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBLBUF_ENA`"]
pub type DBLBUF_ENA_R = crate::R<bool, DBLBUF_ENA_A>;
impl DBLBUF_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLBUF_ENA_A {
        match self.bits {
            false => DBLBUF_ENA_A::DISABLE,
            true => DBLBUF_ENA_A::ENABLE_WHEN_THIS_BI,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DBLBUF_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE_WHEN_THIS_BI`"]
    #[inline(always)]
    pub fn is_enable_when_this_bi(&self) -> bool {
        *self == DBLBUF_ENA_A::ENABLE_WHEN_THIS_BI
    }
}
#[doc = "Write proxy for field `DBLBUF_ENA`"]
pub struct DBLBUF_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DBLBUF_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBLBUF_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DBLBUF_ENA_A::DISABLE)
    }
    #[doc = "Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    #[inline(always)]
    pub fn enable_when_this_bi(self) -> &'a mut W {
        self.variant(DBLBUF_ENA_A::ENABLE_WHEN_THIS_BI)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Time-out counter operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNT_ENA_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CNT_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: CNT_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CNT_ENA`"]
pub type CNT_ENA_R = crate::R<bool, CNT_ENA_A>;
impl CNT_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNT_ENA_A {
        match self.bits {
            false => CNT_ENA_A::DISABLE,
            true => CNT_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CNT_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CNT_ENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `CNT_ENA`"]
pub struct CNT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNT_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CNT_ENA_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CNT_ENA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "DMA access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ENA_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)."]
    ENABLE_DMA_BURST_RE = 1,
}
impl From<DMA_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_ENA`"]
pub type DMA_ENA_R = crate::R<bool, DMA_ENA_A>;
impl DMA_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_ENA_A {
        match self.bits {
            false => DMA_ENA_A::DISABLE,
            true => DMA_ENA_A::ENABLE_DMA_BURST_RE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE_DMA_BURST_RE`"]
    #[inline(always)]
    pub fn is_enable_dma_burst_re(&self) -> bool {
        *self == DMA_ENA_A::ENABLE_DMA_BURST_RE
    }
}
#[doc = "Write proxy for field `DMA_ENA`"]
pub struct DMA_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_ENA_A::DISABLE)
    }
    #[doc = "Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)."]
    #[inline(always)]
    pub fn enable_dma_burst_re(self) -> &'a mut W {
        self.variant(DMA_ENA_A::ENABLE_DMA_BURST_RE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA interrupt request"]
    #[inline(always)]
    pub fn int_dma_req(&self) -> INT_DMA_REQ_R {
        INT_DMA_REQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Double buffering"]
    #[inline(always)]
    pub fn dblbuf_ena(&self) -> DBLBUF_ENA_R {
        DBLBUF_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Time-out counter operation"]
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
    #[doc = "Bit 0 - DMA interrupt request"]
    #[inline(always)]
    pub fn int_dma_req(&mut self) -> INT_DMA_REQ_W {
        INT_DMA_REQ_W { w: self }
    }
    #[doc = "Bit 1 - Double buffering"]
    #[inline(always)]
    pub fn dblbuf_ena(&mut self) -> DBLBUF_ENA_W {
        DBLBUF_ENA_W { w: self }
    }
    #[doc = "Bit 2 - Time-out counter operation"]
    #[inline(always)]
    pub fn cnt_ena(&mut self) -> CNT_ENA_W {
        CNT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - DMA access"]
    #[inline(always)]
    pub fn dma_ena(&mut self) -> DMA_ENA_W {
        DMA_ENA_W { w: self }
    }
}
