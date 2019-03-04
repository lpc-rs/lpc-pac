#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `PERIPHREQEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPHREQENR {
    #[doc = "Disabled. Peripheral DMA requests are disabled."]
    DISABLED,
    #[doc = "Enabled. Peripheral DMA requests are enabled."]
    ENABLED,
}
impl PERIPHREQENR {
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
            PERIPHREQENR::DISABLED => false,
            PERIPHREQENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PERIPHREQENR {
        match value {
            false => PERIPHREQENR::DISABLED,
            true => PERIPHREQENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PERIPHREQENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PERIPHREQENR::ENABLED
    }
}
#[doc = "Possible values of the field `HWTRIGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWTRIGENR {
    #[doc = "Disabled. Hardware triggering is not used."]
    DISABLED,
    #[doc = "Enabled. Use hardware triggering."]
    ENABLED,
}
impl HWTRIGENR {
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
            HWTRIGENR::DISABLED => false,
            HWTRIGENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HWTRIGENR {
        match value {
            false => HWTRIGENR::DISABLED,
            true => HWTRIGENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HWTRIGENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HWTRIGENR::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGPOLR {
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    ACTIVE_LOW_FALLING,
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    ACTIVE_HIGH_RISING,
}
impl TRIGPOLR {
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
            TRIGPOLR::ACTIVE_LOW_FALLING => false,
            TRIGPOLR::ACTIVE_HIGH_RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGPOLR {
        match value {
            false => TRIGPOLR::ACTIVE_LOW_FALLING,
            true => TRIGPOLR::ACTIVE_HIGH_RISING,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW_FALLING`"]
    #[inline]
    pub fn is_active_low_falling(&self) -> bool {
        *self == TRIGPOLR::ACTIVE_LOW_FALLING
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH_RISING`"]
    #[inline]
    pub fn is_active_high_rising(&self) -> bool {
        *self == TRIGPOLR::ACTIVE_HIGH_RISING
    }
}
#[doc = "Possible values of the field `TRIGTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGTYPER {
    #[doc = "Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    EDGE,
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed."]
    LEVEL,
}
impl TRIGTYPER {
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
            TRIGTYPER::EDGE => false,
            TRIGTYPER::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGTYPER {
        match value {
            false => TRIGTYPER::EDGE,
            true => TRIGTYPER::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == TRIGTYPER::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == TRIGTYPER::LEVEL
    }
}
#[doc = "Possible values of the field `TRIGBURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGBURSTR {
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    SINGLE,
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    BURST,
}
impl TRIGBURSTR {
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
            TRIGBURSTR::SINGLE => false,
            TRIGBURSTR::BURST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGBURSTR {
        match value {
            false => TRIGBURSTR::SINGLE,
            true => TRIGBURSTR::BURST,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == TRIGBURSTR::SINGLE
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline]
    pub fn is_burst(&self) -> bool {
        *self == TRIGBURSTR::BURST
    }
}
#[doc = r" Value of the field"]
pub struct BURSTPOWERR {
    bits: u8,
}
impl BURSTPOWERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SRCBURSTWRAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCBURSTWRAPR {
    #[doc = "Disabled. Source burst wrapping is not enabled for this DMA channel."]
    DISABLED,
    #[doc = "Enabled. Source burst wrapping is enabled for this DMA channel."]
    ENABLED,
}
impl SRCBURSTWRAPR {
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
            SRCBURSTWRAPR::DISABLED => false,
            SRCBURSTWRAPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRCBURSTWRAPR {
        match value {
            false => SRCBURSTWRAPR::DISABLED,
            true => SRCBURSTWRAPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SRCBURSTWRAPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SRCBURSTWRAPR::ENABLED
    }
}
#[doc = "Possible values of the field `DSTBURSTWRAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSTBURSTWRAPR {
    #[doc = "Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    DISABLED,
    #[doc = "Enabled. Destination burst wrapping is enabled for this DMA channel."]
    ENABLED,
}
impl DSTBURSTWRAPR {
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
            DSTBURSTWRAPR::DISABLED => false,
            DSTBURSTWRAPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DSTBURSTWRAPR {
        match value {
            false => DSTBURSTWRAPR::DISABLED,
            true => DSTBURSTWRAPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DSTBURSTWRAPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DSTBURSTWRAPR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct CHPRIORITYR {
    bits: u8,
}
impl CHPRIORITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `PERIPHREQEN`"]
pub enum PERIPHREQENW {
    #[doc = "Disabled. Peripheral DMA requests are disabled."]
    DISABLED,
    #[doc = "Enabled. Peripheral DMA requests are enabled."]
    ENABLED,
}
impl PERIPHREQENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PERIPHREQENW::DISABLED => false,
            PERIPHREQENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERIPHREQENW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIPHREQENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERIPHREQENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Peripheral DMA requests are disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PERIPHREQENW::DISABLED)
    }
    #[doc = "Enabled. Peripheral DMA requests are enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PERIPHREQENW::ENABLED)
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
#[doc = "Values that can be written to the field `HWTRIGEN`"]
pub enum HWTRIGENW {
    #[doc = "Disabled. Hardware triggering is not used."]
    DISABLED,
    #[doc = "Enabled. Use hardware triggering."]
    ENABLED,
}
impl HWTRIGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HWTRIGENW::DISABLED => false,
            HWTRIGENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HWTRIGENW<'a> {
    w: &'a mut W,
}
impl<'a> _HWTRIGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HWTRIGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Hardware triggering is not used."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HWTRIGENW::DISABLED)
    }
    #[doc = "Enabled. Use hardware triggering."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HWTRIGENW::ENABLED)
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
#[doc = "Values that can be written to the field `TRIGPOL`"]
pub enum TRIGPOLW {
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    ACTIVE_LOW_FALLING,
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    ACTIVE_HIGH_RISING,
}
impl TRIGPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGPOLW::ACTIVE_LOW_FALLING => false,
            TRIGPOLW::ACTIVE_HIGH_RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    #[inline]
    pub fn active_low_falling(self) -> &'a mut W {
        self.variant(TRIGPOLW::ACTIVE_LOW_FALLING)
    }
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    #[inline]
    pub fn active_high_rising(self) -> &'a mut W {
        self.variant(TRIGPOLW::ACTIVE_HIGH_RISING)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGTYPE`"]
pub enum TRIGTYPEW {
    #[doc = "Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    EDGE,
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed."]
    LEVEL,
}
impl TRIGTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGTYPEW::EDGE => false,
            TRIGTYPEW::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGTYPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(TRIGTYPEW::EDGE)
    }
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed."]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(TRIGTYPEW::LEVEL)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGBURST`"]
pub enum TRIGBURSTW {
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    SINGLE,
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    BURST,
}
impl TRIGBURSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGBURSTW::SINGLE => false,
            TRIGBURSTW::BURST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGBURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGBURSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGBURSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(TRIGBURSTW::SINGLE)
    }
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    #[inline]
    pub fn burst(self) -> &'a mut W {
        self.variant(TRIGBURSTW::BURST)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BURSTPOWERW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTPOWERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRCBURSTWRAP`"]
pub enum SRCBURSTWRAPW {
    #[doc = "Disabled. Source burst wrapping is not enabled for this DMA channel."]
    DISABLED,
    #[doc = "Enabled. Source burst wrapping is enabled for this DMA channel."]
    ENABLED,
}
impl SRCBURSTWRAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRCBURSTWRAPW::DISABLED => false,
            SRCBURSTWRAPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCBURSTWRAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCBURSTWRAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCBURSTWRAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Source burst wrapping is not enabled for this DMA channel."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRCBURSTWRAPW::DISABLED)
    }
    #[doc = "Enabled. Source burst wrapping is enabled for this DMA channel."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRCBURSTWRAPW::ENABLED)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DSTBURSTWRAP`"]
pub enum DSTBURSTWRAPW {
    #[doc = "Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    DISABLED,
    #[doc = "Enabled. Destination burst wrapping is enabled for this DMA channel."]
    ENABLED,
}
impl DSTBURSTWRAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DSTBURSTWRAPW::DISABLED => false,
            DSTBURSTWRAPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSTBURSTWRAPW<'a> {
    w: &'a mut W,
}
impl<'a> _DSTBURSTWRAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSTBURSTWRAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DSTBURSTWRAPW::DISABLED)
    }
    #[doc = "Enabled. Destination burst wrapping is enabled for this DMA channel."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DSTBURSTWRAPW::ENABLED)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHPRIORITYW<'a> {
    w: &'a mut W,
}
impl<'a> _CHPRIORITYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[inline]
    pub fn periphreqen(&self) -> PERIPHREQENR {
        PERIPHREQENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Hardware Triggering Enable for this channel."]
    #[inline]
    pub fn hwtrigen(&self) -> HWTRIGENR {
        HWTRIGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[inline]
    pub fn trigpol(&self) -> TRIGPOLR {
        TRIGPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[inline]
    pub fn trigtype(&self) -> TRIGTYPER {
        TRIGTYPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[inline]
    pub fn trigburst(&self) -> TRIGBURSTR {
        TRIGBURSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
    #[inline]
    pub fn burstpower(&self) -> BURSTPOWERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BURSTPOWERR { bits }
    }
    #[doc = "Bit 14 - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[inline]
    pub fn srcburstwrap(&self) -> SRCBURSTWRAPR {
        SRCBURSTWRAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[inline]
    pub fn dstburstwrap(&self) -> DSTBURSTWRAPR {
        DSTBURSTWRAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
    #[inline]
    pub fn chpriority(&self) -> CHPRIORITYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHPRIORITYR { bits }
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
    #[doc = "Bit 0 - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[inline]
    pub fn periphreqen(&mut self) -> _PERIPHREQENW {
        _PERIPHREQENW { w: self }
    }
    #[doc = "Bit 1 - Hardware Triggering Enable for this channel."]
    #[inline]
    pub fn hwtrigen(&mut self) -> _HWTRIGENW {
        _HWTRIGENW { w: self }
    }
    #[doc = "Bit 4 - Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[inline]
    pub fn trigpol(&mut self) -> _TRIGPOLW {
        _TRIGPOLW { w: self }
    }
    #[doc = "Bit 5 - Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[inline]
    pub fn trigtype(&mut self) -> _TRIGTYPEW {
        _TRIGTYPEW { w: self }
    }
    #[doc = "Bit 6 - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[inline]
    pub fn trigburst(&mut self) -> _TRIGBURSTW {
        _TRIGBURSTW { w: self }
    }
    #[doc = "Bits 8:11 - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
    #[inline]
    pub fn burstpower(&mut self) -> _BURSTPOWERW {
        _BURSTPOWERW { w: self }
    }
    #[doc = "Bit 14 - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[inline]
    pub fn srcburstwrap(&mut self) -> _SRCBURSTWRAPW {
        _SRCBURSTWRAPW { w: self }
    }
    #[doc = "Bit 15 - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[inline]
    pub fn dstburstwrap(&mut self) -> _DSTBURSTWRAPW {
        _DSTBURSTWRAPW { w: self }
    }
    #[doc = "Bits 16:18 - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
    #[inline]
    pub fn chpriority(&mut self) -> _CHPRIORITYW {
        _CHPRIORITYW { w: self }
    }
}
