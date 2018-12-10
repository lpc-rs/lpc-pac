#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MSTCTL {
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
#[doc = "Possible values of the field `MSTCONTINUE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTCONTINUER {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    CONTINUE,
}
impl MSTCONTINUER {
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
            MSTCONTINUER::NO_EFFECT => false,
            MSTCONTINUER::CONTINUE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTCONTINUER {
        match value {
            false => MSTCONTINUER::NO_EFFECT,
            true => MSTCONTINUER::CONTINUE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTCONTINUER::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline]
    pub fn is_continue_(&self) -> bool {
        *self == MSTCONTINUER::CONTINUE
    }
}
#[doc = "Possible values of the field `MSTSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTARTR {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    START,
}
impl MSTSTARTR {
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
            MSTSTARTR::NO_EFFECT => false,
            MSTSTARTR::START => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTSTARTR {
        match value {
            false => MSTSTARTR::NO_EFFECT,
            true => MSTSTARTR::START,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTSTARTR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == MSTSTARTR::START
    }
}
#[doc = "Possible values of the field `MSTSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTOPR {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    STOP,
}
impl MSTSTOPR {
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
            MSTSTOPR::NO_EFFECT => false,
            MSTSTOPR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTSTOPR {
        match value {
            false => MSTSTOPR::NO_EFFECT,
            true => MSTSTOPR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTSTOPR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == MSTSTOPR::STOP
    }
}
#[doc = "Possible values of the field `MSTDMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTDMAR {
    #[doc = "Disable. No DMA requests are generated for master operation."]
    DISABLE,
    #[doc = "Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    ENABLE,
}
impl MSTDMAR {
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
            MSTDMAR::DISABLE => false,
            MSTDMAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTDMAR {
        match value {
            false => MSTDMAR::DISABLE,
            true => MSTDMAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MSTDMAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MSTDMAR::ENABLE
    }
}
#[doc = "Values that can be written to the field `MSTCONTINUE`"]
pub enum MSTCONTINUEW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    CONTINUE,
}
impl MSTCONTINUEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTCONTINUEW::NO_EFFECT => false,
            MSTCONTINUEW::CONTINUE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTCONTINUEW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTCONTINUEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTCONTINUEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCONTINUEW::NO_EFFECT)
    }
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    #[inline]
    pub fn continue_(self) -> &'a mut W {
        self.variant(MSTCONTINUEW::CONTINUE)
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
#[doc = "Values that can be written to the field `MSTSTART`"]
pub enum MSTSTARTW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    START,
}
impl MSTSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSTARTW::NO_EFFECT => false,
            MSTSTARTW::START => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTSTARTW::NO_EFFECT)
    }
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(MSTSTARTW::START)
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
#[doc = "Values that can be written to the field `MSTSTOP`"]
pub enum MSTSTOPW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    STOP,
}
impl MSTSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSTOPW::NO_EFFECT => false,
            MSTSTOPW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSTOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTSTOPW::NO_EFFECT)
    }
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(MSTSTOPW::STOP)
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
#[doc = "Values that can be written to the field `MSTDMA`"]
pub enum MSTDMAW {
    #[doc = "Disable. No DMA requests are generated for master operation."]
    DISABLE,
    #[doc = "Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    ENABLE,
}
impl MSTDMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTDMAW::DISABLE => false,
            MSTDMAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTDMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTDMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. No DMA requests are generated for master operation."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MSTDMAW::DISABLE)
    }
    #[doc = "Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MSTDMAW::ENABLE)
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
    #[doc = "Bit 0 - Master Continue. This bit is write-only."]
    #[inline]
    pub fn mstcontinue(&self) -> MSTCONTINUER {
        MSTCONTINUER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Master Start control. This bit is write-only."]
    #[inline]
    pub fn mststart(&self) -> MSTSTARTR {
        MSTSTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Master Stop control. This bit is write-only."]
    #[inline]
    pub fn mststop(&self) -> MSTSTOPR {
        MSTSTOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline]
    pub fn mstdma(&self) -> MSTDMAR {
        MSTDMAR::_from({
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
    #[doc = "Bit 0 - Master Continue. This bit is write-only."]
    #[inline]
    pub fn mstcontinue(&mut self) -> _MSTCONTINUEW {
        _MSTCONTINUEW { w: self }
    }
    #[doc = "Bit 1 - Master Start control. This bit is write-only."]
    #[inline]
    pub fn mststart(&mut self) -> _MSTSTARTW {
        _MSTSTARTW { w: self }
    }
    #[doc = "Bit 2 - Master Stop control. This bit is write-only."]
    #[inline]
    pub fn mststop(&mut self) -> _MSTSTOPW {
        _MSTSTOPW { w: self }
    }
    #[doc = "Bit 3 - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline]
    pub fn mstdma(&mut self) -> _MSTDMAW {
        _MSTDMAW { w: self }
    }
}
