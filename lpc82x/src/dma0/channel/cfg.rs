#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CFG_SPEC>> for R {
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
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
impl core::convert::From<crate::W<CFG_SPEC>> for W {
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPHREQEN_A {
    #[doc = "0: Disabled. Peripheral DMA requests are disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. Peripheral DMA requests are enabled."]
    ENABLED = 1,
}
impl From<PERIPHREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPHREQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIPHREQEN` reader - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
pub struct PERIPHREQEN_R(crate::FieldReader<bool, PERIPHREQEN_A>);
impl PERIPHREQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERIPHREQEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPHREQEN_A {
        match self.bits {
            false => PERIPHREQEN_A::DISABLED,
            true => PERIPHREQEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PERIPHREQEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
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
#[doc = "Field `PERIPHREQEN` writer - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
pub struct PERIPHREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPHREQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIPHREQEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. Peripheral DMA requests are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PERIPHREQEN_A::DISABLED)
    }
    #[doc = "Enabled. Peripheral DMA requests are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PERIPHREQEN_A::ENABLED)
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
#[doc = "Hardware Triggering Enable for this channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWTRIGEN_A {
    #[doc = "0: Disabled. Hardware triggering is not used."]
    DISABLED = 0,
    #[doc = "1: Enabled. Use hardware triggering."]
    ENABLED = 1,
}
impl From<HWTRIGEN_A> for bool {
    #[inline(always)]
    fn from(variant: HWTRIGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWTRIGEN` reader - Hardware Triggering Enable for this channel."]
pub struct HWTRIGEN_R(crate::FieldReader<bool, HWTRIGEN_A>);
impl HWTRIGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWTRIGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWTRIGEN_A {
        match self.bits {
            false => HWTRIGEN_A::DISABLED,
            true => HWTRIGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HWTRIGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
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
#[doc = "Field `HWTRIGEN` writer - Hardware Triggering Enable for this channel."]
pub struct HWTRIGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HWTRIGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HWTRIGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. Hardware triggering is not used."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HWTRIGEN_A::DISABLED)
    }
    #[doc = "Enabled. Use hardware triggering."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HWTRIGEN_A::ENABLED)
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
#[doc = "Trigger Polarity. Selects the polarity of a hardware trigger for this channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGPOL_A {
    #[doc = "0: Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    ACTIVE_LOW_FALLING = 0,
    #[doc = "1: Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    ACTIVE_HIGH_RISING = 1,
}
impl From<TRIGPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGPOL` reader - Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
pub struct TRIGPOL_R(crate::FieldReader<bool, TRIGPOL_A>);
impl TRIGPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRIGPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGPOL_A {
        match self.bits {
            false => TRIGPOL_A::ACTIVE_LOW_FALLING,
            true => TRIGPOL_A::ACTIVE_HIGH_RISING,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW_FALLING`"]
    #[inline(always)]
    pub fn is_active_low_falling(&self) -> bool {
        **self == TRIGPOL_A::ACTIVE_LOW_FALLING
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH_RISING`"]
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
#[doc = "Field `TRIGPOL` writer - Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
pub struct TRIGPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    #[inline(always)]
    pub fn active_low_falling(self) -> &'a mut W {
        self.variant(TRIGPOL_A::ACTIVE_LOW_FALLING)
    }
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    #[inline(always)]
    pub fn active_high_rising(self) -> &'a mut W {
        self.variant(TRIGPOL_A::ACTIVE_HIGH_RISING)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Trigger Type. Selects hardware trigger as edge triggered or level triggered.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGTYPE_A {
    #[doc = "0: Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    EDGE = 0,
    #[doc = "1: Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed."]
    LEVEL = 1,
}
impl From<TRIGTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGTYPE` reader - Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
pub struct TRIGTYPE_R(crate::FieldReader<bool, TRIGTYPE_A>);
impl TRIGTYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRIGTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGTYPE_A {
        match self.bits {
            false => TRIGTYPE_A::EDGE,
            true => TRIGTYPE_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        **self == TRIGTYPE_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
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
#[doc = "Field `TRIGTYPE` writer - Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
pub struct TRIGTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGTYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(TRIGTYPE_A::EDGE)
    }
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed."]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(TRIGTYPE_A::LEVEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Trigger Burst. Selects whether hardware triggers cause a single or burst transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGBURST_A {
    #[doc = "0: Single transfer. Hardware trigger causes a single transfer."]
    SINGLE = 0,
    #[doc = "1: Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    BURST = 1,
}
impl From<TRIGBURST_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGBURST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGBURST` reader - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
pub struct TRIGBURST_R(crate::FieldReader<bool, TRIGBURST_A>);
impl TRIGBURST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRIGBURST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGBURST_A {
        match self.bits {
            false => TRIGBURST_A::SINGLE,
            true => TRIGBURST_A::BURST,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == TRIGBURST_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `BURST`"]
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
#[doc = "Field `TRIGBURST` writer - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
pub struct TRIGBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGBURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGBURST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(TRIGBURST_A::SINGLE)
    }
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(TRIGBURST_A::BURST)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `BURSTPOWER` reader - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
pub struct BURSTPOWER_R(crate::FieldReader<u8, u8>);
impl BURSTPOWER_R {
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
#[doc = "Field `BURSTPOWER` writer - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
pub struct BURSTPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTPOWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCBURSTWRAP_A {
    #[doc = "0: Disabled. Source burst wrapping is not enabled for this DMA channel."]
    DISABLED = 0,
    #[doc = "1: Enabled. Source burst wrapping is enabled for this DMA channel."]
    ENABLED = 1,
}
impl From<SRCBURSTWRAP_A> for bool {
    #[inline(always)]
    fn from(variant: SRCBURSTWRAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCBURSTWRAP` reader - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
pub struct SRCBURSTWRAP_R(crate::FieldReader<bool, SRCBURSTWRAP_A>);
impl SRCBURSTWRAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRCBURSTWRAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCBURSTWRAP_A {
        match self.bits {
            false => SRCBURSTWRAP_A::DISABLED,
            true => SRCBURSTWRAP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SRCBURSTWRAP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
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
#[doc = "Field `SRCBURSTWRAP` writer - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
pub struct SRCBURSTWRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCBURSTWRAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRCBURSTWRAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. Source burst wrapping is not enabled for this DMA channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRCBURSTWRAP_A::DISABLED)
    }
    #[doc = "Enabled. Source burst wrapping is enabled for this DMA channel."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRCBURSTWRAP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSTBURSTWRAP_A {
    #[doc = "0: Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    DISABLED = 0,
    #[doc = "1: Enabled. Destination burst wrapping is enabled for this DMA channel."]
    ENABLED = 1,
}
impl From<DSTBURSTWRAP_A> for bool {
    #[inline(always)]
    fn from(variant: DSTBURSTWRAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSTBURSTWRAP` reader - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
pub struct DSTBURSTWRAP_R(crate::FieldReader<bool, DSTBURSTWRAP_A>);
impl DSTBURSTWRAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSTBURSTWRAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSTBURSTWRAP_A {
        match self.bits {
            false => DSTBURSTWRAP_A::DISABLED,
            true => DSTBURSTWRAP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DSTBURSTWRAP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
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
#[doc = "Field `DSTBURSTWRAP` writer - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
pub struct DSTBURSTWRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTBURSTWRAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSTBURSTWRAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DSTBURSTWRAP_A::DISABLED)
    }
    #[doc = "Enabled. Destination burst wrapping is enabled for this DMA channel."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DSTBURSTWRAP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CHPRIORITY` reader - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
pub struct CHPRIORITY_R(crate::FieldReader<u8, u8>);
impl CHPRIORITY_R {
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
#[doc = "Field `CHPRIORITY` writer - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
pub struct CHPRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[inline(always)]
    pub fn periphreqen(&self) -> PERIPHREQEN_R {
        PERIPHREQEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hardware Triggering Enable for this channel."]
    #[inline(always)]
    pub fn hwtrigen(&self) -> HWTRIGEN_R {
        HWTRIGEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[inline(always)]
    pub fn trigtype(&self) -> TRIGTYPE_R {
        TRIGTYPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[inline(always)]
    pub fn trigburst(&self) -> TRIGBURST_R {
        TRIGBURST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
    #[inline(always)]
    pub fn burstpower(&self) -> BURSTPOWER_R {
        BURSTPOWER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[inline(always)]
    pub fn srcburstwrap(&self) -> SRCBURSTWRAP_R {
        SRCBURSTWRAP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[inline(always)]
    pub fn dstburstwrap(&self) -> DSTBURSTWRAP_R {
        DSTBURSTWRAP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
    #[inline(always)]
    pub fn chpriority(&self) -> CHPRIORITY_R {
        CHPRIORITY_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[inline(always)]
    pub fn periphreqen(&mut self) -> PERIPHREQEN_W {
        PERIPHREQEN_W { w: self }
    }
    #[doc = "Bit 1 - Hardware Triggering Enable for this channel."]
    #[inline(always)]
    pub fn hwtrigen(&mut self) -> HWTRIGEN_W {
        HWTRIGEN_W { w: self }
    }
    #[doc = "Bit 4 - Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W {
        TRIGPOL_W { w: self }
    }
    #[doc = "Bit 5 - Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[inline(always)]
    pub fn trigtype(&mut self) -> TRIGTYPE_W {
        TRIGTYPE_W { w: self }
    }
    #[doc = "Bit 6 - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[inline(always)]
    pub fn trigburst(&mut self) -> TRIGBURST_W {
        TRIGBURST_W { w: self }
    }
    #[doc = "Bits 8:11 - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
    #[inline(always)]
    pub fn burstpower(&mut self) -> BURSTPOWER_W {
        BURSTPOWER_W { w: self }
    }
    #[doc = "Bit 14 - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[inline(always)]
    pub fn srcburstwrap(&mut self) -> SRCBURSTWRAP_W {
        SRCBURSTWRAP_W { w: self }
    }
    #[doc = "Bit 15 - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[inline(always)]
    pub fn dstburstwrap(&mut self) -> DSTBURSTWRAP_W {
        DSTBURSTWRAP_W { w: self }
    }
    #[doc = "Bits 16:18 - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
    #[inline(always)]
    pub fn chpriority(&mut self) -> CHPRIORITY_W {
        CHPRIORITY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register for DMA channel .\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
