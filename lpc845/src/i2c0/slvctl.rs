#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLVCTL {
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
#[doc = "Possible values of the field `SLVCONTINUE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVCONTINUER {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Informs the Slave function to continue to the next operation."]
    CONTINUE,
}
impl SLVCONTINUER {
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
            SLVCONTINUER::NO_EFFECT => false,
            SLVCONTINUER::CONTINUE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVCONTINUER {
        match value {
            false => SLVCONTINUER::NO_EFFECT,
            true => SLVCONTINUER::CONTINUE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == SLVCONTINUER::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline]
    pub fn is_continue_(&self) -> bool {
        *self == SLVCONTINUER::CONTINUE
    }
}
#[doc = "Possible values of the field `SLVNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNACKR {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    NACK,
}
impl SLVNACKR {
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
            SLVNACKR::NO_EFFECT => false,
            SLVNACKR::NACK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVNACKR {
        match value {
            false => SLVNACKR::NO_EFFECT,
            true => SLVNACKR::NACK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == SLVNACKR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline]
    pub fn is_nack(&self) -> bool {
        *self == SLVNACKR::NACK
    }
}
#[doc = "Possible values of the field `SLVDMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDMAR {
    #[doc = "Disabled. No DMA requests are issued for Slave mode operation."]
    DISABLED,
    #[doc = "Enabled. DMA requests are issued for I2C slave data transmission and reception."]
    ENABLED,
}
impl SLVDMAR {
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
            SLVDMAR::DISABLED => false,
            SLVDMAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVDMAR {
        match value {
            false => SLVDMAR::DISABLED,
            true => SLVDMAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SLVDMAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SLVDMAR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SLVCONTINUE`"]
pub enum SLVCONTINUEW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Informs the Slave function to continue to the next operation."]
    CONTINUE,
}
impl SLVCONTINUEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVCONTINUEW::NO_EFFECT => false,
            SLVCONTINUEW::CONTINUE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVCONTINUEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVCONTINUEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVCONTINUEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SLVCONTINUEW::NO_EFFECT)
    }
    #[doc = "Informs the Slave function to continue to the next operation."]
    #[inline]
    pub fn continue_(self) -> &'a mut W {
        self.variant(SLVCONTINUEW::CONTINUE)
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
#[doc = "Values that can be written to the field `SLVNACK`"]
pub enum SLVNACKW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    NACK,
}
impl SLVNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVNACKW::NO_EFFECT => false,
            SLVNACKW::NACK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVNACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVNACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SLVNACKW::NO_EFFECT)
    }
    #[doc = "NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    #[inline]
    pub fn nack(self) -> &'a mut W {
        self.variant(SLVNACKW::NACK)
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
#[doc = "Values that can be written to the field `SLVDMA`"]
pub enum SLVDMAW {
    #[doc = "Disabled. No DMA requests are issued for Slave mode operation."]
    DISABLED,
    #[doc = "Enabled. DMA requests are issued for I2C slave data transmission and reception."]
    ENABLED,
}
impl SLVDMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVDMAW::DISABLED => false,
            SLVDMAW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVDMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVDMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. No DMA requests are issued for Slave mode operation."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVDMAW::DISABLED)
    }
    #[doc = "Enabled. DMA requests are issued for I2C slave data transmission and reception."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVDMAW::ENABLED)
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
    #[doc = "Bit 0 - Slave Continue."]
    #[inline]
    pub fn slvcontinue(&self) -> SLVCONTINUER {
        SLVCONTINUER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Slave NACK."]
    #[inline]
    pub fn slvnack(&self) -> SLVNACKR {
        SLVNACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Slave DMA enable."]
    #[inline]
    pub fn slvdma(&self) -> SLVDMAR {
        SLVDMAR::_from({
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
    #[doc = "Bit 0 - Slave Continue."]
    #[inline]
    pub fn slvcontinue(&mut self) -> _SLVCONTINUEW {
        _SLVCONTINUEW { w: self }
    }
    #[doc = "Bit 1 - Slave NACK."]
    #[inline]
    pub fn slvnack(&mut self) -> _SLVNACKW {
        _SLVNACKW { w: self }
    }
    #[doc = "Bit 3 - Slave DMA enable."]
    #[inline]
    pub fn slvdma(&mut self) -> _SLVDMAW {
        _SLVDMAW { w: self }
    }
}
