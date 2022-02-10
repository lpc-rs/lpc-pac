///Register `CFG` reader
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFG` writer
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
///Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPHREQEN_A {
    ///0: Disabled. Peripheral DMA requests are disabled.
    DISABLED = 0,
    ///1: Enabled. Peripheral DMA requests are enabled.
    ENABLED = 1,
}
impl From<PERIPHREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPHREQEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PERIPHREQEN` reader - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller.
pub struct PERIPHREQEN_R(crate::FieldReader<bool, PERIPHREQEN_A>);
impl PERIPHREQEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERIPHREQEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PERIPHREQEN_A {
        match self.bits {
            false => PERIPHREQEN_A::DISABLED,
            true => PERIPHREQEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PERIPHREQEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PERIPHREQEN_A::ENABLED
    }
}
impl core::ops::Deref for PERIPHREQEN_R {
    type Target = crate::FieldReader<bool, PERIPHREQEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PERIPHREQEN` writer - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller.
pub struct PERIPHREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPHREQEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PERIPHREQEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. Peripheral DMA requests are disabled.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PERIPHREQEN_A::DISABLED)
    }
    ///Enabled. Peripheral DMA requests are enabled.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PERIPHREQEN_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Hardware Triggering Enable for this channel.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWTRIGEN_A {
    ///0: Disabled. Hardware triggering is not used.
    DISABLED = 0,
    ///1: Enabled. Use hardware triggering.
    ENABLED = 1,
}
impl From<HWTRIGEN_A> for bool {
    #[inline(always)]
    fn from(variant: HWTRIGEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HWTRIGEN` reader - Hardware Triggering Enable for this channel.
pub struct HWTRIGEN_R(crate::FieldReader<bool, HWTRIGEN_A>);
impl HWTRIGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HWTRIGEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HWTRIGEN_A {
        match self.bits {
            false => HWTRIGEN_A::DISABLED,
            true => HWTRIGEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HWTRIGEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HWTRIGEN_A::ENABLED
    }
}
impl core::ops::Deref for HWTRIGEN_R {
    type Target = crate::FieldReader<bool, HWTRIGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HWTRIGEN` writer - Hardware Triggering Enable for this channel.
pub struct HWTRIGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HWTRIGEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HWTRIGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. Hardware triggering is not used.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HWTRIGEN_A::DISABLED)
    }
    ///Enabled. Use hardware triggering.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HWTRIGEN_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Trigger Polarity. Selects the polarity of a hardware trigger for this channel.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGPOL_A {
    ///0: Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE.
    ACTIVE_LOW_FALLING = 0,
    ///1: Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE.
    ACTIVE_HIGH_RISING = 1,
}
impl From<TRIGPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRIGPOL` reader - Trigger Polarity. Selects the polarity of a hardware trigger for this channel.
pub struct TRIGPOL_R(crate::FieldReader<bool, TRIGPOL_A>);
impl TRIGPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGPOL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TRIGPOL_A {
        match self.bits {
            false => TRIGPOL_A::ACTIVE_LOW_FALLING,
            true => TRIGPOL_A::ACTIVE_HIGH_RISING,
        }
    }
    ///Checks if the value of the field is `ACTIVE_LOW_FALLING`
    #[inline(always)]
    pub fn is_active_low_falling(&self) -> bool {
        **self == TRIGPOL_A::ACTIVE_LOW_FALLING
    }
    ///Checks if the value of the field is `ACTIVE_HIGH_RISING`
    #[inline(always)]
    pub fn is_active_high_rising(&self) -> bool {
        **self == TRIGPOL_A::ACTIVE_HIGH_RISING
    }
}
impl core::ops::Deref for TRIGPOL_R {
    type Target = crate::FieldReader<bool, TRIGPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRIGPOL` writer - Trigger Polarity. Selects the polarity of a hardware trigger for this channel.
pub struct TRIGPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TRIGPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE.
    #[inline(always)]
    pub fn active_low_falling(self) -> &'a mut W {
        self.variant(TRIGPOL_A::ACTIVE_LOW_FALLING)
    }
    ///Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE.
    #[inline(always)]
    pub fn active_high_rising(self) -> &'a mut W {
        self.variant(TRIGPOL_A::ACTIVE_HIGH_RISING)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Trigger Type. Selects hardware trigger as edge triggered or level triggered.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGTYPE_A {
    ///0: Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger.
    EDGE = 0,
    ///1: Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed.
    LEVEL = 1,
}
impl From<TRIGTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGTYPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRIGTYPE` reader - Trigger Type. Selects hardware trigger as edge triggered or level triggered.
pub struct TRIGTYPE_R(crate::FieldReader<bool, TRIGTYPE_A>);
impl TRIGTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGTYPE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TRIGTYPE_A {
        match self.bits {
            false => TRIGTYPE_A::EDGE,
            true => TRIGTYPE_A::LEVEL,
        }
    }
    ///Checks if the value of the field is `EDGE`
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        **self == TRIGTYPE_A::EDGE
    }
    ///Checks if the value of the field is `LEVEL`
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        **self == TRIGTYPE_A::LEVEL
    }
}
impl core::ops::Deref for TRIGTYPE_R {
    type Target = crate::FieldReader<bool, TRIGTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRIGTYPE` writer - Trigger Type. Selects hardware trigger as edge triggered or level triggered.
pub struct TRIGTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGTYPE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TRIGTYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger.
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(TRIGTYPE_A::EDGE)
    }
    ///Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed.
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(TRIGTYPE_A::LEVEL)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Trigger Burst. Selects whether hardware triggers cause a single or burst transfer.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGBURST_A {
    ///0: Single transfer. Hardware trigger causes a single transfer.
    SINGLE = 0,
    ///1: Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete.
    BURST = 1,
}
impl From<TRIGBURST_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGBURST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRIGBURST` reader - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer.
pub struct TRIGBURST_R(crate::FieldReader<bool, TRIGBURST_A>);
impl TRIGBURST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGBURST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TRIGBURST_A {
        match self.bits {
            false => TRIGBURST_A::SINGLE,
            true => TRIGBURST_A::BURST,
        }
    }
    ///Checks if the value of the field is `SINGLE`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == TRIGBURST_A::SINGLE
    }
    ///Checks if the value of the field is `BURST`
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        **self == TRIGBURST_A::BURST
    }
}
impl core::ops::Deref for TRIGBURST_R {
    type Target = crate::FieldReader<bool, TRIGBURST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRIGBURST` writer - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer.
pub struct TRIGBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGBURST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TRIGBURST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Single transfer. Hardware trigger causes a single transfer.
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(TRIGBURST_A::SINGLE)
    }
    ///Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete.
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(TRIGBURST_A::BURST)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Field `BURSTPOWER` reader - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size.
pub struct BURSTPOWER_R(crate::FieldReader<u8, u8>);
impl BURSTPOWER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BURSTPOWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BURSTPOWER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BURSTPOWER` writer - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size.
pub struct BURSTPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTPOWER_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCBURSTWRAP_A {
    ///0: Disabled. Source burst wrapping is not enabled for this DMA channel.
    DISABLED = 0,
    ///1: Enabled. Source burst wrapping is enabled for this DMA channel.
    ENABLED = 1,
}
impl From<SRCBURSTWRAP_A> for bool {
    #[inline(always)]
    fn from(variant: SRCBURSTWRAP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SRCBURSTWRAP` reader - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst.
pub struct SRCBURSTWRAP_R(crate::FieldReader<bool, SRCBURSTWRAP_A>);
impl SRCBURSTWRAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRCBURSTWRAP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SRCBURSTWRAP_A {
        match self.bits {
            false => SRCBURSTWRAP_A::DISABLED,
            true => SRCBURSTWRAP_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SRCBURSTWRAP_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SRCBURSTWRAP_A::ENABLED
    }
}
impl core::ops::Deref for SRCBURSTWRAP_R {
    type Target = crate::FieldReader<bool, SRCBURSTWRAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SRCBURSTWRAP` writer - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst.
pub struct SRCBURSTWRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCBURSTWRAP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SRCBURSTWRAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. Source burst wrapping is not enabled for this DMA channel.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRCBURSTWRAP_A::DISABLED)
    }
    ///Enabled. Source burst wrapping is enabled for this DMA channel.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRCBURSTWRAP_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSTBURSTWRAP_A {
    ///0: Disabled. Destination burst wrapping is not enabled for this DMA channel.
    DISABLED = 0,
    ///1: Enabled. Destination burst wrapping is enabled for this DMA channel.
    ENABLED = 1,
}
impl From<DSTBURSTWRAP_A> for bool {
    #[inline(always)]
    fn from(variant: DSTBURSTWRAP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSTBURSTWRAP` reader - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst.
pub struct DSTBURSTWRAP_R(crate::FieldReader<bool, DSTBURSTWRAP_A>);
impl DSTBURSTWRAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSTBURSTWRAP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DSTBURSTWRAP_A {
        match self.bits {
            false => DSTBURSTWRAP_A::DISABLED,
            true => DSTBURSTWRAP_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DSTBURSTWRAP_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DSTBURSTWRAP_A::ENABLED
    }
}
impl core::ops::Deref for DSTBURSTWRAP_R {
    type Target = crate::FieldReader<bool, DSTBURSTWRAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DSTBURSTWRAP` writer - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst.
pub struct DSTBURSTWRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTBURSTWRAP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DSTBURSTWRAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. Destination burst wrapping is not enabled for this DMA channel.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DSTBURSTWRAP_A::DISABLED)
    }
    ///Enabled. Destination burst wrapping is enabled for this DMA channel.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DSTBURSTWRAP_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///Field `CHPRIORITY` reader - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority.
pub struct CHPRIORITY_R(crate::FieldReader<u8, u8>);
impl CHPRIORITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHPRIORITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHPRIORITY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CHPRIORITY` writer - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority.
pub struct CHPRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPRIORITY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller.
    #[inline(always)]
    pub fn periphreqen(&self) -> PERIPHREQEN_R {
        PERIPHREQEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Hardware Triggering Enable for this channel.
    #[inline(always)]
    pub fn hwtrigen(&self) -> HWTRIGEN_R {
        HWTRIGEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 4 - Trigger Polarity. Selects the polarity of a hardware trigger for this channel.
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Trigger Type. Selects hardware trigger as edge triggered or level triggered.
    #[inline(always)]
    pub fn trigtype(&self) -> TRIGTYPE_R {
        TRIGTYPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer.
    #[inline(always)]
    pub fn trigburst(&self) -> TRIGBURST_R {
        TRIGBURST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bits 8:11 - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size.
    #[inline(always)]
    pub fn burstpower(&self) -> BURSTPOWER_R {
        BURSTPOWER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 14 - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst.
    #[inline(always)]
    pub fn srcburstwrap(&self) -> SRCBURSTWRAP_R {
        SRCBURSTWRAP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst.
    #[inline(always)]
    pub fn dstburstwrap(&self) -> DSTBURSTWRAP_R {
        DSTBURSTWRAP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 16:18 - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority.
    #[inline(always)]
    pub fn chpriority(&self) -> CHPRIORITY_R {
        CHPRIORITY_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    ///Bit 0 - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller.
    #[inline(always)]
    pub fn periphreqen(&mut self) -> PERIPHREQEN_W {
        PERIPHREQEN_W { w: self }
    }
    ///Bit 1 - Hardware Triggering Enable for this channel.
    #[inline(always)]
    pub fn hwtrigen(&mut self) -> HWTRIGEN_W {
        HWTRIGEN_W { w: self }
    }
    ///Bit 4 - Trigger Polarity. Selects the polarity of a hardware trigger for this channel.
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W {
        TRIGPOL_W { w: self }
    }
    ///Bit 5 - Trigger Type. Selects hardware trigger as edge triggered or level triggered.
    #[inline(always)]
    pub fn trigtype(&mut self) -> TRIGTYPE_W {
        TRIGTYPE_W { w: self }
    }
    ///Bit 6 - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer.
    #[inline(always)]
    pub fn trigburst(&mut self) -> TRIGBURST_W {
        TRIGBURST_W { w: self }
    }
    ///Bits 8:11 - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size.
    #[inline(always)]
    pub fn burstpower(&mut self) -> BURSTPOWER_W {
        BURSTPOWER_W { w: self }
    }
    ///Bit 14 - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst.
    #[inline(always)]
    pub fn srcburstwrap(&mut self) -> SRCBURSTWRAP_W {
        SRCBURSTWRAP_W { w: self }
    }
    ///Bit 15 - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst.
    #[inline(always)]
    pub fn dstburstwrap(&mut self) -> DSTBURSTWRAP_W {
        DSTBURSTWRAP_W { w: self }
    }
    ///Bits 16:18 - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority.
    #[inline(always)]
    pub fn chpriority(&mut self) -> CHPRIORITY_W {
        CHPRIORITY_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Configuration register for DMA channel .
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfg::R](R) reader structure
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfg::W](W) writer structure
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
///`reset()` method sets CFG to value 0
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
