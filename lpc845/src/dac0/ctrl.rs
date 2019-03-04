#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `INT_DMA_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_DMA_REQR {
    #[doc = "This bit is cleared on any write to the DACR register."]
    CLEAR,
    #[doc = "This bit is set by hardware when the timer times out."]
    SET,
}
impl INT_DMA_REQR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            INT_DMA_REQR::CLEAR => false,
            INT_DMA_REQR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_DMA_REQR {
        match value {
            false => INT_DMA_REQR::CLEAR,
            true => INT_DMA_REQR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == INT_DMA_REQR::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == INT_DMA_REQR::SET
    }
}
#[doc = "Possible values of the field `DBLBUF_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLBUF_ENAR {
    #[doc = "DACR double-buffering is disabled."]
    DISABLED,
    #[doc = "When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    ENABLED,
}
impl DBLBUF_ENAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DBLBUF_ENAR::DISABLED => false,
            DBLBUF_ENAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBLBUF_ENAR {
        match value {
            false => DBLBUF_ENAR::DISABLED,
            true => DBLBUF_ENAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DBLBUF_ENAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DBLBUF_ENAR::ENABLED
    }
}
#[doc = "Possible values of the field `CNT_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNT_ENAR {
    #[doc = "Time-out counter operation is disabled."]
    DISABLED,
    #[doc = "Time-out counter operation is enabled."]
    ENABLED,
}
impl CNT_ENAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CNT_ENAR::DISABLED => false,
            CNT_ENAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNT_ENAR {
        match value {
            false => CNT_ENAR::DISABLED,
            true => CNT_ENAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CNT_ENAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CNT_ENAR::ENABLED
    }
}
#[doc = "Possible values of the field `DMA_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ENAR {
    #[doc = "DMA access is disabled."]
    DISABLED,
    #[doc = "DMA Burst Request Input 7 is enabled for the DAC"]
    ENABLED,
}
impl DMA_ENAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DMA_ENAR::DISABLED => false,
            DMA_ENAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_ENAR {
        match value {
            false => DMA_ENAR::DISABLED,
            true => DMA_ENAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_ENAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_ENAR::ENABLED
    }
}
#[doc = "Values that can be written to the field `INT_DMA_REQ`"]
pub enum INT_DMA_REQW {
    #[doc = "This bit is cleared on any write to the DACR register."]
    CLEAR,
    #[doc = "This bit is set by hardware when the timer times out."]
    SET,
}
impl INT_DMA_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT_DMA_REQW::CLEAR => false,
            INT_DMA_REQW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT_DMA_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_DMA_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT_DMA_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is cleared on any write to the DACR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(INT_DMA_REQW::CLEAR)
    }
    #[doc = "This bit is set by hardware when the timer times out."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(INT_DMA_REQW::SET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBLBUF_ENA`"]
pub enum DBLBUF_ENAW {
    #[doc = "DACR double-buffering is disabled."]
    DISABLED,
    #[doc = "When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    ENABLED,
}
impl DBLBUF_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBLBUF_ENAW::DISABLED => false,
            DBLBUF_ENAW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBLBUF_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _DBLBUF_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBLBUF_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DACR double-buffering is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBLBUF_ENAW::DISABLED)
    }
    #[doc = "When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBLBUF_ENAW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNT_ENA`"]
pub enum CNT_ENAW {
    #[doc = "Time-out counter operation is disabled."]
    DISABLED,
    #[doc = "Time-out counter operation is enabled."]
    ENABLED,
}
impl CNT_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNT_ENAW::DISABLED => false,
            CNT_ENAW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNT_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _CNT_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNT_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Time-out counter operation is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CNT_ENAW::DISABLED)
    }
    #[doc = "Time-out counter operation is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CNT_ENAW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA_ENA`"]
pub enum DMA_ENAW {
    #[doc = "DMA access is disabled."]
    DISABLED,
    #[doc = "DMA Burst Request Input 7 is enabled for the DAC"]
    ENABLED,
}
impl DMA_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_ENAW::DISABLED => false,
            DMA_ENAW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA access is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_ENAW::DISABLED)
    }
    #[doc = "DMA Burst Request Input 7 is enabled for the DAC"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_ENAW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - DMA request"]
    #[inline]
    pub fn int_dma_req(&self) -> INT_DMA_REQR {
        INT_DMA_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - dacr double buffer"]
    #[inline]
    pub fn dblbuf_ena(&self) -> DBLBUF_ENAR {
        DBLBUF_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - time-out counter operation"]
    #[inline]
    pub fn cnt_ena(&self) -> CNT_ENAR {
        CNT_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - DMA access"]
    #[inline]
    pub fn dma_ena(&self) -> DMA_ENAR {
        DMA_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - DMA request"]
    #[inline]
    pub fn int_dma_req(&mut self) -> _INT_DMA_REQW {
        _INT_DMA_REQW { w: self }
    }
    #[doc = "Bit 1 - dacr double buffer"]
    #[inline]
    pub fn dblbuf_ena(&mut self) -> _DBLBUF_ENAW {
        _DBLBUF_ENAW { w: self }
    }
    #[doc = "Bit 2 - time-out counter operation"]
    #[inline]
    pub fn cnt_ena(&mut self) -> _CNT_ENAW {
        _CNT_ENAW { w: self }
    }
    #[doc = "Bit 3 - DMA access"]
    #[inline]
    pub fn dma_ena(&mut self) -> _DMA_ENAW {
        _DMA_ENAW { w: self }
    }
}
