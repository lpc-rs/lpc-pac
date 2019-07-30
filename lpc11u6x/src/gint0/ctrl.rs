#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTR {
    #[doc = "No interrupt request is pending."]
    NO_INTERRUPT_REQUEST,
    #[doc = "Interrupt request is active."]
    INTERRUPT_REQUEST_IS,
}
impl crate::ToBits<bool> for INTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INTR::NO_INTERRUPT_REQUEST => false,
            INTR::INTERRUPT_REQUEST_IS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INT_R = crate::FR<bool, INTR>;
impl INT_R {
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_REQUEST`"]
    #[inline(always)]
    pub fn is_no_interrupt_request(&self) -> bool {
        *self == INTR::NO_INTERRUPT_REQUEST
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_REQUEST_IS`"]
    #[inline(always)]
    pub fn is_interrupt_request_is(&self) -> bool {
        *self == INTR::INTERRUPT_REQUEST_IS
    }
}
#[doc = "Values that can be written to the field `INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTW {
    #[doc = "No interrupt request is pending."]
    NO_INTERRUPT_REQUEST,
    #[doc = "Interrupt request is active."]
    INTERRUPT_REQUEST_IS,
}
impl INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INTW::NO_INTERRUPT_REQUEST => false,
            INTW::INTERRUPT_REQUEST_IS => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INTW<'a> {
    w: &'a mut W,
}
impl<'a> _INTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt request is pending."]
    #[inline(always)]
    pub fn no_interrupt_request(self) -> &'a mut W {
        self.variant(INTW::NO_INTERRUPT_REQUEST)
    }
    #[doc = "Interrupt request is active."]
    #[inline(always)]
    pub fn interrupt_request_is(self) -> &'a mut W {
        self.variant(INTW::INTERRUPT_REQUEST_IS)
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
#[doc = "Possible values of the field `COMB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBR {
    #[doc = "OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    OR_FUNCTIONALITY_A_,
    #[doc = "AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    AND_FUNCTIONALITY_A,
}
impl crate::ToBits<bool> for COMBR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            COMBR::OR_FUNCTIONALITY_A_ => false,
            COMBR::AND_FUNCTIONALITY_A => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type COMB_R = crate::FR<bool, COMBR>;
impl COMB_R {
    #[doc = "Checks if the value of the field is `OR_FUNCTIONALITY_A_`"]
    #[inline(always)]
    pub fn is_or_functionality_a_(&self) -> bool {
        *self == COMBR::OR_FUNCTIONALITY_A_
    }
    #[doc = "Checks if the value of the field is `AND_FUNCTIONALITY_A`"]
    #[inline(always)]
    pub fn is_and_functionality_a(&self) -> bool {
        *self == COMBR::AND_FUNCTIONALITY_A
    }
}
#[doc = "Values that can be written to the field `COMB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBW {
    #[doc = "OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    OR_FUNCTIONALITY_A_,
    #[doc = "AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    AND_FUNCTIONALITY_A,
}
impl COMBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            COMBW::OR_FUNCTIONALITY_A_ => false,
            COMBW::AND_FUNCTIONALITY_A => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _COMBW<'a> {
    w: &'a mut W,
}
impl<'a> _COMBW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    #[inline(always)]
    pub fn or_functionality_a_(self) -> &'a mut W {
        self.variant(COMBW::OR_FUNCTIONALITY_A_)
    }
    #[doc = "AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    #[inline(always)]
    pub fn and_functionality_a(self) -> &'a mut W {
        self.variant(COMBW::AND_FUNCTIONALITY_A)
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
#[doc = "Possible values of the field `TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGR {
    #[doc = "Edge-triggered"]
    EDGE_TRIGGERED,
    #[doc = "Level-triggered"]
    LEVEL_TRIGGERED,
}
impl crate::ToBits<bool> for TRIGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TRIGR::EDGE_TRIGGERED => false,
            TRIGR::LEVEL_TRIGGERED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRIG_R = crate::FR<bool, TRIGR>;
impl TRIG_R {
    #[doc = "Checks if the value of the field is `EDGE_TRIGGERED`"]
    #[inline(always)]
    pub fn is_edge_triggered(&self) -> bool {
        *self == TRIGR::EDGE_TRIGGERED
    }
    #[doc = "Checks if the value of the field is `LEVEL_TRIGGERED`"]
    #[inline(always)]
    pub fn is_level_triggered(&self) -> bool {
        *self == TRIGR::LEVEL_TRIGGERED
    }
}
#[doc = "Values that can be written to the field `TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGW {
    #[doc = "Edge-triggered"]
    EDGE_TRIGGERED,
    #[doc = "Level-triggered"]
    LEVEL_TRIGGERED,
}
impl TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGW::EDGE_TRIGGERED => false,
            TRIGW::LEVEL_TRIGGERED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Edge-triggered"]
    #[inline(always)]
    pub fn edge_triggered(self) -> &'a mut W {
        self.variant(TRIGW::EDGE_TRIGGERED)
    }
    #[doc = "Level-triggered"]
    #[inline(always)]
    pub fn level_triggered(self) -> &'a mut W {
        self.variant(TRIGW::LEVEL_TRIGGERED)
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
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Combine enabled inputs for group interrupt"]
    #[inline(always)]
    pub fn comb(&self) -> COMB_R {
        COMB_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Group interrupt trigger"]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
    #[inline(always)]
    pub fn int(&mut self) -> _INTW {
        _INTW { w: self }
    }
    #[doc = "Bit 1 - Combine enabled inputs for group interrupt"]
    #[inline(always)]
    pub fn comb(&mut self) -> _COMBW {
        _COMBW { w: self }
    }
    #[doc = "Bit 2 - Group interrupt trigger"]
    #[inline(always)]
    pub fn trig(&mut self) -> _TRIGW {
        _TRIGW { w: self }
    }
}
