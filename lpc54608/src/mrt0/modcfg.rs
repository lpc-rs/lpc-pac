#[doc = "Reader of register MODCFG"]
pub type R = crate::R<u32, super::MODCFG>;
#[doc = "Writer for register MODCFG"]
pub type W = crate::W<u32, super::MODCFG>;
#[doc = "Register MODCFG `reset()`'s with value 0x0173"]
impl crate::ResetValue for super::MODCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0173
    }
}
#[doc = "Reader of field `NOC`"]
pub type NOC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NOC`"]
pub struct NOC_W<'a> {
    w: &'a mut W,
}
impl<'a> NOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `NOB`"]
pub type NOB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NOB`"]
pub struct NOB_W<'a> {
    w: &'a mut W,
}
impl<'a> NOB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Selects the operating mode for the INUSE flags and the IDLE_CH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTITASK_A {
    #[doc = "0: Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    HARDWARE_STATUS_MODE = 0,
    #[doc = "1: Multi-task mode."]
    MULTI_TASK_MODE = 1,
}
impl From<MULTITASK_A> for bool {
    #[inline(always)]
    fn from(variant: MULTITASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MULTITASK`"]
pub type MULTITASK_R = crate::R<bool, MULTITASK_A>;
impl MULTITASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MULTITASK_A {
        match self.bits {
            false => MULTITASK_A::HARDWARE_STATUS_MODE,
            true => MULTITASK_A::MULTI_TASK_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `HARDWARE_STATUS_MODE`"]
    #[inline(always)]
    pub fn is_hardware_status_mode(&self) -> bool {
        *self == MULTITASK_A::HARDWARE_STATUS_MODE
    }
    #[doc = "Checks if the value of the field is `MULTI_TASK_MODE`"]
    #[inline(always)]
    pub fn is_multi_task_mode(&self) -> bool {
        *self == MULTITASK_A::MULTI_TASK_MODE
    }
}
#[doc = "Write proxy for field `MULTITASK`"]
pub struct MULTITASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MULTITASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MULTITASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    #[inline(always)]
    pub fn hardware_status_mode(self) -> &'a mut W {
        self.variant(MULTITASK_A::HARDWARE_STATUS_MODE)
    }
    #[doc = "Multi-task mode."]
    #[inline(always)]
    pub fn multi_task_mode(self) -> &'a mut W {
        self.variant(MULTITASK_A::MULTI_TASK_MODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline(always)]
    pub fn noc(&self) -> NOC_R {
        NOC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
    #[inline(always)]
    pub fn nob(&self) -> NOB_R {
        NOB_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline(always)]
    pub fn multitask(&self) -> MULTITASK_R {
        MULTITASK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline(always)]
    pub fn noc(&mut self) -> NOC_W {
        NOC_W { w: self }
    }
    #[doc = "Bits 4:8 - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
    #[inline(always)]
    pub fn nob(&mut self) -> NOB_W {
        NOB_W { w: self }
    }
    #[doc = "Bit 31 - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline(always)]
    pub fn multitask(&mut self) -> MULTITASK_W {
        MULTITASK_W { w: self }
    }
}
