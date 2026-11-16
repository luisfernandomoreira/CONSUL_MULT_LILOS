#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "Field `BOOT_MODE` reader - Boot mode (Refer to Boot configuration for details)"]
pub type BootModeR = crate::FieldReader;
#[doc = "Field `BOOT_MODE` writer - Boot mode (Refer to Boot configuration for details)"]
pub type BootModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA11_RMP` reader - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
pub type Pa11RmpR = crate::BitReader;
#[doc = "Field `PA11_RMP` writer - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
pub type Pa11RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA12_RMP` reader - PA12 pin remapping"]
pub type Pa12RmpR = crate::BitReader;
#[doc = "Field `PA12_RMP` writer - PA12 pin remapping"]
pub type Pa12RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB6FMPEN` reader - I2C Fm+ mode on PB6 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pb6fmpenR = crate::BitReader;
#[doc = "Field `PB6FMPEN` writer - I2C Fm+ mode on PB6 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pb6fmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB7FMPEN` reader - I2C Fm+ mode on PB7 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pb7fmpenR = crate::BitReader;
#[doc = "Field `PB7FMPEN` writer - I2C Fm+ mode on PB7 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pb7fmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB8FMPEN` reader - I2C Fm+ mode on PB8 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pb8fmpenR = crate::BitReader;
#[doc = "Field `PB8FMPEN` writer - I2C Fm+ mode on PB8 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pb8fmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB9FMPEN` reader - I2C Fm+ mode on PB9 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pb9fmpenR = crate::BitReader;
#[doc = "Field `PB9FMPEN` writer - I2C Fm+ mode on PB9 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pb9fmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0_FMP` reader - Fast Mode Plus (FM+) enable for I2C0 (for GD32C231xx / GD32C221xx series)"]
pub type I2c0FmpR = crate::BitReader;
#[doc = "Field `I2C0_FMP` writer - Fast Mode Plus (FM+) enable for I2C0 (for GD32C231xx / GD32C221xx series)"]
pub type I2c0FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA9FMPEN` reader - I2C Fm+ mode on PA9 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pa9fmpenR = crate::BitReader;
#[doc = "Field `PA9FMPEN` writer - I2C Fm+ mode on PA9 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pa9fmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA10FMPEN` reader - I2C Fm+ mode on PA10 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pa10fmpenR = crate::BitReader;
#[doc = "Field `PA10FMPEN` writer - I2C Fm+ mode on PA10 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pa10fmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC14FMPEN` reader - I2C Fm+ mode on PC14 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pc14fmpenR = crate::BitReader;
#[doc = "Field `PC14FMPEN` writer - I2C Fm+ mode on PC14 pin enable (for GD32C231xx / GD32C221xx series)"]
pub type Pc14fmpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Boot mode (Refer to Boot configuration for details)"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BootModeR {
        BootModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
    #[inline(always)]
    pub fn pa11_rmp(&self) -> Pa11RmpR {
        Pa11RmpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PA12 pin remapping"]
    #[inline(always)]
    pub fn pa12_rmp(&self) -> Pa12RmpR {
        Pa12RmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C Fm+ mode on PB6 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pb6fmpen(&self) -> Pb6fmpenR {
        Pb6fmpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2C Fm+ mode on PB7 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pb7fmpen(&self) -> Pb7fmpenR {
        Pb7fmpenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2C Fm+ mode on PB8 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pb8fmpen(&self) -> Pb8fmpenR {
        Pb8fmpenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I2C Fm+ mode on PB9 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pb9fmpen(&self) -> Pb9fmpenR {
        Pb9fmpenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Fast Mode Plus (FM+) enable for I2C0 (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn i2c0_fmp(&self) -> I2c0FmpR {
        I2c0FmpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C Fm+ mode on PA9 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pa9fmpen(&self) -> Pa9fmpenR {
        Pa9fmpenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C Fm+ mode on PA10 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pa10fmpen(&self) -> Pa10fmpenR {
        Pa10fmpenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C Fm+ mode on PC14 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pc14fmpen(&self) -> Pc14fmpenR {
        Pc14fmpenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Boot mode (Refer to Boot configuration for details)"]
    #[inline(always)]
    pub fn boot_mode(&mut self) -> BootModeW<'_, Cfg0Spec> {
        BootModeW::new(self, 0)
    }
    #[doc = "Bit 3 - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
    #[inline(always)]
    pub fn pa11_rmp(&mut self) -> Pa11RmpW<'_, Cfg0Spec> {
        Pa11RmpW::new(self, 3)
    }
    #[doc = "Bit 4 - PA12 pin remapping"]
    #[inline(always)]
    pub fn pa12_rmp(&mut self) -> Pa12RmpW<'_, Cfg0Spec> {
        Pa12RmpW::new(self, 4)
    }
    #[doc = "Bit 16 - I2C Fm+ mode on PB6 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pb6fmpen(&mut self) -> Pb6fmpenW<'_, Cfg0Spec> {
        Pb6fmpenW::new(self, 16)
    }
    #[doc = "Bit 17 - I2C Fm+ mode on PB7 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pb7fmpen(&mut self) -> Pb7fmpenW<'_, Cfg0Spec> {
        Pb7fmpenW::new(self, 17)
    }
    #[doc = "Bit 18 - I2C Fm+ mode on PB8 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pb8fmpen(&mut self) -> Pb8fmpenW<'_, Cfg0Spec> {
        Pb8fmpenW::new(self, 18)
    }
    #[doc = "Bit 19 - I2C Fm+ mode on PB9 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pb9fmpen(&mut self) -> Pb9fmpenW<'_, Cfg0Spec> {
        Pb9fmpenW::new(self, 19)
    }
    #[doc = "Bit 20 - Fast Mode Plus (FM+) enable for I2C0 (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn i2c0_fmp(&mut self) -> I2c0FmpW<'_, Cfg0Spec> {
        I2c0FmpW::new(self, 20)
    }
    #[doc = "Bit 22 - I2C Fm+ mode on PA9 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pa9fmpen(&mut self) -> Pa9fmpenW<'_, Cfg0Spec> {
        Pa9fmpenW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C Fm+ mode on PA10 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pa10fmpen(&mut self) -> Pa10fmpenW<'_, Cfg0Spec> {
        Pa10fmpenW::new(self, 23)
    }
    #[doc = "Bit 24 - I2C Fm+ mode on PC14 pin enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn pc14fmpen(&mut self) -> Pc14fmpenW<'_, Cfg0Spec> {
        Pc14fmpenW::new(self, 24)
    }
}
#[doc = "System configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for Cfg0Spec {}
