#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSRSTSTAT {
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
#[doc = "Possible values of the field `POR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR {
    #[doc = "No POR detected"]
    NO_POR_DETECTED,
    #[doc = "POR detected"]
    POR_DETECTED,
}
impl crate::ToBits<bool> for PORR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PORR::NO_POR_DETECTED => false,
            PORR::POR_DETECTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type POR_R = crate::FR<bool, PORR>;
impl POR_R {
    #[doc = "Checks if the value of the field is `NO_POR_DETECTED`"]
    #[inline(always)]
    pub fn is_no_por_detected(&self) -> bool {
        *self == PORR::NO_POR_DETECTED
    }
    #[doc = "Checks if the value of the field is `POR_DETECTED`"]
    #[inline(always)]
    pub fn is_por_detected(&self) -> bool {
        *self == PORR::POR_DETECTED
    }
}
#[doc = "Values that can be written to the field `POR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORW {
    #[doc = "No POR detected"]
    NO_POR_DETECTED,
    #[doc = "POR detected"]
    POR_DETECTED,
}
impl PORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PORW::NO_POR_DETECTED => false,
            PORW::POR_DETECTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PORW<'a> {
    w: &'a mut W,
}
impl<'a> _PORW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No POR detected"]
    #[inline(always)]
    pub fn no_por_detected(self) -> &'a mut W {
        self.variant(PORW::NO_POR_DETECTED)
    }
    #[doc = "POR detected"]
    #[inline(always)]
    pub fn por_detected(self) -> &'a mut W {
        self.variant(PORW::POR_DETECTED)
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
#[doc = "Possible values of the field `EXTRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRSTR {
    #[doc = "No reset event detected"]
    NO_RESET_EVENT_DETEC,
    #[doc = "Reset detected"]
    RESET_DETECTED,
}
impl crate::ToBits<bool> for EXTRSTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EXTRSTR::NO_RESET_EVENT_DETEC => false,
            EXTRSTR::RESET_DETECTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EXTRST_R = crate::FR<bool, EXTRSTR>;
impl EXTRST_R {
    #[doc = "Checks if the value of the field is `NO_RESET_EVENT_DETEC`"]
    #[inline(always)]
    pub fn is_no_reset_event_detec(&self) -> bool {
        *self == EXTRSTR::NO_RESET_EVENT_DETEC
    }
    #[doc = "Checks if the value of the field is `RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_reset_detected(&self) -> bool {
        *self == EXTRSTR::RESET_DETECTED
    }
}
#[doc = "Values that can be written to the field `EXTRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRSTW {
    #[doc = "No reset event detected"]
    NO_RESET_EVENT_DETEC,
    #[doc = "Reset detected"]
    RESET_DETECTED,
}
impl EXTRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTRSTW::NO_RESET_EVENT_DETEC => false,
            EXTRSTW::RESET_DETECTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EXTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTRSTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset event detected"]
    #[inline(always)]
    pub fn no_reset_event_detec(self) -> &'a mut W {
        self.variant(EXTRSTW::NO_RESET_EVENT_DETEC)
    }
    #[doc = "Reset detected"]
    #[inline(always)]
    pub fn reset_detected(self) -> &'a mut W {
        self.variant(EXTRSTW::RESET_DETECTED)
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
#[doc = "Possible values of the field `WDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTR {
    #[doc = "No WDT reset detected"]
    NO_WDT_RESET_DETECTE,
    #[doc = "WDT reset detected"]
    WDT_RESET_DETECTED,
}
impl crate::ToBits<bool> for WDTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WDTR::NO_WDT_RESET_DETECTE => false,
            WDTR::WDT_RESET_DETECTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WDT_R = crate::FR<bool, WDTR>;
impl WDT_R {
    #[doc = "Checks if the value of the field is `NO_WDT_RESET_DETECTE`"]
    #[inline(always)]
    pub fn is_no_wdt_reset_detecte(&self) -> bool {
        *self == WDTR::NO_WDT_RESET_DETECTE
    }
    #[doc = "Checks if the value of the field is `WDT_RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_wdt_reset_detected(&self) -> bool {
        *self == WDTR::WDT_RESET_DETECTED
    }
}
#[doc = "Values that can be written to the field `WDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTW {
    #[doc = "No WDT reset detected"]
    NO_WDT_RESET_DETECTE,
    #[doc = "WDT reset detected"]
    WDT_RESET_DETECTED,
}
impl WDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTW::NO_WDT_RESET_DETECTE => false,
            WDTW::WDT_RESET_DETECTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WDTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No WDT reset detected"]
    #[inline(always)]
    pub fn no_wdt_reset_detecte(self) -> &'a mut W {
        self.variant(WDTW::NO_WDT_RESET_DETECTE)
    }
    #[doc = "WDT reset detected"]
    #[inline(always)]
    pub fn wdt_reset_detected(self) -> &'a mut W {
        self.variant(WDTW::WDT_RESET_DETECTED)
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
#[doc = "Possible values of the field `BOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODR {
    #[doc = "No BOD reset detected"]
    NO_BOD_RESET_DETECTE,
    #[doc = "BOD reset detected"]
    BOD_RESET_DETECTED,
}
impl crate::ToBits<bool> for BODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BODR::NO_BOD_RESET_DETECTE => false,
            BODR::BOD_RESET_DETECTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BOD_R = crate::FR<bool, BODR>;
impl BOD_R {
    #[doc = "Checks if the value of the field is `NO_BOD_RESET_DETECTE`"]
    #[inline(always)]
    pub fn is_no_bod_reset_detecte(&self) -> bool {
        *self == BODR::NO_BOD_RESET_DETECTE
    }
    #[doc = "Checks if the value of the field is `BOD_RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_bod_reset_detected(&self) -> bool {
        *self == BODR::BOD_RESET_DETECTED
    }
}
#[doc = "Values that can be written to the field `BOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODW {
    #[doc = "No BOD reset detected"]
    NO_BOD_RESET_DETECTE,
    #[doc = "BOD reset detected"]
    BOD_RESET_DETECTED,
}
impl BODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BODW::NO_BOD_RESET_DETECTE => false,
            BODW::BOD_RESET_DETECTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BODW<'a> {
    w: &'a mut W,
}
impl<'a> _BODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No BOD reset detected"]
    #[inline(always)]
    pub fn no_bod_reset_detecte(self) -> &'a mut W {
        self.variant(BODW::NO_BOD_RESET_DETECTE)
    }
    #[doc = "BOD reset detected"]
    #[inline(always)]
    pub fn bod_reset_detected(self) -> &'a mut W {
        self.variant(BODW::BOD_RESET_DETECTED)
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
#[doc = "Possible values of the field `SYSRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRSTR {
    #[doc = "No System reset detected"]
    NO_SYSTEM_RESET_DETE,
    #[doc = "System reset detected"]
    SYSTEM_RESET_DETECTE,
}
impl crate::ToBits<bool> for SYSRSTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SYSRSTR::NO_SYSTEM_RESET_DETE => false,
            SYSRSTR::SYSTEM_RESET_DETECTE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SYSRST_R = crate::FR<bool, SYSRSTR>;
impl SYSRST_R {
    #[doc = "Checks if the value of the field is `NO_SYSTEM_RESET_DETE`"]
    #[inline(always)]
    pub fn is_no_system_reset_dete(&self) -> bool {
        *self == SYSRSTR::NO_SYSTEM_RESET_DETE
    }
    #[doc = "Checks if the value of the field is `SYSTEM_RESET_DETECTE`"]
    #[inline(always)]
    pub fn is_system_reset_detecte(&self) -> bool {
        *self == SYSRSTR::SYSTEM_RESET_DETECTE
    }
}
#[doc = "Values that can be written to the field `SYSRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRSTW {
    #[doc = "No System reset detected"]
    NO_SYSTEM_RESET_DETE,
    #[doc = "System reset detected"]
    SYSTEM_RESET_DETECTE,
}
impl SYSRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSRSTW::NO_SYSTEM_RESET_DETE => false,
            SYSRSTW::SYSTEM_RESET_DETECTE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSRSTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No System reset detected"]
    #[inline(always)]
    pub fn no_system_reset_dete(self) -> &'a mut W {
        self.variant(SYSRSTW::NO_SYSTEM_RESET_DETE)
    }
    #[doc = "System reset detected"]
    #[inline(always)]
    pub fn system_reset_detecte(self) -> &'a mut W {
        self.variant(SYSRSTW::SYSTEM_RESET_DETECTE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - POR reset status"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of the external RESET pin"]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of the Watchdog reset"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset"]
    #[inline(always)]
    pub fn bod(&self) -> BOD_R {
        BOD_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of the software system reset"]
    #[inline(always)]
    pub fn sysrst(&self) -> SYSRST_R {
        SYSRST_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - POR reset status"]
    #[inline(always)]
    pub fn por(&mut self) -> _PORW {
        _PORW { w: self }
    }
    #[doc = "Bit 1 - Status of the external RESET pin"]
    #[inline(always)]
    pub fn extrst(&mut self) -> _EXTRSTW {
        _EXTRSTW { w: self }
    }
    #[doc = "Bit 2 - Status of the Watchdog reset"]
    #[inline(always)]
    pub fn wdt(&mut self) -> _WDTW {
        _WDTW { w: self }
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset"]
    #[inline(always)]
    pub fn bod(&mut self) -> _BODW {
        _BODW { w: self }
    }
    #[doc = "Bit 4 - Status of the software system reset"]
    #[inline(always)]
    pub fn sysrst(&mut self) -> _SYSRSTW {
        _SYSRSTW { w: self }
    }
}
