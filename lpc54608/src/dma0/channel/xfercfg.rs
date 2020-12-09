#[doc = "Reader of register XFERCFG"]
pub type R = crate::R<u32, super::XFERCFG>;
#[doc = "Writer for register XFERCFG"]
pub type W = crate::W<u32, super::XFERCFG>;
#[doc = "Register XFERCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::XFERCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGVALID_A {
    #[doc = "0: Not valid. The channel descriptor is not considered valid until validated by an associated SETVALID0 setting."]
    NOT_VALID = 0,
    #[doc = "1: Valid. The current channel descriptor is considered valid."]
    VALID = 1,
}
impl From<CFGVALID_A> for bool {
    #[inline(always)]
    fn from(variant: CFGVALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CFGVALID`"]
pub type CFGVALID_R = crate::R<bool, CFGVALID_A>;
impl CFGVALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGVALID_A {
        match self.bits {
            false => CFGVALID_A::NOT_VALID,
            true => CFGVALID_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == CFGVALID_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == CFGVALID_A::VALID
    }
}
#[doc = "Write proxy for field `CFGVALID`"]
pub struct CFGVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGVALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGVALID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not valid. The channel descriptor is not considered valid until validated by an associated SETVALID0 setting."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(CFGVALID_A::NOT_VALID)
    }
    #[doc = "Valid. The current channel descriptor is considered valid."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(CFGVALID_A::VALID)
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
#[doc = "Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RELOAD_A {
    #[doc = "0: Disabled. Do not reload the channels' control structure when the current descriptor is exhausted."]
    DISABLED = 0,
    #[doc = "1: Enabled. Reload the channels' control structure when the current descriptor is exhausted."]
    ENABLED = 1,
}
impl From<RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RELOAD`"]
pub type RELOAD_R = crate::R<bool, RELOAD_A>;
impl RELOAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_A {
        match self.bits {
            false => RELOAD_A::DISABLED,
            true => RELOAD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RELOAD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RELOAD_A::ENABLED
    }
}
#[doc = "Write proxy for field `RELOAD`"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RELOAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. Do not reload the channels' control structure when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RELOAD_A::DISABLED)
    }
    #[doc = "Enabled. Reload the channels' control structure when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RELOAD_A::ENABLED)
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
#[doc = "Software Trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIG_A {
    #[doc = "0: Not set. When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    NOT_SET = 0,
    #[doc = "1: Set. When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    SET = 1,
}
impl From<SWTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWTRIG`"]
pub type SWTRIG_R = crate::R<bool, SWTRIG_A>;
impl SWTRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWTRIG_A {
        match self.bits {
            false => SWTRIG_A::NOT_SET,
            true => SWTRIG_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SET`"]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == SWTRIG_A::NOT_SET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SWTRIG_A::SET
    }
}
#[doc = "Write proxy for field `SWTRIG`"]
pub struct SWTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWTRIG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not set. When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    #[inline(always)]
    pub fn not_set(self) -> &'a mut W {
        self.variant(SWTRIG_A::NOT_SET)
    }
    #[doc = "Set. When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SWTRIG_A::SET)
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
#[doc = "Clear Trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRTRIG_A {
    #[doc = "0: Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    NOT_CLEARED = 0,
    #[doc = "1: Cleared. The trigger is cleared when this descriptor is exhausted"]
    CLEARED = 1,
}
impl From<CLRTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: CLRTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLRTRIG`"]
pub type CLRTRIG_R = crate::R<bool, CLRTRIG_A>;
impl CLRTRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRTRIG_A {
        match self.bits {
            false => CLRTRIG_A::NOT_CLEARED,
            true => CLRTRIG_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_CLEARED`"]
    #[inline(always)]
    pub fn is_not_cleared(&self) -> bool {
        *self == CLRTRIG_A::NOT_CLEARED
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLRTRIG_A::CLEARED
    }
}
#[doc = "Write proxy for field `CLRTRIG`"]
pub struct CLRTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRTRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRTRIG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    #[inline(always)]
    pub fn not_cleared(self) -> &'a mut W {
        self.variant(CLRTRIG_A::NOT_CLEARED)
    }
    #[doc = "Cleared. The trigger is cleared when this descriptor is exhausted"]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLRTRIG_A::CLEARED)
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
#[doc = "Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETINTA_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Set. The INTA flag for this channel will be set when the current descriptor is exhausted."]
    SET = 1,
}
impl From<SETINTA_A> for bool {
    #[inline(always)]
    fn from(variant: SETINTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SETINTA`"]
pub type SETINTA_R = crate::R<bool, SETINTA_A>;
impl SETINTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SETINTA_A {
        match self.bits {
            false => SETINTA_A::NO_EFFECT,
            true => SETINTA_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SETINTA_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SETINTA_A::SET
    }
}
#[doc = "Write proxy for field `SETINTA`"]
pub struct SETINTA_W<'a> {
    w: &'a mut W,
}
impl<'a> SETINTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETINTA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETINTA_A::NO_EFFECT)
    }
    #[doc = "Set. The INTA flag for this channel will be set when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SETINTA_A::SET)
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
#[doc = "Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETINTB_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Set. The INTB flag for this channel will be set when the current descriptor is exhausted."]
    SET = 1,
}
impl From<SETINTB_A> for bool {
    #[inline(always)]
    fn from(variant: SETINTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SETINTB`"]
pub type SETINTB_R = crate::R<bool, SETINTB_A>;
impl SETINTB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SETINTB_A {
        match self.bits {
            false => SETINTB_A::NO_EFFECT,
            true => SETINTB_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SETINTB_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SETINTB_A::SET
    }
}
#[doc = "Write proxy for field `SETINTB`"]
pub struct SETINTB_W<'a> {
    w: &'a mut W,
}
impl<'a> SETINTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETINTB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETINTB_A::NO_EFFECT)
    }
    #[doc = "Set. The INTB flag for this channel will be set when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SETINTB_A::SET)
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
#[doc = "Transfer width used for this DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WIDTH_A {
    #[doc = "0: 8-bit. 8-bit transfers are performed (8-bit source reads and destination writes)."]
    BIT_8 = 0,
    #[doc = "1: 16-bit. 6-bit transfers are performed (16-bit source reads and destination writes)."]
    BIT_16 = 1,
    #[doc = "2: 32-bit. 32-bit transfers are performed (32-bit source reads and destination writes)."]
    BIT_32 = 2,
}
impl From<WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WIDTH`"]
pub type WIDTH_R = crate::R<u8, WIDTH_A>;
impl WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WIDTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WIDTH_A::BIT_8),
            1 => Val(WIDTH_A::BIT_16),
            2 => Val(WIDTH_A::BIT_32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIT_8`"]
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        *self == WIDTH_A::BIT_8
    }
    #[doc = "Checks if the value of the field is `BIT_16`"]
    #[inline(always)]
    pub fn is_bit_16(&self) -> bool {
        *self == WIDTH_A::BIT_16
    }
    #[doc = "Checks if the value of the field is `BIT_32`"]
    #[inline(always)]
    pub fn is_bit_32(&self) -> bool {
        *self == WIDTH_A::BIT_32
    }
}
#[doc = "Write proxy for field `WIDTH`"]
pub struct WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8-bit. 8-bit transfers are performed (8-bit source reads and destination writes)."]
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut W {
        self.variant(WIDTH_A::BIT_8)
    }
    #[doc = "16-bit. 6-bit transfers are performed (16-bit source reads and destination writes)."]
    #[inline(always)]
    pub fn bit_16(self) -> &'a mut W {
        self.variant(WIDTH_A::BIT_16)
    }
    #[doc = "32-bit. 32-bit transfers are performed (32-bit source reads and destination writes)."]
    #[inline(always)]
    pub fn bit_32(self) -> &'a mut W {
        self.variant(WIDTH_A::BIT_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Determines whether the source address is incremented for each DMA transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRCINC_A {
    #[doc = "0: No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    NO_INCREMENT = 0,
    #[doc = "1: 1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    WIDTH_X_1 = 1,
    #[doc = "2: 2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2 = 2,
    #[doc = "3: 4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4 = 3,
}
impl From<SRCINC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCINC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRCINC`"]
pub type SRCINC_R = crate::R<u8, SRCINC_A>;
impl SRCINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCINC_A {
        match self.bits {
            0 => SRCINC_A::NO_INCREMENT,
            1 => SRCINC_A::WIDTH_X_1,
            2 => SRCINC_A::WIDTH_X_2,
            3 => SRCINC_A::WIDTH_X_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_INCREMENT`"]
    #[inline(always)]
    pub fn is_no_increment(&self) -> bool {
        *self == SRCINC_A::NO_INCREMENT
    }
    #[doc = "Checks if the value of the field is `WIDTH_X_1`"]
    #[inline(always)]
    pub fn is_width_x_1(&self) -> bool {
        *self == SRCINC_A::WIDTH_X_1
    }
    #[doc = "Checks if the value of the field is `WIDTH_X_2`"]
    #[inline(always)]
    pub fn is_width_x_2(&self) -> bool {
        *self == SRCINC_A::WIDTH_X_2
    }
    #[doc = "Checks if the value of the field is `WIDTH_X_4`"]
    #[inline(always)]
    pub fn is_width_x_4(&self) -> bool {
        *self == SRCINC_A::WIDTH_X_4
    }
}
#[doc = "Write proxy for field `SRCINC`"]
pub struct SRCINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRCINC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    #[inline(always)]
    pub fn no_increment(self) -> &'a mut W {
        self.variant(SRCINC_A::NO_INCREMENT)
    }
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    #[inline(always)]
    pub fn width_x_1(self) -> &'a mut W {
        self.variant(SRCINC_A::WIDTH_X_1)
    }
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn width_x_2(self) -> &'a mut W {
        self.variant(SRCINC_A::WIDTH_X_2)
    }
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn width_x_4(self) -> &'a mut W {
        self.variant(SRCINC_A::WIDTH_X_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Determines whether the destination address is incremented for each DMA transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSTINC_A {
    #[doc = "0: No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    NO_INCREMENT = 0,
    #[doc = "1: 1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    WIDTH_X_1 = 1,
    #[doc = "2: 2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2 = 2,
    #[doc = "3: 4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4 = 3,
}
impl From<DSTINC_A> for u8 {
    #[inline(always)]
    fn from(variant: DSTINC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSTINC`"]
pub type DSTINC_R = crate::R<u8, DSTINC_A>;
impl DSTINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSTINC_A {
        match self.bits {
            0 => DSTINC_A::NO_INCREMENT,
            1 => DSTINC_A::WIDTH_X_1,
            2 => DSTINC_A::WIDTH_X_2,
            3 => DSTINC_A::WIDTH_X_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_INCREMENT`"]
    #[inline(always)]
    pub fn is_no_increment(&self) -> bool {
        *self == DSTINC_A::NO_INCREMENT
    }
    #[doc = "Checks if the value of the field is `WIDTH_X_1`"]
    #[inline(always)]
    pub fn is_width_x_1(&self) -> bool {
        *self == DSTINC_A::WIDTH_X_1
    }
    #[doc = "Checks if the value of the field is `WIDTH_X_2`"]
    #[inline(always)]
    pub fn is_width_x_2(&self) -> bool {
        *self == DSTINC_A::WIDTH_X_2
    }
    #[doc = "Checks if the value of the field is `WIDTH_X_4`"]
    #[inline(always)]
    pub fn is_width_x_4(&self) -> bool {
        *self == DSTINC_A::WIDTH_X_4
    }
}
#[doc = "Write proxy for field `DSTINC`"]
pub struct DSTINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSTINC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    #[inline(always)]
    pub fn no_increment(self) -> &'a mut W {
        self.variant(DSTINC_A::NO_INCREMENT)
    }
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    #[inline(always)]
    pub fn width_x_1(self) -> &'a mut W {
        self.variant(DSTINC_A::WIDTH_X_1)
    }
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn width_x_2(self) -> &'a mut W {
        self.variant(DSTINC_A::WIDTH_X_2)
    }
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn width_x_4(self) -> &'a mut W {
        self.variant(DSTINC_A::WIDTH_X_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `XFERCOUNT`"]
pub type XFERCOUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XFERCOUNT`"]
pub struct XFERCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[inline(always)]
    pub fn cfgvalid(&self) -> CFGVALID_R {
        CFGVALID_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software Trigger."]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear Trigger."]
    #[inline(always)]
    pub fn clrtrig(&self) -> CLRTRIG_R {
        CLRTRIG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn setinta(&self) -> SETINTA_R {
        SETINTA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn setintb(&self) -> SETINTB_R {
        SETINTB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Transfer width used for this DMA channel."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Determines whether the source address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn srcinc(&self) -> SRCINC_R {
        SRCINC_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Determines whether the destination address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn dstinc(&self) -> DSTINC_R {
        DSTINC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:25 - Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
    #[inline(always)]
    pub fn xfercount(&self) -> XFERCOUNT_R {
        XFERCOUNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[inline(always)]
    pub fn cfgvalid(&mut self) -> CFGVALID_W {
        CFGVALID_W { w: self }
    }
    #[doc = "Bit 1 - Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
    #[doc = "Bit 2 - Software Trigger."]
    #[inline(always)]
    pub fn swtrig(&mut self) -> SWTRIG_W {
        SWTRIG_W { w: self }
    }
    #[doc = "Bit 3 - Clear Trigger."]
    #[inline(always)]
    pub fn clrtrig(&mut self) -> CLRTRIG_W {
        CLRTRIG_W { w: self }
    }
    #[doc = "Bit 4 - Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn setinta(&mut self) -> SETINTA_W {
        SETINTA_W { w: self }
    }
    #[doc = "Bit 5 - Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn setintb(&mut self) -> SETINTB_W {
        SETINTB_W { w: self }
    }
    #[doc = "Bits 8:9 - Transfer width used for this DMA channel."]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W {
        WIDTH_W { w: self }
    }
    #[doc = "Bits 12:13 - Determines whether the source address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn srcinc(&mut self) -> SRCINC_W {
        SRCINC_W { w: self }
    }
    #[doc = "Bits 14:15 - Determines whether the destination address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn dstinc(&mut self) -> DSTINC_W {
        DSTINC_W { w: self }
    }
    #[doc = "Bits 16:25 - Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
    #[inline(always)]
    pub fn xfercount(&mut self) -> XFERCOUNT_W {
        XFERCOUNT_W { w: self }
    }
}
