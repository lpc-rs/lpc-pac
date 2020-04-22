#[doc = "Reader of register AHBCLKCTRL1"]
pub type R = crate::R<u32, super::AHBCLKCTRL1>;
#[doc = "Writer for register AHBCLKCTRL1"]
pub type W = crate::W<u32, super::AHBCLKCTRL1>;
#[doc = "Register AHBCLKCTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AHBCLKCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MRT`"]
pub type MRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MRT`"]
pub struct MRT_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_W<'a> {
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
#[doc = "Reader of field `RIT`"]
pub type RIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RIT`"]
pub struct RIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RIT_W<'a> {
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
#[doc = "Reader of field `SCT0`"]
pub type SCT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCT0`"]
pub struct SCT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_W<'a> {
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
#[doc = "Reader of field `MCAN0`"]
pub type MCAN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCAN0`"]
pub struct MCAN0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCAN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `MCAN1`"]
pub type MCAN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCAN1`"]
pub struct MCAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCAN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `UTICK`"]
pub type UTICK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTICK`"]
pub struct UTICK_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `FLEXCOMM0`"]
pub type FLEXCOMM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLEXCOMM0`"]
pub struct FLEXCOMM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `FLEXCOMM1`"]
pub type FLEXCOMM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLEXCOMM1`"]
pub struct FLEXCOMM1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `FLEXCOMM2`"]
pub type FLEXCOMM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLEXCOMM2`"]
pub struct FLEXCOMM2_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FLEXCOMM3`"]
pub type FLEXCOMM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLEXCOMM3`"]
pub struct FLEXCOMM3_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM3_W<'a> {
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
#[doc = "Reader of field `FLEXCOMM4`"]
pub type FLEXCOMM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLEXCOMM4`"]
pub struct FLEXCOMM4_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM4_W<'a> {
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
#[doc = "Reader of field `FLEXCOMM5`"]
pub type FLEXCOMM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLEXCOMM5`"]
pub struct FLEXCOMM5_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `FLEXCOMM6`"]
pub type FLEXCOMM6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLEXCOMM6`"]
pub struct FLEXCOMM6_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `FLEXCOMM7`"]
pub type FLEXCOMM7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLEXCOMM7`"]
pub struct FLEXCOMM7_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DMIC`"]
pub type DMIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMIC`"]
pub struct DMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `CTIMER2`"]
pub type CTIMER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTIMER2`"]
pub struct CTIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `USB0D`"]
pub type USB0D_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB0D`"]
pub struct USB0D_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0D_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `CTIMER0`"]
pub type CTIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTIMER0`"]
pub struct CTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CTIMER1`"]
pub type CTIMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTIMER1`"]
pub struct CTIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enables the clock for the Multi-Rate Timer."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the clock for the Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn rit(&self) -> RIT_R {
        RIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the clock for SCT0."]
    #[inline(always)]
    pub fn sct0(&self) -> SCT0_R {
        SCT0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for MCAN0."]
    #[inline(always)]
    pub fn mcan0(&self) -> MCAN0_R {
        MCAN0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for MCAN1."]
    #[inline(always)]
    pub fn mcan1(&self) -> MCAN1_R {
        MCAN1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the Micro-tick Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn utick(&self) -> UTICK_R {
        UTICK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for Flexcomm 0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm0(&self) -> FLEXCOMM0_R {
        FLEXCOMM0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for Flexcomm 1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm1(&self) -> FLEXCOMM1_R {
        FLEXCOMM1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for Flexcomm 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm2(&self) -> FLEXCOMM2_R {
        FLEXCOMM2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for Flexcomm 3. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm3(&self) -> FLEXCOMM3_R {
        FLEXCOMM3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for Flexcomm 4. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm4(&self) -> FLEXCOMM4_R {
        FLEXCOMM4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for Flexcomm 5. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm5(&self) -> FLEXCOMM5_R {
        FLEXCOMM5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for Flexcomm 6. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm6(&self) -> FLEXCOMM6_R {
        FLEXCOMM6_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for Flexcomm 7. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm7(&self) -> FLEXCOMM7_R {
        FLEXCOMM7_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the digital microphone interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dmic(&self) -> DMIC_R {
        DMIC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for CTIMER 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer2(&self) -> CTIMER2_R {
        CTIMER2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 device interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn usb0d(&self) -> USB0D_R {
        USB0D_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enables the clock for timer CTIMER0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer0(&self) -> CTIMER0_R {
        CTIMER0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for timer CTIMER1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer1(&self) -> CTIMER1_R {
        CTIMER1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the clock for the Multi-Rate Timer."]
    #[inline(always)]
    pub fn mrt(&mut self) -> MRT_W {
        MRT_W { w: self }
    }
    #[doc = "Bit 1 - Enables the clock for the Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn rit(&mut self) -> RIT_W {
        RIT_W { w: self }
    }
    #[doc = "Bit 2 - Enables the clock for SCT0."]
    #[inline(always)]
    pub fn sct0(&mut self) -> SCT0_W {
        SCT0_W { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for MCAN0."]
    #[inline(always)]
    pub fn mcan0(&mut self) -> MCAN0_W {
        MCAN0_W { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for MCAN1."]
    #[inline(always)]
    pub fn mcan1(&mut self) -> MCAN1_W {
        MCAN1_W { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the Micro-tick Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn utick(&mut self) -> UTICK_W {
        UTICK_W { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for Flexcomm 0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm0(&mut self) -> FLEXCOMM0_W {
        FLEXCOMM0_W { w: self }
    }
    #[doc = "Bit 12 - Enables the clock for Flexcomm 1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm1(&mut self) -> FLEXCOMM1_W {
        FLEXCOMM1_W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for Flexcomm 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm2(&mut self) -> FLEXCOMM2_W {
        FLEXCOMM2_W { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for Flexcomm 3. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm3(&mut self) -> FLEXCOMM3_W {
        FLEXCOMM3_W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for Flexcomm 4. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm4(&mut self) -> FLEXCOMM4_W {
        FLEXCOMM4_W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for Flexcomm 5. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm5(&mut self) -> FLEXCOMM5_W {
        FLEXCOMM5_W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for Flexcomm 6. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm6(&mut self) -> FLEXCOMM6_W {
        FLEXCOMM6_W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for Flexcomm 7. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm7(&mut self) -> FLEXCOMM7_W {
        FLEXCOMM7_W { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the digital microphone interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dmic(&mut self) -> DMIC_W {
        DMIC_W { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for CTIMER 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer2(&mut self) -> CTIMER2_W {
        CTIMER2_W { w: self }
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 device interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn usb0d(&mut self) -> USB0D_W {
        USB0D_W { w: self }
    }
    #[doc = "Bit 26 - Enables the clock for timer CTIMER0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer0(&mut self) -> CTIMER0_W {
        CTIMER0_W { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for timer CTIMER1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer1(&mut self) -> CTIMER1_W {
        CTIMER1_W { w: self }
    }
}
