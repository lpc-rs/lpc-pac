#[doc = "Reader of register PCONP"]
pub type R = crate::R<u32, super::PCONP>;
#[doc = "Writer for register PCONP"]
pub type W = crate::W<u32, super::PCONP>;
#[doc = "Register PCONP `reset()`'s with value 0x03be"]
impl crate::ResetValue for super::PCONP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03be
    }
}
#[doc = "Reader of field `PCTIM0`"]
pub type PCTIM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCTIM0`"]
pub struct PCTIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTIM0_W<'a> {
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
#[doc = "Reader of field `PCTIM1`"]
pub type PCTIM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCTIM1`"]
pub struct PCTIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTIM1_W<'a> {
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
#[doc = "Reader of field `PCUART0`"]
pub type PCUART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCUART0`"]
pub struct PCUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCUART0_W<'a> {
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
#[doc = "Reader of field `PCUART1`"]
pub type PCUART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCUART1`"]
pub struct PCUART1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCUART1_W<'a> {
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
#[doc = "Reader of field `PCPWM1`"]
pub type PCPWM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCPWM1`"]
pub struct PCPWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCPWM1_W<'a> {
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
#[doc = "Reader of field `PCI2C0`"]
pub type PCI2C0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCI2C0`"]
pub struct PCI2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCI2C0_W<'a> {
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
#[doc = "Reader of field `PCSPI`"]
pub type PCSPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSPI`"]
pub struct PCSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSPI_W<'a> {
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
#[doc = "Reader of field `PCRTC`"]
pub type PCRTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCRTC`"]
pub struct PCRTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCRTC_W<'a> {
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
#[doc = "Reader of field `PCSSP1`"]
pub type PCSSP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSSP1`"]
pub struct PCSSP1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSSP1_W<'a> {
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
#[doc = "Reader of field `PCADC`"]
pub type PCADC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCADC`"]
pub struct PCADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCADC_W<'a> {
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
#[doc = "Reader of field `PCCAN1`"]
pub type PCCAN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCCAN1`"]
pub struct PCCAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCAN1_W<'a> {
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
#[doc = "Reader of field `PCCAN2`"]
pub type PCCAN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCCAN2`"]
pub struct PCCAN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCAN2_W<'a> {
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
#[doc = "Reader of field `PCGPIO`"]
pub type PCGPIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCGPIO`"]
pub struct PCGPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PCGPIO_W<'a> {
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
#[doc = "Reader of field `PCRIT`"]
pub type PCRIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCRIT`"]
pub struct PCRIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCRIT_W<'a> {
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
#[doc = "Reader of field `PCMCPWM`"]
pub type PCMCPWM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCMCPWM`"]
pub struct PCMCPWM_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMCPWM_W<'a> {
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
#[doc = "Reader of field `PCQEI`"]
pub type PCQEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCQEI`"]
pub struct PCQEI_W<'a> {
    w: &'a mut W,
}
impl<'a> PCQEI_W<'a> {
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
#[doc = "Reader of field `PCI2C1`"]
pub type PCI2C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCI2C1`"]
pub struct PCI2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCI2C1_W<'a> {
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
#[doc = "Reader of field `PCSSP0`"]
pub type PCSSP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSSP0`"]
pub struct PCSSP0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSSP0_W<'a> {
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
#[doc = "Reader of field `PCTIM2`"]
pub type PCTIM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCTIM2`"]
pub struct PCTIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTIM2_W<'a> {
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
#[doc = "Reader of field `PCTIM3`"]
pub type PCTIM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCTIM3`"]
pub struct PCTIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTIM3_W<'a> {
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
#[doc = "Reader of field `PCUART2`"]
pub type PCUART2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCUART2`"]
pub struct PCUART2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCUART2_W<'a> {
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
#[doc = "Reader of field `PCUART3`"]
pub type PCUART3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCUART3`"]
pub struct PCUART3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCUART3_W<'a> {
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
#[doc = "Reader of field `PCI2C2`"]
pub type PCI2C2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCI2C2`"]
pub struct PCI2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCI2C2_W<'a> {
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
#[doc = "Reader of field `PCI2S`"]
pub type PCI2S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCI2S`"]
pub struct PCI2S_W<'a> {
    w: &'a mut W,
}
impl<'a> PCI2S_W<'a> {
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
#[doc = "Reader of field `PCGPDMA`"]
pub type PCGPDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCGPDMA`"]
pub struct PCGPDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> PCGPDMA_W<'a> {
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
#[doc = "Reader of field `PCENET`"]
pub type PCENET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCENET`"]
pub struct PCENET_W<'a> {
    w: &'a mut W,
}
impl<'a> PCENET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `PCUSB`"]
pub type PCUSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCUSB`"]
pub struct PCUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PCUSB_W<'a> {
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
    #[doc = "Bit 1 - Timer/Counter 0 power/clock control bit."]
    #[inline(always)]
    pub fn pctim0(&self) -> PCTIM0_R {
        PCTIM0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter 1 power/clock control bit."]
    #[inline(always)]
    pub fn pctim1(&self) -> PCTIM1_R {
        PCTIM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART0 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart0(&self) -> PCUART0_R {
        PCUART0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART1 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart1(&self) -> PCUART1_R {
        PCUART1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PWM1 power/clock control bit."]
    #[inline(always)]
    pub fn pcpwm1(&self) -> PCPWM1_R {
        PCPWM1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The I2C0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c0(&self) -> PCI2C0_R {
        PCI2C0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The SPI interface power/clock control bit."]
    #[inline(always)]
    pub fn pcspi(&self) -> PCSPI_R {
        PCSPI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The RTC power/clock control bit."]
    #[inline(always)]
    pub fn pcrtc(&self) -> PCRTC_R {
        PCRTC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The SSP 1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp1(&self) -> PCSSP1_R {
        PCSSP1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN."]
    #[inline(always)]
    pub fn pcadc(&self) -> PCADC_R {
        PCADC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CAN Controller 1 power/clock control bit."]
    #[inline(always)]
    pub fn pccan1(&self) -> PCCAN1_R {
        PCCAN1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CAN Controller 2 power/clock control bit."]
    #[inline(always)]
    pub fn pccan2(&self) -> PCCAN2_R {
        PCCAN2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
    #[inline(always)]
    pub fn pcgpio(&self) -> PCGPIO_R {
        PCGPIO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Repetitive Interrupt Timer power/clock control bit."]
    #[inline(always)]
    pub fn pcrit(&self) -> PCRIT_R {
        PCRIT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Motor Control PWM"]
    #[inline(always)]
    pub fn pcmcpwm(&self) -> PCMCPWM_R {
        PCMCPWM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface power/clock control bit."]
    #[inline(always)]
    pub fn pcqei(&self) -> PCQEI_R {
        PCQEI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The I2C1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c1(&self) -> PCI2C1_R {
        PCI2C1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The SSP0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp0(&self) -> PCSSP0_R {
        PCSSP0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timer 2 power/clock control bit."]
    #[inline(always)]
    pub fn pctim2(&self) -> PCTIM2_R {
        PCTIM2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Timer 3 power/clock control bit."]
    #[inline(always)]
    pub fn pctim3(&self) -> PCTIM3_R {
        PCTIM3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - UART 2 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart2(&self) -> PCUART2_R {
        PCUART2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - UART 3 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart3(&self) -> PCUART3_R {
        PCUART3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - I2C interface 2 power/clock control bit."]
    #[inline(always)]
    pub fn pci2c2(&self) -> PCI2C2_R {
        PCI2C2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - I2S interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2s(&self) -> PCI2S_R {
        PCI2S_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GPDMA function power/clock control bit."]
    #[inline(always)]
    pub fn pcgpdma(&self) -> PCGPDMA_R {
        PCGPDMA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Ethernet block power/clock control bit."]
    #[inline(always)]
    pub fn pcenet(&self) -> PCENET_R {
        PCENET_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - USB interface power/clock control bit."]
    #[inline(always)]
    pub fn pcusb(&self) -> PCUSB_R {
        PCUSB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Timer/Counter 0 power/clock control bit."]
    #[inline(always)]
    pub fn pctim0(&mut self) -> PCTIM0_W {
        PCTIM0_W { w: self }
    }
    #[doc = "Bit 2 - Timer/Counter 1 power/clock control bit."]
    #[inline(always)]
    pub fn pctim1(&mut self) -> PCTIM1_W {
        PCTIM1_W { w: self }
    }
    #[doc = "Bit 3 - UART0 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart0(&mut self) -> PCUART0_W {
        PCUART0_W { w: self }
    }
    #[doc = "Bit 4 - UART1 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart1(&mut self) -> PCUART1_W {
        PCUART1_W { w: self }
    }
    #[doc = "Bit 6 - PWM1 power/clock control bit."]
    #[inline(always)]
    pub fn pcpwm1(&mut self) -> PCPWM1_W {
        PCPWM1_W { w: self }
    }
    #[doc = "Bit 7 - The I2C0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c0(&mut self) -> PCI2C0_W {
        PCI2C0_W { w: self }
    }
    #[doc = "Bit 8 - The SPI interface power/clock control bit."]
    #[inline(always)]
    pub fn pcspi(&mut self) -> PCSPI_W {
        PCSPI_W { w: self }
    }
    #[doc = "Bit 9 - The RTC power/clock control bit."]
    #[inline(always)]
    pub fn pcrtc(&mut self) -> PCRTC_W {
        PCRTC_W { w: self }
    }
    #[doc = "Bit 10 - The SSP 1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp1(&mut self) -> PCSSP1_W {
        PCSSP1_W { w: self }
    }
    #[doc = "Bit 12 - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN."]
    #[inline(always)]
    pub fn pcadc(&mut self) -> PCADC_W {
        PCADC_W { w: self }
    }
    #[doc = "Bit 13 - CAN Controller 1 power/clock control bit."]
    #[inline(always)]
    pub fn pccan1(&mut self) -> PCCAN1_W {
        PCCAN1_W { w: self }
    }
    #[doc = "Bit 14 - CAN Controller 2 power/clock control bit."]
    #[inline(always)]
    pub fn pccan2(&mut self) -> PCCAN2_W {
        PCCAN2_W { w: self }
    }
    #[doc = "Bit 15 - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
    #[inline(always)]
    pub fn pcgpio(&mut self) -> PCGPIO_W {
        PCGPIO_W { w: self }
    }
    #[doc = "Bit 16 - Repetitive Interrupt Timer power/clock control bit."]
    #[inline(always)]
    pub fn pcrit(&mut self) -> PCRIT_W {
        PCRIT_W { w: self }
    }
    #[doc = "Bit 17 - Motor Control PWM"]
    #[inline(always)]
    pub fn pcmcpwm(&mut self) -> PCMCPWM_W {
        PCMCPWM_W { w: self }
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface power/clock control bit."]
    #[inline(always)]
    pub fn pcqei(&mut self) -> PCQEI_W {
        PCQEI_W { w: self }
    }
    #[doc = "Bit 19 - The I2C1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c1(&mut self) -> PCI2C1_W {
        PCI2C1_W { w: self }
    }
    #[doc = "Bit 21 - The SSP0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp0(&mut self) -> PCSSP0_W {
        PCSSP0_W { w: self }
    }
    #[doc = "Bit 22 - Timer 2 power/clock control bit."]
    #[inline(always)]
    pub fn pctim2(&mut self) -> PCTIM2_W {
        PCTIM2_W { w: self }
    }
    #[doc = "Bit 23 - Timer 3 power/clock control bit."]
    #[inline(always)]
    pub fn pctim3(&mut self) -> PCTIM3_W {
        PCTIM3_W { w: self }
    }
    #[doc = "Bit 24 - UART 2 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart2(&mut self) -> PCUART2_W {
        PCUART2_W { w: self }
    }
    #[doc = "Bit 25 - UART 3 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart3(&mut self) -> PCUART3_W {
        PCUART3_W { w: self }
    }
    #[doc = "Bit 26 - I2C interface 2 power/clock control bit."]
    #[inline(always)]
    pub fn pci2c2(&mut self) -> PCI2C2_W {
        PCI2C2_W { w: self }
    }
    #[doc = "Bit 27 - I2S interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2s(&mut self) -> PCI2S_W {
        PCI2S_W { w: self }
    }
    #[doc = "Bit 29 - GPDMA function power/clock control bit."]
    #[inline(always)]
    pub fn pcgpdma(&mut self) -> PCGPDMA_W {
        PCGPDMA_W { w: self }
    }
    #[doc = "Bit 30 - Ethernet block power/clock control bit."]
    #[inline(always)]
    pub fn pcenet(&mut self) -> PCENET_W {
        PCENET_W { w: self }
    }
    #[doc = "Bit 31 - USB interface power/clock control bit."]
    #[inline(always)]
    pub fn pcusb(&mut self) -> PCUSB_W {
        PCUSB_W { w: self }
    }
}
