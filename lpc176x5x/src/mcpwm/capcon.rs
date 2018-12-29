#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CAPCON {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CAP0MCI0_RER {
    bits: bool,
}
impl CAP0MCI0_RER {
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
pub struct CAP0MCI0_FER {
    bits: bool,
}
impl CAP0MCI0_FER {
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
pub struct CAP0MCI1_RER {
    bits: bool,
}
impl CAP0MCI1_RER {
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
pub struct CAP0MCI1_FER {
    bits: bool,
}
impl CAP0MCI1_FER {
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
pub struct CAP0MCI2_RER {
    bits: bool,
}
impl CAP0MCI2_RER {
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
pub struct CAP0MCI2_FER {
    bits: bool,
}
impl CAP0MCI2_FER {
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
pub struct CAP1MCI0_RER {
    bits: bool,
}
impl CAP1MCI0_RER {
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
pub struct CAP1MCI0_FER {
    bits: bool,
}
impl CAP1MCI0_FER {
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
pub struct CAP1MCI1_RER {
    bits: bool,
}
impl CAP1MCI1_RER {
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
pub struct CAP1MCI1_FER {
    bits: bool,
}
impl CAP1MCI1_FER {
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
pub struct CAP1MCI2_RER {
    bits: bool,
}
impl CAP1MCI2_RER {
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
pub struct CAP1MCI2_FER {
    bits: bool,
}
impl CAP1MCI2_FER {
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
pub struct CAP2MCI0_RER {
    bits: bool,
}
impl CAP2MCI0_RER {
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
pub struct CAP2MCI0_FER {
    bits: bool,
}
impl CAP2MCI0_FER {
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
pub struct CAP2MCI1_RER {
    bits: bool,
}
impl CAP2MCI1_RER {
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
pub struct CAP2MCI1_FER {
    bits: bool,
}
impl CAP2MCI1_FER {
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
pub struct CAP2MCI2_RER {
    bits: bool,
}
impl CAP2MCI2_RER {
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
pub struct CAP2MCI2_FER {
    bits: bool,
}
impl CAP2MCI2_FER {
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
pub struct RT0R {
    bits: bool,
}
impl RT0R {
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
pub struct RT1R {
    bits: bool,
}
impl RT1R {
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
pub struct RT2R {
    bits: bool,
}
impl RT2R {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI0."]
    #[inline]
    pub fn cap0mci0_re(&self) -> CAP0MCI0_RER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP0MCI0_RER { bits }
    }
    #[doc = "Bit 1 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI0."]
    #[inline]
    pub fn cap0mci0_fe(&self) -> CAP0MCI0_FER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP0MCI0_FER { bits }
    }
    #[doc = "Bit 2 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI1."]
    #[inline]
    pub fn cap0mci1_re(&self) -> CAP0MCI1_RER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP0MCI1_RER { bits }
    }
    #[doc = "Bit 3 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI1."]
    #[inline]
    pub fn cap0mci1_fe(&self) -> CAP0MCI1_FER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP0MCI1_FER { bits }
    }
    #[doc = "Bit 4 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI2."]
    #[inline]
    pub fn cap0mci2_re(&self) -> CAP0MCI2_RER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP0MCI2_RER { bits }
    }
    #[doc = "Bit 5 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI2."]
    #[inline]
    pub fn cap0mci2_fe(&self) -> CAP0MCI2_FER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP0MCI2_FER { bits }
    }
    #[doc = "Bit 6 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI0."]
    #[inline]
    pub fn cap1mci0_re(&self) -> CAP1MCI0_RER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP1MCI0_RER { bits }
    }
    #[doc = "Bit 7 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI0."]
    #[inline]
    pub fn cap1mci0_fe(&self) -> CAP1MCI0_FER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP1MCI0_FER { bits }
    }
    #[doc = "Bit 8 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI1."]
    #[inline]
    pub fn cap1mci1_re(&self) -> CAP1MCI1_RER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP1MCI1_RER { bits }
    }
    #[doc = "Bit 9 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI1."]
    #[inline]
    pub fn cap1mci1_fe(&self) -> CAP1MCI1_FER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP1MCI1_FER { bits }
    }
    #[doc = "Bit 10 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI2."]
    #[inline]
    pub fn cap1mci2_re(&self) -> CAP1MCI2_RER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP1MCI2_RER { bits }
    }
    #[doc = "Bit 11 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI2."]
    #[inline]
    pub fn cap1mci2_fe(&self) -> CAP1MCI2_FER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP1MCI2_FER { bits }
    }
    #[doc = "Bit 12 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI0."]
    #[inline]
    pub fn cap2mci0_re(&self) -> CAP2MCI0_RER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP2MCI0_RER { bits }
    }
    #[doc = "Bit 13 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI0."]
    #[inline]
    pub fn cap2mci0_fe(&self) -> CAP2MCI0_FER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP2MCI0_FER { bits }
    }
    #[doc = "Bit 14 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI1."]
    #[inline]
    pub fn cap2mci1_re(&self) -> CAP2MCI1_RER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP2MCI1_RER { bits }
    }
    #[doc = "Bit 15 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI1."]
    #[inline]
    pub fn cap2mci1_fe(&self) -> CAP2MCI1_FER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP2MCI1_FER { bits }
    }
    #[doc = "Bit 16 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI2."]
    #[inline]
    pub fn cap2mci2_re(&self) -> CAP2MCI2_RER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP2MCI2_RER { bits }
    }
    #[doc = "Bit 17 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI2."]
    #[inline]
    pub fn cap2mci2_fe(&self) -> CAP2MCI2_FER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAP2MCI2_FER { bits }
    }
    #[doc = "Bit 18 - If this bit is 1, TC0 is reset by a channel 0 capture event."]
    #[inline]
    pub fn rt0(&self) -> RT0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RT0R { bits }
    }
    #[doc = "Bit 19 - If this bit is 1, TC1 is reset by a channel 1 capture event."]
    #[inline]
    pub fn rt1(&self) -> RT1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RT1R { bits }
    }
    #[doc = "Bit 20 - If this bit is 1, TC2 is reset by a channel 2 capture event."]
    #[inline]
    pub fn rt2(&self) -> RT2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RT2R { bits }
    }
}
