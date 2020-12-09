#[doc = "Reader of register STARTER0"]
pub type R = crate::R<u32, super::STARTER0>;
#[doc = "Writer for register STARTER0"]
pub type W = crate::W<u32, super::STARTER0>;
#[doc = "Register STARTER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_BOD`"]
pub type WDT_BOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_BOD`"]
pub struct WDT_BOD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_BOD_W<'a> {
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
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
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
#[doc = "Reader of field `GINT0`"]
pub type GINT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GINT0`"]
pub struct GINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT0_W<'a> {
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
#[doc = "Reader of field `GINT1`"]
pub type GINT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GINT1`"]
pub struct GINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT1_W<'a> {
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
#[doc = "Reader of field `PIN_INT0`"]
pub type PIN_INT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIN_INT0`"]
pub struct PIN_INT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_INT0_W<'a> {
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
#[doc = "Reader of field `PIN_INT1`"]
pub type PIN_INT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIN_INT1`"]
pub struct PIN_INT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_INT1_W<'a> {
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
#[doc = "Reader of field `PIN_INT2`"]
pub type PIN_INT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIN_INT2`"]
pub struct PIN_INT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_INT2_W<'a> {
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
#[doc = "Reader of field `PIN_INT3`"]
pub type PIN_INT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIN_INT3`"]
pub struct PIN_INT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_INT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CTIMER3`"]
pub type CTIMER3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTIMER3`"]
pub struct CTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ADC0_SEQA`"]
pub type ADC0_SEQA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0_SEQA`"]
pub struct ADC0_SEQA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_SEQA_W<'a> {
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
#[doc = "Reader of field `ADC0_SEQB`"]
pub type ADC0_SEQB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0_SEQB`"]
pub struct ADC0_SEQB_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_SEQB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ADC0_THCMP`"]
pub type ADC0_THCMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0_THCMP`"]
pub struct ADC0_THCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_THCMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `HWVAD`"]
pub type HWVAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HWVAD`"]
pub struct HWVAD_W<'a> {
    w: &'a mut W,
}
impl<'a> HWVAD_W<'a> {
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
#[doc = "Reader of field `USB0_NEEDCLK`"]
pub type USB0_NEEDCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB0_NEEDCLK`"]
pub struct USB0_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_NEEDCLK_W<'a> {
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
#[doc = "Reader of field `USB0`"]
pub type USB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB0`"]
pub struct USB0_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - WWDT and BOD interrupt wake-up."]
    #[inline(always)]
    pub fn wdt_bod(&self) -> WDT_BOD_R {
        WDT_BOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA wake-up."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Group interrupt 0 wake-up."]
    #[inline(always)]
    pub fn gint0(&self) -> GINT0_R {
        GINT0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Group interrupt 1 wake-up."]
    #[inline(always)]
    pub fn gint1(&self) -> GINT1_R {
        GINT1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO pin interrupt 0 wake-up."]
    #[inline(always)]
    pub fn pin_int0(&self) -> PIN_INT0_R {
        PIN_INT0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO pin interrupt 1 wake-up."]
    #[inline(always)]
    pub fn pin_int1(&self) -> PIN_INT1_R {
        PIN_INT1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO pin interrupt 2 wake-up."]
    #[inline(always)]
    pub fn pin_int2(&self) -> PIN_INT2_R {
        PIN_INT2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO pin interrupt 3 wake-up."]
    #[inline(always)]
    pub fn pin_int3(&self) -> PIN_INT3_R {
        PIN_INT3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Micro-tick Timer wake-up."]
    #[inline(always)]
    pub fn utick(&self) -> UTICK_R {
        UTICK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Multi-Rate Timer wake-up."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Standard counter/timer CTIMER0 wake-up."]
    #[inline(always)]
    pub fn ctimer0(&self) -> CTIMER0_R {
        CTIMER0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Standard counter/timer CTIMER1 wake-up."]
    #[inline(always)]
    pub fn ctimer1(&self) -> CTIMER1_R {
        CTIMER1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SCT0 wake-up."]
    #[inline(always)]
    pub fn sct0(&self) -> SCT0_R {
        SCT0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Standard counter/timer CTIMER3 wake-up."]
    #[inline(always)]
    pub fn ctimer3(&self) -> CTIMER3_R {
        CTIMER3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Flexcomm0 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm0(&self) -> FLEXCOMM0_R {
        FLEXCOMM0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Flexcomm1 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm1(&self) -> FLEXCOMM1_R {
        FLEXCOMM1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Flexcomm2 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm2(&self) -> FLEXCOMM2_R {
        FLEXCOMM2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Flexcomm3 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm3(&self) -> FLEXCOMM3_R {
        FLEXCOMM3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Flexcomm4 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm4(&self) -> FLEXCOMM4_R {
        FLEXCOMM4_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Flexcomm5 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm5(&self) -> FLEXCOMM5_R {
        FLEXCOMM5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Flexcomm6 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm6(&self) -> FLEXCOMM6_R {
        FLEXCOMM6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Flexcomm7 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm7(&self) -> FLEXCOMM7_R {
        FLEXCOMM7_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC0 sequence A interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqa(&self) -> ADC0_SEQA_R {
        ADC0_SEQA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADC0 sequence B interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqb(&self) -> ADC0_SEQB_R {
        ADC0_SEQB_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC0 threshold and error interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_thcmp(&self) -> ADC0_THCMP_R {
        ADC0_THCMP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Digital microphone interrupt wake-up."]
    #[inline(always)]
    pub fn dmic(&self) -> DMIC_R {
        DMIC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Hardware voice activity detect interrupt wake-up."]
    #[inline(always)]
    pub fn hwvad(&self) -> HWVAD_R {
        HWVAD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB activity interrupt wake-up."]
    #[inline(always)]
    pub fn usb0_needclk(&self) -> USB0_NEEDCLK_R {
        USB0_NEEDCLK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - USB function interrupt wake-up."]
    #[inline(always)]
    pub fn usb0(&self) -> USB0_R {
        USB0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RTC interrupt alarm and wake-up timer."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WWDT and BOD interrupt wake-up."]
    #[inline(always)]
    pub fn wdt_bod(&mut self) -> WDT_BOD_W {
        WDT_BOD_W { w: self }
    }
    #[doc = "Bit 1 - DMA wake-up."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 2 - Group interrupt 0 wake-up."]
    #[inline(always)]
    pub fn gint0(&mut self) -> GINT0_W {
        GINT0_W { w: self }
    }
    #[doc = "Bit 3 - Group interrupt 1 wake-up."]
    #[inline(always)]
    pub fn gint1(&mut self) -> GINT1_W {
        GINT1_W { w: self }
    }
    #[doc = "Bit 4 - GPIO pin interrupt 0 wake-up."]
    #[inline(always)]
    pub fn pin_int0(&mut self) -> PIN_INT0_W {
        PIN_INT0_W { w: self }
    }
    #[doc = "Bit 5 - GPIO pin interrupt 1 wake-up."]
    #[inline(always)]
    pub fn pin_int1(&mut self) -> PIN_INT1_W {
        PIN_INT1_W { w: self }
    }
    #[doc = "Bit 6 - GPIO pin interrupt 2 wake-up."]
    #[inline(always)]
    pub fn pin_int2(&mut self) -> PIN_INT2_W {
        PIN_INT2_W { w: self }
    }
    #[doc = "Bit 7 - GPIO pin interrupt 3 wake-up."]
    #[inline(always)]
    pub fn pin_int3(&mut self) -> PIN_INT3_W {
        PIN_INT3_W { w: self }
    }
    #[doc = "Bit 8 - Micro-tick Timer wake-up."]
    #[inline(always)]
    pub fn utick(&mut self) -> UTICK_W {
        UTICK_W { w: self }
    }
    #[doc = "Bit 9 - Multi-Rate Timer wake-up."]
    #[inline(always)]
    pub fn mrt(&mut self) -> MRT_W {
        MRT_W { w: self }
    }
    #[doc = "Bit 10 - Standard counter/timer CTIMER0 wake-up."]
    #[inline(always)]
    pub fn ctimer0(&mut self) -> CTIMER0_W {
        CTIMER0_W { w: self }
    }
    #[doc = "Bit 11 - Standard counter/timer CTIMER1 wake-up."]
    #[inline(always)]
    pub fn ctimer1(&mut self) -> CTIMER1_W {
        CTIMER1_W { w: self }
    }
    #[doc = "Bit 12 - SCT0 wake-up."]
    #[inline(always)]
    pub fn sct0(&mut self) -> SCT0_W {
        SCT0_W { w: self }
    }
    #[doc = "Bit 13 - Standard counter/timer CTIMER3 wake-up."]
    #[inline(always)]
    pub fn ctimer3(&mut self) -> CTIMER3_W {
        CTIMER3_W { w: self }
    }
    #[doc = "Bit 14 - Flexcomm0 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm0(&mut self) -> FLEXCOMM0_W {
        FLEXCOMM0_W { w: self }
    }
    #[doc = "Bit 15 - Flexcomm1 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm1(&mut self) -> FLEXCOMM1_W {
        FLEXCOMM1_W { w: self }
    }
    #[doc = "Bit 16 - Flexcomm2 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm2(&mut self) -> FLEXCOMM2_W {
        FLEXCOMM2_W { w: self }
    }
    #[doc = "Bit 17 - Flexcomm3 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm3(&mut self) -> FLEXCOMM3_W {
        FLEXCOMM3_W { w: self }
    }
    #[doc = "Bit 18 - Flexcomm4 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm4(&mut self) -> FLEXCOMM4_W {
        FLEXCOMM4_W { w: self }
    }
    #[doc = "Bit 19 - Flexcomm5 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm5(&mut self) -> FLEXCOMM5_W {
        FLEXCOMM5_W { w: self }
    }
    #[doc = "Bit 20 - Flexcomm6 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm6(&mut self) -> FLEXCOMM6_W {
        FLEXCOMM6_W { w: self }
    }
    #[doc = "Bit 21 - Flexcomm7 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm7(&mut self) -> FLEXCOMM7_W {
        FLEXCOMM7_W { w: self }
    }
    #[doc = "Bit 22 - ADC0 sequence A interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqa(&mut self) -> ADC0_SEQA_W {
        ADC0_SEQA_W { w: self }
    }
    #[doc = "Bit 23 - ADC0 sequence B interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqb(&mut self) -> ADC0_SEQB_W {
        ADC0_SEQB_W { w: self }
    }
    #[doc = "Bit 24 - ADC0 threshold and error interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_thcmp(&mut self) -> ADC0_THCMP_W {
        ADC0_THCMP_W { w: self }
    }
    #[doc = "Bit 25 - Digital microphone interrupt wake-up."]
    #[inline(always)]
    pub fn dmic(&mut self) -> DMIC_W {
        DMIC_W { w: self }
    }
    #[doc = "Bit 26 - Hardware voice activity detect interrupt wake-up."]
    #[inline(always)]
    pub fn hwvad(&mut self) -> HWVAD_W {
        HWVAD_W { w: self }
    }
    #[doc = "Bit 27 - USB activity interrupt wake-up."]
    #[inline(always)]
    pub fn usb0_needclk(&mut self) -> USB0_NEEDCLK_W {
        USB0_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 28 - USB function interrupt wake-up."]
    #[inline(always)]
    pub fn usb0(&mut self) -> USB0_W {
        USB0_W { w: self }
    }
    #[doc = "Bit 29 - RTC interrupt alarm and wake-up timer."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
}
