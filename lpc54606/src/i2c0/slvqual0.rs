#[doc = "Reader of register SLVQUAL0"]
pub type R = crate::R<u32, super::SLVQUAL0>;
#[doc = "Writer for register SLVQUAL0"]
pub type W = crate::W<u32, super::SLVQUAL0>;
#[doc = "Register SLVQUAL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SLVQUAL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Qualify mode for slave address 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUALMODE0_A {
    #[doc = "0: Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    MASK = 0,
    #[doc = "1: Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    EXTEND = 1,
}
impl From<QUALMODE0_A> for bool {
    #[inline(always)]
    fn from(variant: QUALMODE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QUALMODE0`"]
pub type QUALMODE0_R = crate::R<bool, QUALMODE0_A>;
impl QUALMODE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUALMODE0_A {
        match self.bits {
            false => QUALMODE0_A::MASK,
            true => QUALMODE0_A::EXTEND,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == QUALMODE0_A::MASK
    }
    #[doc = "Checks if the value of the field is `EXTEND`"]
    #[inline(always)]
    pub fn is_extend(&self) -> bool {
        *self == QUALMODE0_A::EXTEND
    }
}
#[doc = "Write proxy for field `QUALMODE0`"]
pub struct QUALMODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> QUALMODE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUALMODE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(QUALMODE0_A::MASK)
    }
    #[doc = "Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    #[inline(always)]
    pub fn extend(self) -> &'a mut W {
        self.variant(QUALMODE0_A::EXTEND)
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
#[doc = "Reader of field `SLVQUAL0`"]
pub type SLVQUAL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLVQUAL0`"]
pub struct SLVQUAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVQUAL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Qualify mode for slave address 0."]
    #[inline(always)]
    pub fn qualmode0(&self) -> QUALMODE0_R {
        QUALMODE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\]
<= received address <= SLVQUAL0\\[7:1\\])."]
    #[inline(always)]
    pub fn slvqual0(&self) -> SLVQUAL0_R {
        SLVQUAL0_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Qualify mode for slave address 0."]
    #[inline(always)]
    pub fn qualmode0(&mut self) -> QUALMODE0_W {
        QUALMODE0_W { w: self }
    }
    #[doc = "Bits 1:7 - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\]
<= received address <= SLVQUAL0\\[7:1\\])."]
    #[inline(always)]
    pub fn slvqual0(&mut self) -> SLVQUAL0_W {
        SLVQUAL0_W { w: self }
    }
}
