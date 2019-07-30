#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IER {
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
#[doc = "Possible values of the field `RBRINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBRINTENR {
    #[doc = "Disable the RDA interrupt."]
    DISABLE_THE_RDA_INTE,
    #[doc = "Enable the RDA interrupt."]
    ENABLE_THE_RDA_INTER,
}
impl crate::ToBits<bool> for RBRINTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RBRINTENR::DISABLE_THE_RDA_INTE => false,
            RBRINTENR::ENABLE_THE_RDA_INTER => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RBRINTEN_R = crate::FR<bool, RBRINTENR>;
impl RBRINTEN_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_RDA_INTE`"]
    #[inline(always)]
    pub fn is_disable_the_rda_inte(&self) -> bool {
        *self == RBRINTENR::DISABLE_THE_RDA_INTE
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_RDA_INTER`"]
    #[inline(always)]
    pub fn is_enable_the_rda_inter(&self) -> bool {
        *self == RBRINTENR::ENABLE_THE_RDA_INTER
    }
}
#[doc = "Values that can be written to the field `RBRINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBRINTENW {
    #[doc = "Disable the RDA interrupt."]
    DISABLE_THE_RDA_INTE,
    #[doc = "Enable the RDA interrupt."]
    ENABLE_THE_RDA_INTER,
}
impl RBRINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RBRINTENW::DISABLE_THE_RDA_INTE => false,
            RBRINTENW::ENABLE_THE_RDA_INTER => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RBRINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RBRINTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBRINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the RDA interrupt."]
    #[inline(always)]
    pub fn disable_the_rda_inte(self) -> &'a mut W {
        self.variant(RBRINTENW::DISABLE_THE_RDA_INTE)
    }
    #[doc = "Enable the RDA interrupt."]
    #[inline(always)]
    pub fn enable_the_rda_inter(self) -> &'a mut W {
        self.variant(RBRINTENW::ENABLE_THE_RDA_INTER)
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
#[doc = "Possible values of the field `THREINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREINTENR {
    #[doc = "Disable the THRE interrupt."]
    DISABLE_THE_THRE_INT,
    #[doc = "Enable the THRE interrupt."]
    ENABLE_THE_THRE_INTE,
}
impl crate::ToBits<bool> for THREINTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            THREINTENR::DISABLE_THE_THRE_INT => false,
            THREINTENR::ENABLE_THE_THRE_INTE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type THREINTEN_R = crate::FR<bool, THREINTENR>;
impl THREINTEN_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_THRE_INT`"]
    #[inline(always)]
    pub fn is_disable_the_thre_int(&self) -> bool {
        *self == THREINTENR::DISABLE_THE_THRE_INT
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_THRE_INTE`"]
    #[inline(always)]
    pub fn is_enable_the_thre_inte(&self) -> bool {
        *self == THREINTENR::ENABLE_THE_THRE_INTE
    }
}
#[doc = "Values that can be written to the field `THREINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREINTENW {
    #[doc = "Disable the THRE interrupt."]
    DISABLE_THE_THRE_INT,
    #[doc = "Enable the THRE interrupt."]
    ENABLE_THE_THRE_INTE,
}
impl THREINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            THREINTENW::DISABLE_THE_THRE_INT => false,
            THREINTENW::ENABLE_THE_THRE_INTE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _THREINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _THREINTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THREINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the THRE interrupt."]
    #[inline(always)]
    pub fn disable_the_thre_int(self) -> &'a mut W {
        self.variant(THREINTENW::DISABLE_THE_THRE_INT)
    }
    #[doc = "Enable the THRE interrupt."]
    #[inline(always)]
    pub fn enable_the_thre_inte(self) -> &'a mut W {
        self.variant(THREINTENW::ENABLE_THE_THRE_INTE)
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
#[doc = "Possible values of the field `RLSINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLSINTENR {
    #[doc = "Disable the RLS interrupt."]
    DISABLE_THE_RLS_INTE,
    #[doc = "Enable the RLS interrupt."]
    ENABLE_THE_RLS_INTER,
}
impl crate::ToBits<bool> for RLSINTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RLSINTENR::DISABLE_THE_RLS_INTE => false,
            RLSINTENR::ENABLE_THE_RLS_INTER => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RLSINTEN_R = crate::FR<bool, RLSINTENR>;
impl RLSINTEN_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_RLS_INTE`"]
    #[inline(always)]
    pub fn is_disable_the_rls_inte(&self) -> bool {
        *self == RLSINTENR::DISABLE_THE_RLS_INTE
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_RLS_INTER`"]
    #[inline(always)]
    pub fn is_enable_the_rls_inter(&self) -> bool {
        *self == RLSINTENR::ENABLE_THE_RLS_INTER
    }
}
#[doc = "Values that can be written to the field `RLSINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLSINTENW {
    #[doc = "Disable the RLS interrupt."]
    DISABLE_THE_RLS_INTE,
    #[doc = "Enable the RLS interrupt."]
    ENABLE_THE_RLS_INTER,
}
impl RLSINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RLSINTENW::DISABLE_THE_RLS_INTE => false,
            RLSINTENW::ENABLE_THE_RLS_INTER => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RLSINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RLSINTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RLSINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the RLS interrupt."]
    #[inline(always)]
    pub fn disable_the_rls_inte(self) -> &'a mut W {
        self.variant(RLSINTENW::DISABLE_THE_RLS_INTE)
    }
    #[doc = "Enable the RLS interrupt."]
    #[inline(always)]
    pub fn enable_the_rls_inter(self) -> &'a mut W {
        self.variant(RLSINTENW::ENABLE_THE_RLS_INTER)
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
#[doc = "Possible values of the field `MSINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSINTENR {
    #[doc = "Disable the MS interrupt."]
    DISABLE_THE_MS_INTER,
    #[doc = "Enable the MS interrupt."]
    ENABLE_THE_MS_INTERR,
}
impl crate::ToBits<bool> for MSINTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSINTENR::DISABLE_THE_MS_INTER => false,
            MSINTENR::ENABLE_THE_MS_INTERR => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSINTEN_R = crate::FR<bool, MSINTENR>;
impl MSINTEN_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_MS_INTER`"]
    #[inline(always)]
    pub fn is_disable_the_ms_inter(&self) -> bool {
        *self == MSINTENR::DISABLE_THE_MS_INTER
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_MS_INTERR`"]
    #[inline(always)]
    pub fn is_enable_the_ms_interr(&self) -> bool {
        *self == MSINTENR::ENABLE_THE_MS_INTERR
    }
}
#[doc = "Values that can be written to the field `MSINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSINTENW {
    #[doc = "Disable the MS interrupt."]
    DISABLE_THE_MS_INTER,
    #[doc = "Enable the MS interrupt."]
    ENABLE_THE_MS_INTERR,
}
impl MSINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSINTENW::DISABLE_THE_MS_INTER => false,
            MSINTENW::ENABLE_THE_MS_INTERR => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSINTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the MS interrupt."]
    #[inline(always)]
    pub fn disable_the_ms_inter(self) -> &'a mut W {
        self.variant(MSINTENW::DISABLE_THE_MS_INTER)
    }
    #[doc = "Enable the MS interrupt."]
    #[inline(always)]
    pub fn enable_the_ms_interr(self) -> &'a mut W {
        self.variant(MSINTENW::ENABLE_THE_MS_INTERR)
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
#[doc = "Possible values of the field `ABEOINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTENR {
    #[doc = "Disable end of auto-baud Interrupt."]
    DISABLE_END_OF_AUTO_,
    #[doc = "Enable end of auto-baud Interrupt."]
    ENABLE_END_OF_AUTO_B,
}
impl crate::ToBits<bool> for ABEOINTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ABEOINTENR::DISABLE_END_OF_AUTO_ => false,
            ABEOINTENR::ENABLE_END_OF_AUTO_B => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ABEOINTEN_R = crate::FR<bool, ABEOINTENR>;
impl ABEOINTEN_R {
    #[doc = "Checks if the value of the field is `DISABLE_END_OF_AUTO_`"]
    #[inline(always)]
    pub fn is_disable_end_of_auto_(&self) -> bool {
        *self == ABEOINTENR::DISABLE_END_OF_AUTO_
    }
    #[doc = "Checks if the value of the field is `ENABLE_END_OF_AUTO_B`"]
    #[inline(always)]
    pub fn is_enable_end_of_auto_b(&self) -> bool {
        *self == ABEOINTENR::ENABLE_END_OF_AUTO_B
    }
}
#[doc = "Values that can be written to the field `ABEOINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTENW {
    #[doc = "Disable end of auto-baud Interrupt."]
    DISABLE_END_OF_AUTO_,
    #[doc = "Enable end of auto-baud Interrupt."]
    ENABLE_END_OF_AUTO_B,
}
impl ABEOINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ABEOINTENW::DISABLE_END_OF_AUTO_ => false,
            ABEOINTENW::ENABLE_END_OF_AUTO_B => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ABEOINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ABEOINTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABEOINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn disable_end_of_auto_(self) -> &'a mut W {
        self.variant(ABEOINTENW::DISABLE_END_OF_AUTO_)
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn enable_end_of_auto_b(self) -> &'a mut W {
        self.variant(ABEOINTENW::ENABLE_END_OF_AUTO_B)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `ABTOINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTENR {
    #[doc = "Disable auto-baud time-out Interrupt."]
    DISABLE_AUTO_BAUD_TI,
    #[doc = "Enable auto-baud time-out Interrupt."]
    ENABLE_AUTO_BAUD_TIM,
}
impl crate::ToBits<bool> for ABTOINTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ABTOINTENR::DISABLE_AUTO_BAUD_TI => false,
            ABTOINTENR::ENABLE_AUTO_BAUD_TIM => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ABTOINTEN_R = crate::FR<bool, ABTOINTENR>;
impl ABTOINTEN_R {
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_BAUD_TI`"]
    #[inline(always)]
    pub fn is_disable_auto_baud_ti(&self) -> bool {
        *self == ABTOINTENR::DISABLE_AUTO_BAUD_TI
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_BAUD_TIM`"]
    #[inline(always)]
    pub fn is_enable_auto_baud_tim(&self) -> bool {
        *self == ABTOINTENR::ENABLE_AUTO_BAUD_TIM
    }
}
#[doc = "Values that can be written to the field `ABTOINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTENW {
    #[doc = "Disable auto-baud time-out Interrupt."]
    DISABLE_AUTO_BAUD_TI,
    #[doc = "Enable auto-baud time-out Interrupt."]
    ENABLE_AUTO_BAUD_TIM,
}
impl ABTOINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ABTOINTENW::DISABLE_AUTO_BAUD_TI => false,
            ABTOINTENW::ENABLE_AUTO_BAUD_TIM => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ABTOINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ABTOINTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTOINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn disable_auto_baud_ti(self) -> &'a mut W {
        self.variant(ABTOINTENW::DISABLE_AUTO_BAUD_TI)
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn enable_auto_baud_tim(self) -> &'a mut W {
        self.variant(ABTOINTENW::ENABLE_AUTO_BAUD_TIM)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrinten(&self) -> RBRINTEN_R {
        RBRINTEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threinten(&self) -> THREINTEN_R {
        THREINTEN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rlsinten(&self) -> RLSINTEN_R {
        RLSINTEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR."]
    #[inline(always)]
    pub fn msinten(&self) -> MSINTEN_R {
        MSINTEN_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&self) -> ABEOINTEN_R {
        ABEOINTEN_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&self) -> ABTOINTEN_R {
        ABTOINTEN_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrinten(&mut self) -> _RBRINTENW {
        _RBRINTENW { w: self }
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threinten(&mut self) -> _THREINTENW {
        _THREINTENW { w: self }
    }
    #[doc = "Bit 2 - Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rlsinten(&mut self) -> _RLSINTENW {
        _RLSINTENW { w: self }
    }
    #[doc = "Bit 3 - Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR."]
    #[inline(always)]
    pub fn msinten(&mut self) -> _MSINTENW {
        _MSINTENW { w: self }
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&mut self) -> _ABEOINTENW {
        _ABEOINTENW { w: self }
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&mut self) -> _ABTOINTENW {
        _ABTOINTENW { w: self }
    }
}
