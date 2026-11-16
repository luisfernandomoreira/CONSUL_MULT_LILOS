#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl0: Ctl0,
    _reserved1: [u8; 0x04],
    cfg0: Cfg0,
    int: Int,
    ahb1rst: Ahb1rst,
    ahb2rst: Ahb2rst,
    _reserved5: [u8; 0x0c],
    apbrst: Apbrst,
    _reserved6: [u8; 0x08],
    ahb1en: Ahb1en,
    ahb2en: Ahb2en,
    _reserved8: [u8; 0x0c],
    apben: Apben,
    _reserved9: [u8; 0x08],
    ahb1spdpen: Ahb1spdpen,
    ahb2spdpen: Ahb2spdpen,
    _reserved11: [u8; 0x0c],
    apbspdpen: Apbspdpen,
    _reserved12: [u8; 0x08],
    ctl1: Ctl1,
    rstsck: Rstsck,
    _reserved14: [u8; 0x14],
    cfg1: Cfg1,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x08 - Configuration register 0"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x0c - Interrupt register"]
    #[inline(always)]
    pub const fn int(&self) -> &Int {
        &self.int
    }
    #[doc = "0x10 - AHB1 reset register"]
    #[inline(always)]
    pub const fn ahb1rst(&self) -> &Ahb1rst {
        &self.ahb1rst
    }
    #[doc = "0x14 - AHB2 reset register"]
    #[inline(always)]
    pub const fn ahb2rst(&self) -> &Ahb2rst {
        &self.ahb2rst
    }
    #[doc = "0x24 - APB reset register"]
    #[inline(always)]
    pub const fn apbrst(&self) -> &Apbrst {
        &self.apbrst
    }
    #[doc = "0x30 - AHB1 enable register"]
    #[inline(always)]
    pub const fn ahb1en(&self) -> &Ahb1en {
        &self.ahb1en
    }
    #[doc = "0x34 - AHB2 enable register"]
    #[inline(always)]
    pub const fn ahb2en(&self) -> &Ahb2en {
        &self.ahb2en
    }
    #[doc = "0x44 - APB enable register"]
    #[inline(always)]
    pub const fn apben(&self) -> &Apben {
        &self.apben
    }
    #[doc = "0x50 - AHB1 sleep and deep sleep mode enable register"]
    #[inline(always)]
    pub const fn ahb1spdpen(&self) -> &Ahb1spdpen {
        &self.ahb1spdpen
    }
    #[doc = "0x54 - AHB2 sleep and deep sleep mode enable register"]
    #[inline(always)]
    pub const fn ahb2spdpen(&self) -> &Ahb2spdpen {
        &self.ahb2spdpen
    }
    #[doc = "0x64 - APB sleep and deep sleep mode enable register"]
    #[inline(always)]
    pub const fn apbspdpen(&self) -> &Apbspdpen {
        &self.apbspdpen
    }
    #[doc = "0x70 - Control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x74 - Reset source /clock register"]
    #[inline(always)]
    pub const fn rstsck(&self) -> &Rstsck {
        &self.rstsck
    }
    #[doc = "0x8c - Configuration register 1"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
}
#[doc = "CTL0 (rw) register accessor: Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`] module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "CFG0 (rw) register accessor: Configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`] module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "Configuration register 0"]
pub mod cfg0;
#[doc = "INT (rw) register accessor: Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`] module"]
#[doc(alias = "INT")]
pub type Int = crate::Reg<int::IntSpec>;
#[doc = "Interrupt register"]
pub mod int;
#[doc = "AHB1RST (rw) register accessor: AHB1 reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1rst`] module"]
#[doc(alias = "AHB1RST")]
pub type Ahb1rst = crate::Reg<ahb1rst::Ahb1rstSpec>;
#[doc = "AHB1 reset register"]
pub mod ahb1rst;
#[doc = "AHB2RST (rw) register accessor: AHB2 reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2rst`] module"]
#[doc(alias = "AHB2RST")]
pub type Ahb2rst = crate::Reg<ahb2rst::Ahb2rstSpec>;
#[doc = "AHB2 reset register"]
pub mod ahb2rst;
#[doc = "APBRST (rw) register accessor: APB reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrst`] module"]
#[doc(alias = "APBRST")]
pub type Apbrst = crate::Reg<apbrst::ApbrstSpec>;
#[doc = "APB reset register"]
pub mod apbrst;
#[doc = "AHB1EN (rw) register accessor: AHB1 enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1en`] module"]
#[doc(alias = "AHB1EN")]
pub type Ahb1en = crate::Reg<ahb1en::Ahb1enSpec>;
#[doc = "AHB1 enable register"]
pub mod ahb1en;
#[doc = "AHB2EN (rw) register accessor: AHB2 enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2en`] module"]
#[doc(alias = "AHB2EN")]
pub type Ahb2en = crate::Reg<ahb2en::Ahb2enSpec>;
#[doc = "AHB2 enable register"]
pub mod ahb2en;
#[doc = "APBEN (rw) register accessor: APB enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`apben::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apben::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apben`] module"]
#[doc(alias = "APBEN")]
pub type Apben = crate::Reg<apben::ApbenSpec>;
#[doc = "APB enable register"]
pub mod apben;
#[doc = "AHB1SPDPEN (rw) register accessor: AHB1 sleep and deep sleep mode enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1spdpen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1spdpen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1spdpen`] module"]
#[doc(alias = "AHB1SPDPEN")]
pub type Ahb1spdpen = crate::Reg<ahb1spdpen::Ahb1spdpenSpec>;
#[doc = "AHB1 sleep and deep sleep mode enable register"]
pub mod ahb1spdpen;
#[doc = "AHB2SPDPEN (rw) register accessor: AHB2 sleep and deep sleep mode enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2spdpen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2spdpen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2spdpen`] module"]
#[doc(alias = "AHB2SPDPEN")]
pub type Ahb2spdpen = crate::Reg<ahb2spdpen::Ahb2spdpenSpec>;
#[doc = "AHB2 sleep and deep sleep mode enable register"]
pub mod ahb2spdpen;
#[doc = "APBSPDPEN (rw) register accessor: APB sleep and deep sleep mode enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspdpen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspdpen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbspdpen`] module"]
#[doc(alias = "APBSPDPEN")]
pub type Apbspdpen = crate::Reg<apbspdpen::ApbspdpenSpec>;
#[doc = "APB sleep and deep sleep mode enable register"]
pub mod apbspdpen;
#[doc = "CTL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`] module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "RSTSCK (rw) register accessor: Reset source /clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsck`] module"]
#[doc(alias = "RSTSCK")]
pub type Rstsck = crate::Reg<rstsck::RstsckSpec>;
#[doc = "Reset source /clock register"]
pub mod rstsck;
#[doc = "CFG1 (rw) register accessor: Configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`] module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "Configuration register 1"]
pub mod cfg1;
