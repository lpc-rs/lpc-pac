#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFMR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct ACCOFFR {
    bits: bool,
}
impl ACCOFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ACCBPR {
    bits: bool,
}
impl ACCBPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `EFCAN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFCANR {
    #[doc = "Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    SOFTWARE_MUST_READ_A,
    #[doc = "The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    THE_ACCEPTANCE_FILTE,
}
impl EFCANR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EFCANR::SOFTWARE_MUST_READ_A => false,
            EFCANR::THE_ACCEPTANCE_FILTE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EFCANR {
        match value {
            false => EFCANR::SOFTWARE_MUST_READ_A,
            true => EFCANR::THE_ACCEPTANCE_FILTE,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE_MUST_READ_A`"]
    #[inline]
    pub fn is_software_must_read_a(&self) -> bool {
        *self == EFCANR::SOFTWARE_MUST_READ_A
    }
    #[doc = "Checks if the value of the field is `THE_ACCEPTANCE_FILTE`"]
    #[inline]
    pub fn is_the_acceptance_filte(&self) -> bool {
        *self == EFCANR::THE_ACCEPTANCE_FILTE
    }
}
#[doc = r" Proxy"]
pub struct _ACCOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _ACCOFFW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACCBPW<'a> {
    w: &'a mut W,
}
impl<'a> _ACCBPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EFCAN`"]
pub enum EFCANW {
    #[doc = "Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    SOFTWARE_MUST_READ_A,
    #[doc = "The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    THE_ACCEPTANCE_FILTE,
}
impl EFCANW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EFCANW::SOFTWARE_MUST_READ_A => false,
            EFCANW::THE_ACCEPTANCE_FILTE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EFCANW<'a> {
    w: &'a mut W,
}
impl<'a> _EFCANW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EFCANW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    #[inline]
    pub fn software_must_read_a(self) -> &'a mut W {
        self.variant(EFCANW::SOFTWARE_MUST_READ_A)
    }
    #[doc = "The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    #[inline]
    pub fn the_acceptance_filte(self) -> &'a mut W {
        self.variant(EFCANW::THE_ACCEPTANCE_FILTE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
    #[inline]
    pub fn accoff(&self) -> ACCOFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACCOFFR { bits }
    }
    #[doc = "Bit 1 - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
    #[inline]
    pub fn accbp(&self) -> ACCBPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACCBPR { bits }
    }
    #[doc = "Bit 2 - FullCAN mode"]
    #[inline]
    pub fn efcan(&self) -> EFCANR {
        EFCANR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
    #[inline]
    pub fn accoff(&mut self) -> _ACCOFFW {
        _ACCOFFW { w: self }
    }
    #[doc = "Bit 1 - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
    #[inline]
    pub fn accbp(&mut self) -> _ACCBPW {
        _ACCBPW { w: self }
    }
    #[doc = "Bit 2 - FullCAN mode"]
    #[inline]
    pub fn efcan(&mut self) -> _EFCANW {
        _EFCANW { w: self }
    }
}
