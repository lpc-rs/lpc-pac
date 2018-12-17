#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSRSTSTAT {
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
#[doc = "Possible values of the field `POR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR {
    #[doc = "No POR detected"]
    NO_POR_DETECTED,
    #[doc = "POR detected. Writing a one clears this reset."]
    POR_DETECTED_WRITIN,
}
impl PORR {
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
            PORR::NO_POR_DETECTED => false,
            PORR::POR_DETECTED_WRITIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PORR {
        match value {
            false => PORR::NO_POR_DETECTED,
            true => PORR::POR_DETECTED_WRITIN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_POR_DETECTED`"]
    #[inline]
    pub fn is_no_por_detected(&self) -> bool {
        *self == PORR::NO_POR_DETECTED
    }
    #[doc = "Checks if the value of the field is `POR_DETECTED_WRITIN`"]
    #[inline]
    pub fn is_por_detected_writin(&self) -> bool {
        *self == PORR::POR_DETECTED_WRITIN
    }
}
#[doc = "Possible values of the field `EXTRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRSTR {
    #[doc = "No reset event detected."]
    NO_RESET_EVENT_DETEC,
    #[doc = "Reset detected. Writing a one clears this reset."]
    RESET_DETECTED_WRIT,
}
impl EXTRSTR {
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
            EXTRSTR::NO_RESET_EVENT_DETEC => false,
            EXTRSTR::RESET_DETECTED_WRIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTRSTR {
        match value {
            false => EXTRSTR::NO_RESET_EVENT_DETEC,
            true => EXTRSTR::RESET_DETECTED_WRIT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESET_EVENT_DETEC`"]
    #[inline]
    pub fn is_no_reset_event_detec(&self) -> bool {
        *self == EXTRSTR::NO_RESET_EVENT_DETEC
    }
    #[doc = "Checks if the value of the field is `RESET_DETECTED_WRIT`"]
    #[inline]
    pub fn is_reset_detected_writ(&self) -> bool {
        *self == EXTRSTR::RESET_DETECTED_WRIT
    }
}
#[doc = "Possible values of the field `WDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTR {
    #[doc = "No WDT reset detected"]
    NO_WDT_RESET_DETECTE,
    #[doc = "WDT reset detected. Writing a one clears this reset."]
    WDT_RESET_DETECTED_,
}
impl WDTR {
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
            WDTR::NO_WDT_RESET_DETECTE => false,
            WDTR::WDT_RESET_DETECTED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTR {
        match value {
            false => WDTR::NO_WDT_RESET_DETECTE,
            true => WDTR::WDT_RESET_DETECTED_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_WDT_RESET_DETECTE`"]
    #[inline]
    pub fn is_no_wdt_reset_detecte(&self) -> bool {
        *self == WDTR::NO_WDT_RESET_DETECTE
    }
    #[doc = "Checks if the value of the field is `WDT_RESET_DETECTED_`"]
    #[inline]
    pub fn is_wdt_reset_detected_(&self) -> bool {
        *self == WDTR::WDT_RESET_DETECTED_
    }
}
#[doc = "Possible values of the field `BOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODR {
    #[doc = "No BOD reset detected"]
    NO_BOD_RESET_DETECTE,
    #[doc = "BOD reset detected. Writing a one clears this reset."]
    BOD_RESET_DETECTED_,
}
impl BODR {
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
            BODR::NO_BOD_RESET_DETECTE => false,
            BODR::BOD_RESET_DETECTED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODR {
        match value {
            false => BODR::NO_BOD_RESET_DETECTE,
            true => BODR::BOD_RESET_DETECTED_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BOD_RESET_DETECTE`"]
    #[inline]
    pub fn is_no_bod_reset_detecte(&self) -> bool {
        *self == BODR::NO_BOD_RESET_DETECTE
    }
    #[doc = "Checks if the value of the field is `BOD_RESET_DETECTED_`"]
    #[inline]
    pub fn is_bod_reset_detected_(&self) -> bool {
        *self == BODR::BOD_RESET_DETECTED_
    }
}
#[doc = "Possible values of the field `SYSRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRSTR {
    #[doc = "No System reset detected"]
    NO_SYSTEM_RESET_DETE,
    #[doc = "System reset detected. Writing a one clears this reset."]
    SYSTEM_RESET_DETECTE,
}
impl SYSRSTR {
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
            SYSRSTR::NO_SYSTEM_RESET_DETE => false,
            SYSRSTR::SYSTEM_RESET_DETECTE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSRSTR {
        match value {
            false => SYSRSTR::NO_SYSTEM_RESET_DETE,
            true => SYSRSTR::SYSTEM_RESET_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYSTEM_RESET_DETE`"]
    #[inline]
    pub fn is_no_system_reset_dete(&self) -> bool {
        *self == SYSRSTR::NO_SYSTEM_RESET_DETE
    }
    #[doc = "Checks if the value of the field is `SYSTEM_RESET_DETECTE`"]
    #[inline]
    pub fn is_system_reset_detecte(&self) -> bool {
        *self == SYSRSTR::SYSTEM_RESET_DETECTE
    }
}
#[doc = "Values that can be written to the field `POR`"]
pub enum PORW {
    #[doc = "No POR detected"]
    NO_POR_DETECTED,
    #[doc = "POR detected. Writing a one clears this reset."]
    POR_DETECTED_WRITIN,
}
impl PORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PORW::NO_POR_DETECTED => false,
            PORW::POR_DETECTED_WRITIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PORW<'a> {
    w: &'a mut W,
}
impl<'a> _PORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No POR detected"]
    #[inline]
    pub fn no_por_detected(self) -> &'a mut W {
        self.variant(PORW::NO_POR_DETECTED)
    }
    #[doc = "POR detected. Writing a one clears this reset."]
    #[inline]
    pub fn por_detected_writin(self) -> &'a mut W {
        self.variant(PORW::POR_DETECTED_WRITIN)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTRST`"]
pub enum EXTRSTW {
    #[doc = "No reset event detected."]
    NO_RESET_EVENT_DETEC,
    #[doc = "Reset detected. Writing a one clears this reset."]
    RESET_DETECTED_WRIT,
}
impl EXTRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTRSTW::NO_RESET_EVENT_DETEC => false,
            EXTRSTW::RESET_DETECTED_WRIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset event detected."]
    #[inline]
    pub fn no_reset_event_detec(self) -> &'a mut W {
        self.variant(EXTRSTW::NO_RESET_EVENT_DETEC)
    }
    #[doc = "Reset detected. Writing a one clears this reset."]
    #[inline]
    pub fn reset_detected_writ(self) -> &'a mut W {
        self.variant(EXTRSTW::RESET_DETECTED_WRIT)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDT`"]
pub enum WDTW {
    #[doc = "No WDT reset detected"]
    NO_WDT_RESET_DETECTE,
    #[doc = "WDT reset detected. Writing a one clears this reset."]
    WDT_RESET_DETECTED_,
}
impl WDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTW::NO_WDT_RESET_DETECTE => false,
            WDTW::WDT_RESET_DETECTED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No WDT reset detected"]
    #[inline]
    pub fn no_wdt_reset_detecte(self) -> &'a mut W {
        self.variant(WDTW::NO_WDT_RESET_DETECTE)
    }
    #[doc = "WDT reset detected. Writing a one clears this reset."]
    #[inline]
    pub fn wdt_reset_detected_(self) -> &'a mut W {
        self.variant(WDTW::WDT_RESET_DETECTED_)
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
#[doc = "Values that can be written to the field `BOD`"]
pub enum BODW {
    #[doc = "No BOD reset detected"]
    NO_BOD_RESET_DETECTE,
    #[doc = "BOD reset detected. Writing a one clears this reset."]
    BOD_RESET_DETECTED_,
}
impl BODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BODW::NO_BOD_RESET_DETECTE => false,
            BODW::BOD_RESET_DETECTED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODW<'a> {
    w: &'a mut W,
}
impl<'a> _BODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No BOD reset detected"]
    #[inline]
    pub fn no_bod_reset_detecte(self) -> &'a mut W {
        self.variant(BODW::NO_BOD_RESET_DETECTE)
    }
    #[doc = "BOD reset detected. Writing a one clears this reset."]
    #[inline]
    pub fn bod_reset_detected_(self) -> &'a mut W {
        self.variant(BODW::BOD_RESET_DETECTED_)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYSRST`"]
pub enum SYSRSTW {
    #[doc = "No System reset detected"]
    NO_SYSTEM_RESET_DETE,
    #[doc = "System reset detected. Writing a one clears this reset."]
    SYSTEM_RESET_DETECTE,
}
impl SYSRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSRSTW::NO_SYSTEM_RESET_DETE => false,
            SYSRSTW::SYSTEM_RESET_DETECTE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No System reset detected"]
    #[inline]
    pub fn no_system_reset_dete(self) -> &'a mut W {
        self.variant(SYSRSTW::NO_SYSTEM_RESET_DETE)
    }
    #[doc = "System reset detected. Writing a one clears this reset."]
    #[inline]
    pub fn system_reset_detecte(self) -> &'a mut W {
        self.variant(SYSRSTW::SYSTEM_RESET_DETECTE)
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - POR reset status"]
    #[inline]
    pub fn por(&self) -> PORR {
        PORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - External reset status"]
    #[inline]
    pub fn extrst(&self) -> EXTRSTR {
        EXTRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Status of the Watchdog reset"]
    #[inline]
    pub fn wdt(&self) -> WDTR {
        WDTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset"]
    #[inline]
    pub fn bod(&self) -> BODR {
        BODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Status of the software system reset"]
    #[inline]
    pub fn sysrst(&self) -> SYSRSTR {
        SYSRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - POR reset status"]
    #[inline]
    pub fn por(&mut self) -> _PORW {
        _PORW { w: self }
    }
    #[doc = "Bit 1 - External reset status"]
    #[inline]
    pub fn extrst(&mut self) -> _EXTRSTW {
        _EXTRSTW { w: self }
    }
    #[doc = "Bit 2 - Status of the Watchdog reset"]
    #[inline]
    pub fn wdt(&mut self) -> _WDTW {
        _WDTW { w: self }
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset"]
    #[inline]
    pub fn bod(&mut self) -> _BODW {
        _BODW { w: self }
    }
    #[doc = "Bit 4 - Status of the software system reset"]
    #[inline]
    pub fn sysrst(&mut self) -> _SYSRSTW {
        _SYSRSTW { w: self }
    }
}
