#[doc = "Reader of register AFMR"]
pub type R = crate::R<u32, super::AFMR>;
#[doc = "Writer for register AFMR"]
pub type W = crate::W<u32, super::AFMR>;
#[doc = "Register AFMR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACCOFF`"]
pub type ACCOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCOFF`"]
pub struct ACCOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCOFF_W<'a> {
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
#[doc = "Reader of field `ACCBP`"]
pub type ACCBP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCBP`"]
pub struct ACCBP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCBP_W<'a> {
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
#[doc = "FullCAN mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFCAN_A {
    #[doc = "0: Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    SOFTWARE_MUST_READ_A = 0,
    #[doc = "1: The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    THE_ACCEPTANCE_FILTE = 1,
}
impl From<EFCAN_A> for bool {
    #[inline(always)]
    fn from(variant: EFCAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EFCAN`"]
pub type EFCAN_R = crate::R<bool, EFCAN_A>;
impl EFCAN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFCAN_A {
        match self.bits {
            false => EFCAN_A::SOFTWARE_MUST_READ_A,
            true => EFCAN_A::THE_ACCEPTANCE_FILTE,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE_MUST_READ_A`"]
    #[inline(always)]
    pub fn is_software_must_read_a(&self) -> bool {
        *self == EFCAN_A::SOFTWARE_MUST_READ_A
    }
    #[doc = "Checks if the value of the field is `THE_ACCEPTANCE_FILTE`"]
    #[inline(always)]
    pub fn is_the_acceptance_filte(&self) -> bool {
        *self == EFCAN_A::THE_ACCEPTANCE_FILTE
    }
}
#[doc = "Write proxy for field `EFCAN`"]
pub struct EFCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> EFCAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EFCAN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    #[inline(always)]
    pub fn software_must_read_a(self) -> &'a mut W {
        self.variant(EFCAN_A::SOFTWARE_MUST_READ_A)
    }
    #[doc = "The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    #[inline(always)]
    pub fn the_acceptance_filte(self) -> &'a mut W {
        self.variant(EFCAN_A::THE_ACCEPTANCE_FILTE)
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
impl R {
    #[doc = "Bit 0 - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
    #[inline(always)]
    pub fn accoff(&self) -> ACCOFF_R {
        ACCOFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
    #[inline(always)]
    pub fn accbp(&self) -> ACCBP_R {
        ACCBP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FullCAN mode"]
    #[inline(always)]
    pub fn efcan(&self) -> EFCAN_R {
        EFCAN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
    #[inline(always)]
    pub fn accoff(&mut self) -> ACCOFF_W {
        ACCOFF_W { w: self }
    }
    #[doc = "Bit 1 - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
    #[inline(always)]
    pub fn accbp(&mut self) -> ACCBP_W {
        ACCBP_W { w: self }
    }
    #[doc = "Bit 2 - FullCAN mode"]
    #[inline(always)]
    pub fn efcan(&mut self) -> EFCAN_W {
        EFCAN_W { w: self }
    }
}
