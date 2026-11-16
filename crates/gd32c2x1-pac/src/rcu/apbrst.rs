#[doc = "Register `APBRST` reader"]
pub type R = crate::R<ApbrstSpec>;
#[doc = "Register `APBRST` writer"]
pub type W = crate::W<ApbrstSpec>;
#[doc = "Field `SYSCFGRST` reader - System configuration reset"]
pub type SyscfgrstR = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - System configuration reset"]
pub type SyscfgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPRST` reader - Comparator reset"]
pub type CmprstR = crate::BitReader;
#[doc = "Field `CMPRST` writer - Comparator reset"]
pub type CmprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGTRST` reader - WWDGT reset"]
pub type WwdgtrstR = crate::BitReader;
#[doc = "Field `WWDGTRST` writer - WWDGT reset"]
pub type WwdgtrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type AdcrstR = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type AdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0RST` reader - TIMER0 reset"]
pub type Timer0rstR = crate::BitReader;
#[doc = "Field `TIMER0RST` writer - TIMER0 reset"]
pub type Timer0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2RST` reader - TIMER2 reset"]
pub type Timer2rstR = crate::BitReader;
#[doc = "Field `TIMER2RST` writer - TIMER2 reset"]
pub type Timer2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0RST` reader - SPI0 Reset"]
pub type Spi0rstR = crate::BitReader;
#[doc = "Field `SPI0RST` writer - SPI0 Reset"]
pub type Spi0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1 Reset"]
pub type Spi1rstR = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 Reset"]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0RST` reader - USART0 Reset"]
pub type Usart0rstR = crate::BitReader;
#[doc = "Field `USART0RST` writer - USART0 Reset"]
pub type Usart0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 Reset"]
pub type Usart1rstR = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 Reset"]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER13RST` reader - TIMER13 reset"]
pub type Timer13rstR = crate::BitReader;
#[doc = "Field `TIMER13RST` writer - TIMER13 reset"]
pub type Timer13rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER15RST` reader - TIMER15 reset"]
pub type Timer15rstR = crate::BitReader;
#[doc = "Field `TIMER15RST` writer - TIMER15 reset"]
pub type Timer15rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER16RST` reader - TIMER16 reset"]
pub type Timer16rstR = crate::BitReader;
#[doc = "Field `TIMER16RST` writer - TIMER16 reset"]
pub type Timer16rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2RST` reader - USART2 Reset"]
pub type Usart2rstR = crate::BitReader;
#[doc = "Field `USART2RST` writer - USART2 Reset"]
pub type Usart2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0RST` reader - I2C0 reset"]
pub type I2c0rstR = crate::BitReader;
#[doc = "Field `I2C0RST` writer - I2C0 reset"]
pub type I2c0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2c1rstR = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2c1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMURST` reader - PMU reset"]
pub type PmurstR = crate::BitReader;
#[doc = "Field `PMURST` writer - PMU reset"]
pub type PmurstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System configuration reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SyscfgrstR {
        SyscfgrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator reset"]
    #[inline(always)]
    pub fn cmprst(&self) -> CmprstR {
        CmprstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - WWDGT reset"]
    #[inline(always)]
    pub fn wwdgtrst(&self) -> WwdgtrstR {
        WwdgtrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TIMER0 reset"]
    #[inline(always)]
    pub fn timer0rst(&self) -> Timer0rstR {
        Timer0rstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER2 reset"]
    #[inline(always)]
    pub fn timer2rst(&self) -> Timer2rstR {
        Timer2rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    pub fn spi0rst(&self) -> Spi0rstR {
        Spi0rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI1 Reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    pub fn usart0rst(&self) -> Usart0rstR {
        Usart0rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USART1 Reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER13 reset"]
    #[inline(always)]
    pub fn timer13rst(&self) -> Timer13rstR {
        Timer13rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER15 reset"]
    #[inline(always)]
    pub fn timer15rst(&self) -> Timer15rstR {
        Timer15rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER16 reset"]
    #[inline(always)]
    pub fn timer16rst(&self) -> Timer16rstR {
        Timer16rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART2 Reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> Usart2rstR {
        Usart2rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 reset"]
    #[inline(always)]
    pub fn i2c0rst(&self) -> I2c0rstR {
        I2c0rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2c1rstR {
        I2c1rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - PMU reset"]
    #[inline(always)]
    pub fn pmurst(&self) -> PmurstR {
        PmurstR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration reset"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SyscfgrstW<'_, ApbrstSpec> {
        SyscfgrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator reset"]
    #[inline(always)]
    pub fn cmprst(&mut self) -> CmprstW<'_, ApbrstSpec> {
        CmprstW::new(self, 1)
    }
    #[doc = "Bit 8 - WWDGT reset"]
    #[inline(always)]
    pub fn wwdgtrst(&mut self) -> WwdgtrstW<'_, ApbrstSpec> {
        WwdgtrstW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> AdcrstW<'_, ApbrstSpec> {
        AdcrstW::new(self, 9)
    }
    #[doc = "Bit 10 - TIMER0 reset"]
    #[inline(always)]
    pub fn timer0rst(&mut self) -> Timer0rstW<'_, ApbrstSpec> {
        Timer0rstW::new(self, 10)
    }
    #[doc = "Bit 11 - TIMER2 reset"]
    #[inline(always)]
    pub fn timer2rst(&mut self) -> Timer2rstW<'_, ApbrstSpec> {
        Timer2rstW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    pub fn spi0rst(&mut self) -> Spi0rstW<'_, ApbrstSpec> {
        Spi0rstW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI1 Reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> Spi1rstW<'_, ApbrstSpec> {
        Spi1rstW::new(self, 13)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    pub fn usart0rst(&mut self) -> Usart0rstW<'_, ApbrstSpec> {
        Usart0rstW::new(self, 14)
    }
    #[doc = "Bit 15 - USART1 Reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> Usart1rstW<'_, ApbrstSpec> {
        Usart1rstW::new(self, 15)
    }
    #[doc = "Bit 16 - TIMER13 reset"]
    #[inline(always)]
    pub fn timer13rst(&mut self) -> Timer13rstW<'_, ApbrstSpec> {
        Timer13rstW::new(self, 16)
    }
    #[doc = "Bit 17 - TIMER15 reset"]
    #[inline(always)]
    pub fn timer15rst(&mut self) -> Timer15rstW<'_, ApbrstSpec> {
        Timer15rstW::new(self, 17)
    }
    #[doc = "Bit 18 - TIMER16 reset"]
    #[inline(always)]
    pub fn timer16rst(&mut self) -> Timer16rstW<'_, ApbrstSpec> {
        Timer16rstW::new(self, 18)
    }
    #[doc = "Bit 19 - USART2 Reset"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> Usart2rstW<'_, ApbrstSpec> {
        Usart2rstW::new(self, 19)
    }
    #[doc = "Bit 21 - I2C0 reset"]
    #[inline(always)]
    pub fn i2c0rst(&mut self) -> I2c0rstW<'_, ApbrstSpec> {
        I2c0rstW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2c1rstW<'_, ApbrstSpec> {
        I2c1rstW::new(self, 22)
    }
    #[doc = "Bit 28 - PMU reset"]
    #[inline(always)]
    pub fn pmurst(&mut self) -> PmurstW<'_, ApbrstSpec> {
        PmurstW::new(self, 28)
    }
}
#[doc = "APB reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbrstSpec;
impl crate::RegisterSpec for ApbrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrst::R`](R) reader structure"]
impl crate::Readable for ApbrstSpec {}
#[doc = "`write(|w| ..)` method takes [`apbrst::W`](W) writer structure"]
impl crate::Writable for ApbrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBRST to value 0"]
impl crate::Resettable for ApbrstSpec {}
