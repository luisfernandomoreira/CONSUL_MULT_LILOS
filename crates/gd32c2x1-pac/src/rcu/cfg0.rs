#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "Field `SCS` reader - System clock switch"]
pub type ScsR = crate::FieldReader;
#[doc = "Field `SCS` writer - System clock switch"]
pub type ScsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCSS` reader - System clock switch status"]
pub type ScssR = crate::FieldReader;
#[doc = "Field `AHBPSC` reader - AHB prescaler selection"]
pub type AhbpscR = crate::FieldReader;
#[doc = "Field `AHBPSC` writer - AHB prescaler selection"]
pub type AhbpscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `APBPSC` reader - APB prescaler selection"]
pub type ApbpscR = crate::FieldReader;
#[doc = "Field `APBPSC` writer - APB prescaler selection"]
pub type ApbpscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKOUT1SEL` reader - CK_OUT1 clock source selection"]
pub type Ckout1selR = crate::FieldReader;
#[doc = "Field `CKOUT1SEL` writer - CK_OUT1 clock source selection"]
pub type Ckout1selW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKOUT1DIV` reader - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced,"]
pub type Ckout1divR = crate::FieldReader;
#[doc = "Field `CKOUT1DIV` writer - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced,"]
pub type Ckout1divW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKOUT0SEL` reader - CK_OUT0 clock source selection"]
pub type Ckout0selR = crate::FieldReader;
#[doc = "Field `CKOUT0SEL` writer - CK_OUT0 clock source selection"]
pub type Ckout0selW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKOUT0DIV` reader - The CK_OUT0 divider which the CK_OUT0 frequency can be reduced,"]
pub type Ckout0divR = crate::FieldReader;
#[doc = "Field `CKOUT0DIV` writer - The CK_OUT0 divider which the CK_OUT0 frequency can be reduced,"]
pub type Ckout0divW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&self) -> ScsR {
        ScsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn scss(&self) -> ScssR {
        ScssR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&self) -> AhbpscR {
        AhbpscR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - APB prescaler selection"]
    #[inline(always)]
    pub fn apbpsc(&self) -> ApbpscR {
        ApbpscR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 16:18 - CK_OUT1 clock source selection"]
    #[inline(always)]
    pub fn ckout1sel(&self) -> Ckout1selR {
        Ckout1selR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced,"]
    #[inline(always)]
    pub fn ckout1div(&self) -> Ckout1divR {
        Ckout1divR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - CK_OUT0 clock source selection"]
    #[inline(always)]
    pub fn ckout0sel(&self) -> Ckout0selR {
        Ckout0selR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - The CK_OUT0 divider which the CK_OUT0 frequency can be reduced,"]
    #[inline(always)]
    pub fn ckout0div(&self) -> Ckout0divR {
        Ckout0divR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&mut self) -> ScsW<'_, Cfg0Spec> {
        ScsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&mut self) -> AhbpscW<'_, Cfg0Spec> {
        AhbpscW::new(self, 4)
    }
    #[doc = "Bits 11:13 - APB prescaler selection"]
    #[inline(always)]
    pub fn apbpsc(&mut self) -> ApbpscW<'_, Cfg0Spec> {
        ApbpscW::new(self, 11)
    }
    #[doc = "Bits 16:18 - CK_OUT1 clock source selection"]
    #[inline(always)]
    pub fn ckout1sel(&mut self) -> Ckout1selW<'_, Cfg0Spec> {
        Ckout1selW::new(self, 16)
    }
    #[doc = "Bits 20:22 - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced,"]
    #[inline(always)]
    pub fn ckout1div(&mut self) -> Ckout1divW<'_, Cfg0Spec> {
        Ckout1divW::new(self, 20)
    }
    #[doc = "Bits 24:26 - CK_OUT0 clock source selection"]
    #[inline(always)]
    pub fn ckout0sel(&mut self) -> Ckout0selW<'_, Cfg0Spec> {
        Ckout0selW::new(self, 24)
    }
    #[doc = "Bits 28:30 - The CK_OUT0 divider which the CK_OUT0 frequency can be reduced,"]
    #[inline(always)]
    pub fn ckout0div(&mut self) -> Ckout0divW<'_, Cfg0Spec> {
        Ckout0divW::new(self, 28)
    }
}
#[doc = "Configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
