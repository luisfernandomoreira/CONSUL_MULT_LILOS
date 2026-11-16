#[doc = "Register `APBEN` reader"]
pub type R = crate::R<ApbenSpec>;
#[doc = "Register `APBEN` writer"]
pub type W = crate::W<ApbenSpec>;
#[doc = "Field `SYSCFGEN` reader - System configuration clock enable"]
pub type SyscfgenR = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - System configuration clock enable"]
pub type SyscfgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEN` reader - Comparator clock enable"]
pub type CmpenR = crate::BitReader;
#[doc = "Field `CMPEN` writer - Comparator clock enable"]
pub type CmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGTEN` reader - WWDGT clock enable"]
pub type WwdgtenR = crate::BitReader;
#[doc = "Field `WWDGTEN` writer - WWDGT clock enable"]
pub type WwdgtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCEN` reader - ADC interface clock enable"]
pub type AdcenR = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADC interface clock enable"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0EN` reader - TIMER0 timer clock enable"]
pub type Timer0enR = crate::BitReader;
#[doc = "Field `TIMER0EN` writer - TIMER0 timer clock enable"]
pub type Timer0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2EN` reader - TIMER2 timer clock enable"]
pub type Timer2enR = crate::BitReader;
#[doc = "Field `TIMER2EN` writer - TIMER2 timer clock enable"]
pub type Timer2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0EN` reader - SPI0 clock enable"]
pub type Spi0enR = crate::BitReader;
#[doc = "Field `SPI0EN` writer - SPI0 clock enable"]
pub type Spi0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type Spi1enR = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0EN` reader - USART0 clock enable"]
pub type Usart0enR = crate::BitReader;
#[doc = "Field `USART0EN` writer - USART0 clock enable"]
pub type Usart0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type Usart1enR = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type Usart1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER13EN` reader - TIMER13 timer clock enable"]
pub type Timer13enR = crate::BitReader;
#[doc = "Field `TIMER13EN` writer - TIMER13 timer clock enable"]
pub type Timer13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER15EN` reader - TIMER15 timer clock enable"]
pub type Timer15enR = crate::BitReader;
#[doc = "Field `TIMER15EN` writer - TIMER15 timer clock enable"]
pub type Timer15enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER16EN` reader - TIMER16 timer clock enable"]
pub type Timer16enR = crate::BitReader;
#[doc = "Field `TIMER16EN` writer - TIMER16 timer clock enable"]
pub type Timer16enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
pub type Usart2enR = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
pub type Usart2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0EN` reader - I2C0 clock enable"]
pub type I2c0enR = crate::BitReader;
#[doc = "Field `I2C0EN` writer - I2C0 clock enable"]
pub type I2c0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub type I2c1enR = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub type I2c1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGEN` reader - DBG clock enable"]
pub type DbgenR = crate::BitReader;
#[doc = "Field `DBGEN` writer - DBG clock enable"]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMUEN` reader - PMU clock enable"]
pub type PmuenR = crate::BitReader;
#[doc = "Field `PMUEN` writer - PMU clock enable"]
pub type PmuenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System configuration clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SyscfgenR {
        SyscfgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator clock enable"]
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - WWDGT clock enable"]
    #[inline(always)]
    pub fn wwdgten(&self) -> WwdgtenR {
        WwdgtenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TIMER0 timer clock enable"]
    #[inline(always)]
    pub fn timer0en(&self) -> Timer0enR {
        Timer0enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER2 timer clock enable"]
    #[inline(always)]
    pub fn timer2en(&self) -> Timer2enR {
        Timer2enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    pub fn spi0en(&self) -> Spi0enR {
        Spi0enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    pub fn usart0en(&self) -> Usart0enR {
        Usart0enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER13 timer clock enable"]
    #[inline(always)]
    pub fn timer13en(&self) -> Timer13enR {
        Timer13enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER15 timer clock enable"]
    #[inline(always)]
    pub fn timer15en(&self) -> Timer15enR {
        Timer15enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER16 timer clock enable"]
    #[inline(always)]
    pub fn timer16en(&self) -> Timer16enR {
        Timer16enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> Usart2enR {
        Usart2enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    pub fn i2c0en(&self) -> I2c0enR {
        I2c0enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - DBG clock enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PMU clock enable"]
    #[inline(always)]
    pub fn pmuen(&self) -> PmuenR {
        PmuenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SyscfgenW<'_, ApbenSpec> {
        SyscfgenW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator clock enable"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CmpenW<'_, ApbenSpec> {
        CmpenW::new(self, 1)
    }
    #[doc = "Bit 8 - WWDGT clock enable"]
    #[inline(always)]
    pub fn wwdgten(&mut self) -> WwdgtenW<'_, ApbenSpec> {
        WwdgtenW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> AdcenW<'_, ApbenSpec> {
        AdcenW::new(self, 9)
    }
    #[doc = "Bit 10 - TIMER0 timer clock enable"]
    #[inline(always)]
    pub fn timer0en(&mut self) -> Timer0enW<'_, ApbenSpec> {
        Timer0enW::new(self, 10)
    }
    #[doc = "Bit 11 - TIMER2 timer clock enable"]
    #[inline(always)]
    pub fn timer2en(&mut self) -> Timer2enW<'_, ApbenSpec> {
        Timer2enW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    pub fn spi0en(&mut self) -> Spi0enW<'_, ApbenSpec> {
        Spi0enW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> Spi1enW<'_, ApbenSpec> {
        Spi1enW::new(self, 13)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    pub fn usart0en(&mut self) -> Usart0enW<'_, ApbenSpec> {
        Usart0enW::new(self, 14)
    }
    #[doc = "Bit 15 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> Usart1enW<'_, ApbenSpec> {
        Usart1enW::new(self, 15)
    }
    #[doc = "Bit 16 - TIMER13 timer clock enable"]
    #[inline(always)]
    pub fn timer13en(&mut self) -> Timer13enW<'_, ApbenSpec> {
        Timer13enW::new(self, 16)
    }
    #[doc = "Bit 17 - TIMER15 timer clock enable"]
    #[inline(always)]
    pub fn timer15en(&mut self) -> Timer15enW<'_, ApbenSpec> {
        Timer15enW::new(self, 17)
    }
    #[doc = "Bit 18 - TIMER16 timer clock enable"]
    #[inline(always)]
    pub fn timer16en(&mut self) -> Timer16enW<'_, ApbenSpec> {
        Timer16enW::new(self, 18)
    }
    #[doc = "Bit 19 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> Usart2enW<'_, ApbenSpec> {
        Usart2enW::new(self, 19)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    pub fn i2c0en(&mut self) -> I2c0enW<'_, ApbenSpec> {
        I2c0enW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2c1enW<'_, ApbenSpec> {
        I2c1enW::new(self, 22)
    }
    #[doc = "Bit 27 - DBG clock enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DbgenW<'_, ApbenSpec> {
        DbgenW::new(self, 27)
    }
    #[doc = "Bit 28 - PMU clock enable"]
    #[inline(always)]
    pub fn pmuen(&mut self) -> PmuenW<'_, ApbenSpec> {
        PmuenW::new(self, 28)
    }
}
#[doc = "APB enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`apben::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apben::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbenSpec;
impl crate::RegisterSpec for ApbenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apben::R`](R) reader structure"]
impl crate::Readable for ApbenSpec {}
#[doc = "`write(|w| ..)` method takes [`apben::W`](W) writer structure"]
impl crate::Writable for ApbenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBEN to value 0"]
impl crate::Resettable for ApbenSpec {}
