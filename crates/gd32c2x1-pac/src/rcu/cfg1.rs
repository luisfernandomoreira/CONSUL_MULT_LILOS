#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `USART0SEL` reader - CK_USART0 clock source selection"]
pub type Usart0selR = crate::FieldReader;
#[doc = "Field `USART0SEL` writer - CK_USART0 clock source selection"]
pub type Usart0selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C0SEL` reader - CK_I2C0 clock source selection"]
pub type I2c0selR = crate::FieldReader;
#[doc = "Field `I2C0SEL` writer - CK_I2C0 clock source selection"]
pub type I2c0selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C1SEL` reader - CK_I2C1 clock source selection"]
pub type I2c1selR = crate::FieldReader;
#[doc = "Field `I2C1SEL` writer - CK_I2C1 clock source selection"]
pub type I2c1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCSEL` reader - CK_ADC clock source selection"]
pub type AdcselR = crate::BitReader;
#[doc = "Field `ADCSEL` writer - CK_ADC clock source selection"]
pub type AdcselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPSC` reader - ADC clock prescaler selection"]
pub type AdcpscR = crate::FieldReader;
#[doc = "Field `ADCPSC` writer - ADC clock prescaler selection"]
pub type AdcpscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2SSEL` reader - I2S clock source selection"]
pub type I2sselR = crate::FieldReader;
#[doc = "Field `I2SSEL` writer - I2S clock source selection"]
pub type I2sselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - CK_USART0 clock source selection"]
    #[inline(always)]
    pub fn usart0sel(&self) -> Usart0selR {
        Usart0selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CK_I2C0 clock source selection"]
    #[inline(always)]
    pub fn i2c0sel(&self) -> I2c0selR {
        I2c0selR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CK_I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2c1selR {
        I2c1selR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - CK_ADC clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> AdcselR {
        AdcselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - ADC clock prescaler selection"]
    #[inline(always)]
    pub fn adcpsc(&self) -> AdcpscR {
        AdcpscR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - I2S clock source selection"]
    #[inline(always)]
    pub fn i2ssel(&self) -> I2sselR {
        I2sselR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CK_USART0 clock source selection"]
    #[inline(always)]
    pub fn usart0sel(&mut self) -> Usart0selW<'_, Cfg1Spec> {
        Usart0selW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CK_I2C0 clock source selection"]
    #[inline(always)]
    pub fn i2c0sel(&mut self) -> I2c0selW<'_, Cfg1Spec> {
        I2c0selW::new(self, 2)
    }
    #[doc = "Bits 4:5 - CK_I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2c1selW<'_, Cfg1Spec> {
        I2c1selW::new(self, 4)
    }
    #[doc = "Bit 8 - CK_ADC clock source selection"]
    #[inline(always)]
    pub fn adcsel(&mut self) -> AdcselW<'_, Cfg1Spec> {
        AdcselW::new(self, 8)
    }
    #[doc = "Bits 9:12 - ADC clock prescaler selection"]
    #[inline(always)]
    pub fn adcpsc(&mut self) -> AdcpscW<'_, Cfg1Spec> {
        AdcpscW::new(self, 9)
    }
    #[doc = "Bits 14:15 - I2S clock source selection"]
    #[inline(always)]
    pub fn i2ssel(&mut self) -> I2sselW<'_, Cfg1Spec> {
        I2sselW::new(self, 14)
    }
}
#[doc = "Configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {}
