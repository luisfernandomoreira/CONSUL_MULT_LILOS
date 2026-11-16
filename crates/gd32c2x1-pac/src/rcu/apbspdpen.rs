#[doc = "Register `APBSPDPEN` reader"]
pub type R = crate::R<ApbspdpenSpec>;
#[doc = "Register `APBSPDPEN` writer"]
pub type W = crate::W<ApbspdpenSpec>;
#[doc = "Field `SYSCFGSPDPEN` reader - System configuration clock enable when sleep and deep sleep mode"]
pub type SyscfgspdpenR = crate::BitReader;
#[doc = "Field `SYSCFGSPDPEN` writer - System configuration clock enable when sleep and deep sleep mode"]
pub type SyscfgspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPSPDPEN` reader - CMP clock enable when sleep and deep sleep mode"]
pub type CmpspdpenR = crate::BitReader;
#[doc = "Field `CMPSPDPEN` writer - CMP clock enable when sleep and deep sleep mode"]
pub type CmpspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGTSPDPEN` reader - WWDGT clock enable when sleep and deep sleep mode"]
pub type WwdgtspdpenR = crate::BitReader;
#[doc = "Field `WWDGTSPDPEN` writer - WWDGT clock enable when sleep and deep sleep mode"]
pub type WwdgtspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSPDPEN` reader - ADC clock enable when sleep and deep sleep mode"]
pub type AdcspdpenR = crate::BitReader;
#[doc = "Field `ADCSPDPEN` writer - ADC clock enable when sleep and deep sleep mode"]
pub type AdcspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0SPDPEN` reader - TIMER0 clock enable when sleep and deep sleep mode"]
pub type Timer0spdpenR = crate::BitReader;
#[doc = "Field `TIMER0SPDPEN` writer - TIMER0 clock enable when sleep and deep sleep mode"]
pub type Timer0spdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2SPDPEN` reader - TIMER2 clock enable when sleep and deep sleep mode"]
pub type Timer2spdpenR = crate::BitReader;
#[doc = "Field `TIMER2SPDPEN` writer - TIMER2 clock enable when sleep and deep sleep mode"]
pub type Timer2spdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0SPDPEN` reader - SPI0 clock enable when sleep and deep sleep mode"]
pub type Spi0spdpenR = crate::BitReader;
#[doc = "Field `SPI0SPDPEN` writer - SPI0 clock enable when sleep and deep sleep mode"]
pub type Spi0spdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1SPDPEN` reader - SPI1 clock enable when sleep and deep sleep mode"]
pub type Spi1spdpenR = crate::BitReader;
#[doc = "Field `SPI1SPDPEN` writer - SPI1 clock enable when sleep and deep sleep mode"]
pub type Spi1spdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0SPDPEN` reader - USART0 clock enable when sleep and deep sleep mode"]
pub type Usart0spdpenR = crate::BitReader;
#[doc = "Field `USART0SPDPEN` writer - USART0 clock enable when sleep and deep sleep mode"]
pub type Usart0spdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1SPDPEN` reader - USART1 clock enable when sleep and deep sleep mode"]
pub type Usart1spdpenR = crate::BitReader;
#[doc = "Field `USART1SPDPEN` writer - USART1 clock enable when sleep and deep sleep mode"]
pub type Usart1spdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER13SPDPEN` reader - TIMER13 clock enable when sleep and deep sleep mode"]
pub type Timer13spdpenR = crate::BitReader;
#[doc = "Field `TIMER13SPDPEN` writer - TIMER13 clock enable when sleep and deep sleep mode"]
pub type Timer13spdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER15SPDPEN` reader - TIMER15 clock enable when sleep and deep sleep mode"]
pub type Timer15spdpenR = crate::BitReader;
#[doc = "Field `TIMER15SPDPEN` writer - TIMER15 clock enable when sleep and deep sleep mode"]
pub type Timer15spdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER16SPDPEN` reader - TIMER16 clock enable when sleep and deep sleep mode"]
pub type Timer16spdpenR = crate::BitReader;
#[doc = "Field `TIMER16SPDPEN` writer - TIMER16 clock enable when sleep and deep sleep mode"]
pub type Timer16spdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2SPDPEN` reader - USART2 clock enable when sleep mode and deep sleep mode"]
pub type Usart2spdpenR = crate::BitReader;
#[doc = "Field `USART2SPDPEN` writer - USART2 clock enable when sleep mode and deep sleep mode"]
pub type Usart2spdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0SPDPEN` reader - I2C0 clock enable when sleep mode and deep sleep mode"]
pub type I2c0spdpenR = crate::BitReader;
#[doc = "Field `I2C0SPDPEN` writer - I2C0 clock enable when sleep mode and deep sleep mode"]
pub type I2c0spdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1SPDPEN` reader - I2C1 clock enable when sleep mode and deep sleep mode"]
pub type I2c1spdpenR = crate::BitReader;
#[doc = "Field `I2C1SPDPEN` writer - I2C1 clock enable when sleep mode and deep sleep mode"]
pub type I2c1spdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMUSPDPEN` reader - PMU clock enable when sleep and deep sleep mode"]
pub type PmuspdpenR = crate::BitReader;
#[doc = "Field `PMUSPDPEN` writer - PMU clock enable when sleep and deep sleep mode"]
pub type PmuspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System configuration clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn syscfgspdpen(&self) -> SyscfgspdpenR {
        SyscfgspdpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn cmpspdpen(&self) -> CmpspdpenR {
        CmpspdpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - WWDGT clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn wwdgtspdpen(&self) -> WwdgtspdpenR {
        WwdgtspdpenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn adcspdpen(&self) -> AdcspdpenR {
        AdcspdpenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TIMER0 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn timer0spdpen(&self) -> Timer0spdpenR {
        Timer0spdpenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER2 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn timer2spdpen(&self) -> Timer2spdpenR {
        Timer2spdpenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn spi0spdpen(&self) -> Spi0spdpenR {
        Spi0spdpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI1 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn spi1spdpen(&self) -> Spi1spdpenR {
        Spi1spdpenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn usart0spdpen(&self) -> Usart0spdpenR {
        Usart0spdpenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USART1 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn usart1spdpen(&self) -> Usart1spdpenR {
        Usart1spdpenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER13 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn timer13spdpen(&self) -> Timer13spdpenR {
        Timer13spdpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER15 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn timer15spdpen(&self) -> Timer15spdpenR {
        Timer15spdpenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER16 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn timer16spdpen(&self) -> Timer16spdpenR {
        Timer16spdpenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART2 clock enable when sleep mode and deep sleep mode"]
    #[inline(always)]
    pub fn usart2spdpen(&self) -> Usart2spdpenR {
        Usart2spdpenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 clock enable when sleep mode and deep sleep mode"]
    #[inline(always)]
    pub fn i2c0spdpen(&self) -> I2c0spdpenR {
        I2c0spdpenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 clock enable when sleep mode and deep sleep mode"]
    #[inline(always)]
    pub fn i2c1spdpen(&self) -> I2c1spdpenR {
        I2c1spdpenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - PMU clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn pmuspdpen(&self) -> PmuspdpenR {
        PmuspdpenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn syscfgspdpen(&mut self) -> SyscfgspdpenW<'_, ApbspdpenSpec> {
        SyscfgspdpenW::new(self, 0)
    }
    #[doc = "Bit 1 - CMP clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn cmpspdpen(&mut self) -> CmpspdpenW<'_, ApbspdpenSpec> {
        CmpspdpenW::new(self, 1)
    }
    #[doc = "Bit 8 - WWDGT clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn wwdgtspdpen(&mut self) -> WwdgtspdpenW<'_, ApbspdpenSpec> {
        WwdgtspdpenW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn adcspdpen(&mut self) -> AdcspdpenW<'_, ApbspdpenSpec> {
        AdcspdpenW::new(self, 9)
    }
    #[doc = "Bit 10 - TIMER0 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn timer0spdpen(&mut self) -> Timer0spdpenW<'_, ApbspdpenSpec> {
        Timer0spdpenW::new(self, 10)
    }
    #[doc = "Bit 11 - TIMER2 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn timer2spdpen(&mut self) -> Timer2spdpenW<'_, ApbspdpenSpec> {
        Timer2spdpenW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI0 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn spi0spdpen(&mut self) -> Spi0spdpenW<'_, ApbspdpenSpec> {
        Spi0spdpenW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI1 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn spi1spdpen(&mut self) -> Spi1spdpenW<'_, ApbspdpenSpec> {
        Spi1spdpenW::new(self, 13)
    }
    #[doc = "Bit 14 - USART0 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn usart0spdpen(&mut self) -> Usart0spdpenW<'_, ApbspdpenSpec> {
        Usart0spdpenW::new(self, 14)
    }
    #[doc = "Bit 15 - USART1 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn usart1spdpen(&mut self) -> Usart1spdpenW<'_, ApbspdpenSpec> {
        Usart1spdpenW::new(self, 15)
    }
    #[doc = "Bit 16 - TIMER13 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn timer13spdpen(&mut self) -> Timer13spdpenW<'_, ApbspdpenSpec> {
        Timer13spdpenW::new(self, 16)
    }
    #[doc = "Bit 17 - TIMER15 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn timer15spdpen(&mut self) -> Timer15spdpenW<'_, ApbspdpenSpec> {
        Timer15spdpenW::new(self, 17)
    }
    #[doc = "Bit 18 - TIMER16 clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn timer16spdpen(&mut self) -> Timer16spdpenW<'_, ApbspdpenSpec> {
        Timer16spdpenW::new(self, 18)
    }
    #[doc = "Bit 19 - USART2 clock enable when sleep mode and deep sleep mode"]
    #[inline(always)]
    pub fn usart2spdpen(&mut self) -> Usart2spdpenW<'_, ApbspdpenSpec> {
        Usart2spdpenW::new(self, 19)
    }
    #[doc = "Bit 21 - I2C0 clock enable when sleep mode and deep sleep mode"]
    #[inline(always)]
    pub fn i2c0spdpen(&mut self) -> I2c0spdpenW<'_, ApbspdpenSpec> {
        I2c0spdpenW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 clock enable when sleep mode and deep sleep mode"]
    #[inline(always)]
    pub fn i2c1spdpen(&mut self) -> I2c1spdpenW<'_, ApbspdpenSpec> {
        I2c1spdpenW::new(self, 22)
    }
    #[doc = "Bit 28 - PMU clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn pmuspdpen(&mut self) -> PmuspdpenW<'_, ApbspdpenSpec> {
        PmuspdpenW::new(self, 28)
    }
}
#[doc = "APB sleep and deep sleep mode enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspdpen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspdpen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbspdpenSpec;
impl crate::RegisterSpec for ApbspdpenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbspdpen::R`](R) reader structure"]
impl crate::Readable for ApbspdpenSpec {}
#[doc = "`write(|w| ..)` method takes [`apbspdpen::W`](W) writer structure"]
impl crate::Writable for ApbspdpenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBSPDPEN to value 0x106f_ff03"]
impl crate::Resettable for ApbspdpenSpec {
    const RESET_VALUE: u32 = 0x106f_ff03;
}
