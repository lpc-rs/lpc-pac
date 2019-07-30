#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
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
impl crate::ToBits<bool> for PERIPHREQENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PERIPHREQENR::DISABLED => false,
            PERIPHREQENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PERIPHREQEN_R = crate::FR<bool, PERIPHREQENR>;
impl PERIPHREQEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PERIPHREQENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PERIPHREQENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `PERIPHREQEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPHREQENW {
    #[doc = "Disabled. Peripheral DMA requests are disabled."]
    DISABLED,
    #[doc = "Enabled. Peripheral DMA requests are enabled."]
    ENABLED,
}
impl PERIPHREQENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PERIPHREQENW::DISABLED => false,
            PERIPHREQENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PERIPHREQENW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIPHREQENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIPHREQENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Peripheral DMA requests are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PERIPHREQENW::DISABLED)
    }
    #[doc = "Enabled. Peripheral DMA requests are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PERIPHREQENW::ENABLED)
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
#[doc = "Possible values of the field `HWTRIGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWTRIGENR {
    #[doc = "Disabled. Hardware triggering is not used."]
    DISABLED,
    #[doc = "Enabled. Use hardware triggering."]
    ENABLED,
}
impl crate::ToBits<bool> for HWTRIGENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            HWTRIGENR::DISABLED => false,
            HWTRIGENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type HWTRIGEN_R = crate::FR<bool, HWTRIGENR>;
impl HWTRIGEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HWTRIGENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HWTRIGENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `HWTRIGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWTRIGENW {
    #[doc = "Disabled. Hardware triggering is not used."]
    DISABLED,
    #[doc = "Enabled. Use hardware triggering."]
    ENABLED,
}
impl HWTRIGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            HWTRIGENW::DISABLED => false,
            HWTRIGENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _HWTRIGENW<'a> {
    w: &'a mut W,
}
impl<'a> _HWTRIGENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HWTRIGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Hardware triggering is not used."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HWTRIGENW::DISABLED)
    }
    #[doc = "Enabled. Use hardware triggering."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HWTRIGENW::ENABLED)
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
#[doc = "Possible values of the field `TRIGPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGPOLR {
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    ACTIVE_LOW_FALLING,
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    ACTIVE_HIGH_RISING,
}
impl crate::ToBits<bool> for TRIGPOLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TRIGPOLR::ACTIVE_LOW_FALLING => false,
            TRIGPOLR::ACTIVE_HIGH_RISING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRIGPOL_R = crate::FR<bool, TRIGPOLR>;
impl TRIGPOL_R {
    #[doc = "Checks if the value of the field is `ACTIVE_LOW_FALLING`"]
    #[inline(always)]
    pub fn is_active_low_falling(&self) -> bool {
        *self == TRIGPOLR::ACTIVE_LOW_FALLING
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH_RISING`"]
    #[inline(always)]
    pub fn is_active_high_rising(&self) -> bool {
        *self == TRIGPOLR::ACTIVE_HIGH_RISING
    }
}
#[doc = "Values that can be written to the field `TRIGPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGPOLW {
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    ACTIVE_LOW_FALLING,
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    ACTIVE_HIGH_RISING,
}
impl TRIGPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGPOLW::ACTIVE_LOW_FALLING => false,
            TRIGPOLW::ACTIVE_HIGH_RISING => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRIGPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGPOLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    #[inline(always)]
    pub fn active_low_falling(self) -> &'a mut W {
        self.variant(TRIGPOLW::ACTIVE_LOW_FALLING)
    }
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    #[inline(always)]
    pub fn active_high_rising(self) -> &'a mut W {
        self.variant(TRIGPOLW::ACTIVE_HIGH_RISING)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `TRIGTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGTYPER {
    #[doc = "Edge. Hardware trigger is edge triggered."]
    EDGE,
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel."]
    LEVEL,
}
impl crate::ToBits<bool> for TRIGTYPER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TRIGTYPER::EDGE => false,
            TRIGTYPER::LEVEL => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRIGTYPE_R = crate::FR<bool, TRIGTYPER>;
impl TRIGTYPE_R {
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == TRIGTYPER::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == TRIGTYPER::LEVEL
    }
}
#[doc = "Values that can be written to the field `TRIGTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGTYPEW {
    #[doc = "Edge. Hardware trigger is edge triggered."]
    EDGE,
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel."]
    LEVEL,
}
impl TRIGTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGTYPEW::EDGE => false,
            TRIGTYPEW::LEVEL => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRIGTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGTYPEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGTYPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Edge. Hardware trigger is edge triggered."]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(TRIGTYPEW::EDGE)
    }
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel."]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(TRIGTYPEW::LEVEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `TRIGBURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGBURSTR {
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    SINGLE_TRANSFER,
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    BURST_TRANSFER,
}
impl crate::ToBits<bool> for TRIGBURSTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TRIGBURSTR::SINGLE_TRANSFER => false,
            TRIGBURSTR::BURST_TRANSFER => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRIGBURST_R = crate::FR<bool, TRIGBURSTR>;
impl TRIGBURST_R {
    #[doc = "Checks if the value of the field is `SINGLE_TRANSFER`"]
    #[inline(always)]
    pub fn is_single_transfer(&self) -> bool {
        *self == TRIGBURSTR::SINGLE_TRANSFER
    }
    #[doc = "Checks if the value of the field is `BURST_TRANSFER`"]
    #[inline(always)]
    pub fn is_burst_transfer(&self) -> bool {
        *self == TRIGBURSTR::BURST_TRANSFER
    }
}
#[doc = "Values that can be written to the field `TRIGBURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGBURSTW {
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    SINGLE_TRANSFER,
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    BURST_TRANSFER,
}
impl TRIGBURSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGBURSTW::SINGLE_TRANSFER => false,
            TRIGBURSTW::BURST_TRANSFER => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRIGBURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGBURSTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGBURSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    #[inline(always)]
    pub fn single_transfer(self) -> &'a mut W {
        self.variant(TRIGBURSTW::SINGLE_TRANSFER)
    }
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    #[inline(always)]
    pub fn burst_transfer(self) -> &'a mut W {
        self.variant(TRIGBURSTW::BURST_TRANSFER)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type BURSTPOWER_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _BURSTPOWERW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTPOWERW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
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
impl crate::ToBits<bool> for SRCBURSTWRAPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SRCBURSTWRAPR::DISABLED => false,
            SRCBURSTWRAPR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SRCBURSTWRAP_R = crate::FR<bool, SRCBURSTWRAPR>;
impl SRCBURSTWRAP_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRCBURSTWRAPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRCBURSTWRAPR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SRCBURSTWRAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCBURSTWRAPW {
    #[doc = "Disabled. Source burst wrapping is not enabled for this DMA channel."]
    DISABLED,
    #[doc = "Enabled. Source burst wrapping is enabled for this DMA channel."]
    ENABLED,
}
impl SRCBURSTWRAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SRCBURSTWRAPW::DISABLED => false,
            SRCBURSTWRAPW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SRCBURSTWRAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCBURSTWRAPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRCBURSTWRAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Source burst wrapping is not enabled for this DMA channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRCBURSTWRAPW::DISABLED)
    }
    #[doc = "Enabled. Source burst wrapping is enabled for this DMA channel."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRCBURSTWRAPW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
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
impl crate::ToBits<bool> for DSTBURSTWRAPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DSTBURSTWRAPR::DISABLED => false,
            DSTBURSTWRAPR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DSTBURSTWRAP_R = crate::FR<bool, DSTBURSTWRAPR>;
impl DSTBURSTWRAP_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSTBURSTWRAPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSTBURSTWRAPR::ENABLED
    }
}
#[doc = "Values that can be written to the field `DSTBURSTWRAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSTBURSTWRAPW {
    #[doc = "Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    DISABLED,
    #[doc = "Enabled. Destination burst wrapping is enabled for this DMA channel."]
    ENABLED,
}
impl DSTBURSTWRAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DSTBURSTWRAPW::DISABLED => false,
            DSTBURSTWRAPW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DSTBURSTWRAPW<'a> {
    w: &'a mut W,
}
impl<'a> _DSTBURSTWRAPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSTBURSTWRAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DSTBURSTWRAPW::DISABLED)
    }
    #[doc = "Enabled. Destination burst wrapping is enabled for this DMA channel."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DSTBURSTWRAPW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CHPRIORITY_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CHPRIORITYW<'a> {
    w: &'a mut W,
}
impl<'a> _CHPRIORITYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[inline(always)]
    pub fn periphreqen(&self) -> PERIPHREQEN_R {
        PERIPHREQEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hardware Triggering Enable for this channel."]
    #[inline(always)]
    pub fn hwtrigen(&self) -> HWTRIGEN_R {
        HWTRIGEN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[inline(always)]
    pub fn trigtype(&self) -> TRIGTYPE_R {
        TRIGTYPE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[inline(always)]
    pub fn trigburst(&self) -> TRIGBURST_R {
        TRIGBURST_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Burst Power is used in two ways. It always selects the address wrap size when SrcBurstWrap and/or DstBurstWrap is modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). ... 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported."]
    #[inline(always)]
    pub fn burstpower(&self) -> BURSTPOWER_R {
        BURSTPOWER_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Source Burst Wrap. When enabled, the source data address for the DMA is wrapped, meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[inline(always)]
    pub fn srcburstwrap(&self) -> SRCBURSTWRAP_R {
        SRCBURSTWRAP_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Destination Burst Wrap. When enabled, the destination data address for the DMA is wrapped, meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[inline(always)]
    pub fn dstburstwrap(&self) -> DSTBURSTWRAP_R {
        DSTBURSTWRAP_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Priority of this channel when multiple DMA requests are pending. This description reflects a 2-bit priority field providing 4 priority levels. 0x0 = highest priority. 0x3 = lowest priority."]
    #[inline(always)]
    pub fn chpriority(&self) -> CHPRIORITY_R {
        CHPRIORITY_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[inline(always)]
    pub fn periphreqen(&mut self) -> _PERIPHREQENW {
        _PERIPHREQENW { w: self }
    }
    #[doc = "Bit 1 - Hardware Triggering Enable for this channel."]
    #[inline(always)]
    pub fn hwtrigen(&mut self) -> _HWTRIGENW {
        _HWTRIGENW { w: self }
    }
    #[doc = "Bit 4 - Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[inline(always)]
    pub fn trigpol(&mut self) -> _TRIGPOLW {
        _TRIGPOLW { w: self }
    }
    #[doc = "Bit 5 - Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[inline(always)]
    pub fn trigtype(&mut self) -> _TRIGTYPEW {
        _TRIGTYPEW { w: self }
    }
    #[doc = "Bit 6 - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[inline(always)]
    pub fn trigburst(&mut self) -> _TRIGBURSTW {
        _TRIGBURSTW { w: self }
    }
    #[doc = "Bits 8:11 - Burst Power is used in two ways. It always selects the address wrap size when SrcBurstWrap and/or DstBurstWrap is modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). ... 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported."]
    #[inline(always)]
    pub fn burstpower(&mut self) -> _BURSTPOWERW {
        _BURSTPOWERW { w: self }
    }
    #[doc = "Bit 14 - Source Burst Wrap. When enabled, the source data address for the DMA is wrapped, meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[inline(always)]
    pub fn srcburstwrap(&mut self) -> _SRCBURSTWRAPW {
        _SRCBURSTWRAPW { w: self }
    }
    #[doc = "Bit 15 - Destination Burst Wrap. When enabled, the destination data address for the DMA is wrapped, meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[inline(always)]
    pub fn dstburstwrap(&mut self) -> _DSTBURSTWRAPW {
        _DSTBURSTWRAPW { w: self }
    }
    #[doc = "Bits 16:17 - Priority of this channel when multiple DMA requests are pending. This description reflects a 2-bit priority field providing 4 priority levels. 0x0 = highest priority. 0x3 = lowest priority."]
    #[inline(always)]
    pub fn chpriority(&mut self) -> _CHPRIORITYW {
        _CHPRIORITYW { w: self }
    }
}
