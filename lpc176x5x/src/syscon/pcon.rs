#[doc = "Reader of register PCON"]
pub type R = crate::R<u32, super::PCON>;
#[doc = "Writer for register PCON"]
pub type W = crate::W<u32, super::PCON>;
#[doc = "Register PCON `reset()`'s with value 0"]
impl crate::ResetValue for super::PCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PM0`"]
pub type PM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PM0`"]
pub struct PM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PM0_W<'a> {
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
#[doc = "Reader of field `PM1`"]
pub type PM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PM1`"]
pub struct PM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PM1_W<'a> {
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
#[doc = "Reader of field `BODRPM`"]
pub type BODRPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODRPM`"]
pub struct BODRPM_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRPM_W<'a> {
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
#[doc = "Reader of field `BOGD`"]
pub type BOGD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOGD`"]
pub struct BOGD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOGD_W<'a> {
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
#[doc = "Reader of field `BORD`"]
pub type BORD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BORD`"]
pub struct BORD_W<'a> {
    w: &'a mut W,
}
impl<'a> BORD_W<'a> {
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
#[doc = "Reader of field `SMFLAG`"]
pub type SMFLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMFLAG`"]
pub struct SMFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SMFLAG_W<'a> {
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
#[doc = "Reader of field `DSFLAG`"]
pub type DSFLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSFLAG`"]
pub struct DSFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DSFLAG_W<'a> {
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
#[doc = "Reader of field `PDFLAG`"]
pub type PDFLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDFLAG`"]
pub struct PDFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> PDFLAG_W<'a> {
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
#[doc = "Reader of field `DPDFLAG`"]
pub type DPDFLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPDFLAG`"]
pub struct DPDFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DPDFLAG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Power mode control bit 0. This bit controls entry to the Power-down mode."]
    #[inline(always)]
    pub fn pm0(&self) -> PM0_R {
        PM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power mode control bit 1. This bit controls entry to the Deep Power-down mode."]
    #[inline(always)]
    pub fn pm1(&self) -> PM1_R {
        PM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
    #[inline(always)]
    pub fn bodrpm(&self) -> BODRPM_R {
        BODRPM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection. Note: the Brown-Out Reset Disable (BORD, in this register) and the Brown-Out Interrupt (xx) must be disabled when software changes the value of this bit."]
    #[inline(always)]
    pub fn bogd(&self) -> BOGD_R {
        BOGD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled."]
    #[inline(always)]
    pub fn bord(&self) -> BORD_R {
        BORD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn smflag(&self) -> SMFLAG_R {
        SMFLAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn dsflag(&self) -> DSFLAG_R {
        DSFLAG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn pdflag(&self) -> PDFLAG_R {
        PDFLAG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn dpdflag(&self) -> DPDFLAG_R {
        DPDFLAG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power mode control bit 0. This bit controls entry to the Power-down mode."]
    #[inline(always)]
    pub fn pm0(&mut self) -> PM0_W {
        PM0_W { w: self }
    }
    #[doc = "Bit 1 - Power mode control bit 1. This bit controls entry to the Deep Power-down mode."]
    #[inline(always)]
    pub fn pm1(&mut self) -> PM1_W {
        PM1_W { w: self }
    }
    #[doc = "Bit 2 - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
    #[inline(always)]
    pub fn bodrpm(&mut self) -> BODRPM_W {
        BODRPM_W { w: self }
    }
    #[doc = "Bit 3 - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection. Note: the Brown-Out Reset Disable (BORD, in this register) and the Brown-Out Interrupt (xx) must be disabled when software changes the value of this bit."]
    #[inline(always)]
    pub fn bogd(&mut self) -> BOGD_W {
        BOGD_W { w: self }
    }
    #[doc = "Bit 4 - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled."]
    #[inline(always)]
    pub fn bord(&mut self) -> BORD_W {
        BORD_W { w: self }
    }
    #[doc = "Bit 8 - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn smflag(&mut self) -> SMFLAG_W {
        SMFLAG_W { w: self }
    }
    #[doc = "Bit 9 - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn dsflag(&mut self) -> DSFLAG_W {
        DSFLAG_W { w: self }
    }
    #[doc = "Bit 10 - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn pdflag(&mut self) -> PDFLAG_W {
        PDFLAG_W { w: self }
    }
    #[doc = "Bit 11 - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn dpdflag(&mut self) -> DPDFLAG_W {
        DPDFLAG_W { w: self }
    }
}
