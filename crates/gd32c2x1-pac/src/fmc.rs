#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ws: Ws,
    _reserved1: [u8; 0x04],
    key: Key,
    obkey: Obkey,
    stat: Stat,
    ctl: Ctl,
    _reserved5: [u8; 0x08],
    obctl: Obctl,
    dcrp_saddr0: DcrpSaddr0,
    dcrp_eaddr0: DcrpEaddr0,
    wp0: Wp0,
    wp1: Wp1,
    dcrp_saddr1: DcrpSaddr1,
    dcrp_eaddr1: DcrpEaddr1,
    _reserved12: [u8; 0x44],
    scr: Scr,
}
impl RegisterBlock {
    #[doc = "0x00 - Wait state register"]
    #[inline(always)]
    pub const fn ws(&self) -> &Ws {
        &self.ws
    }
    #[doc = "0x08 - Unlock key register"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        &self.key
    }
    #[doc = "0x0c - Option byte unlock key register"]
    #[inline(always)]
    pub const fn obkey(&self) -> &Obkey {
        &self.obkey
    }
    #[doc = "0x10 - Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x14 - Control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x20 - Option byte control register"]
    #[inline(always)]
    pub const fn obctl(&self) -> &Obctl {
        &self.obctl
    }
    #[doc = "0x24 - DCRP start address register 0"]
    #[inline(always)]
    pub const fn dcrp_saddr0(&self) -> &DcrpSaddr0 {
        &self.dcrp_saddr0
    }
    #[doc = "0x28 - DCRP end address register"]
    #[inline(always)]
    pub const fn dcrp_eaddr0(&self) -> &DcrpEaddr0 {
        &self.dcrp_eaddr0
    }
    #[doc = "0x2c - Write protection area 0 register"]
    #[inline(always)]
    pub const fn wp0(&self) -> &Wp0 {
        &self.wp0
    }
    #[doc = "0x30 - Write protection area 1 register"]
    #[inline(always)]
    pub const fn wp1(&self) -> &Wp1 {
        &self.wp1
    }
    #[doc = "0x34 - DCRP start address register"]
    #[inline(always)]
    pub const fn dcrp_saddr1(&self) -> &DcrpSaddr1 {
        &self.dcrp_saddr1
    }
    #[doc = "0x38 - DCRP end address register"]
    #[inline(always)]
    pub const fn dcrp_eaddr1(&self) -> &DcrpEaddr1 {
        &self.dcrp_eaddr1
    }
    #[doc = "0x80 - Secure user area register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
}
#[doc = "WS (rw) register accessor: Wait state register\n\nYou can [`read`](crate::Reg::read) this register and get [`ws::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ws::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ws`] module"]
#[doc(alias = "WS")]
pub type Ws = crate::Reg<ws::WsSpec>;
#[doc = "Wait state register"]
pub mod ws;
#[doc = "KEY (w) register accessor: Unlock key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`] module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "Unlock key register"]
pub mod key;
#[doc = "OBKEY (w) register accessor: Option byte unlock key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obkey`] module"]
#[doc(alias = "OBKEY")]
pub type Obkey = crate::Reg<obkey::ObkeySpec>;
#[doc = "Option byte unlock key register"]
pub mod obkey;
#[doc = "STAT (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register"]
pub mod stat;
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`] module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "OBCTL (rw) register accessor: Option byte control register\n\nYou can [`read`](crate::Reg::read) this register and get [`obctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obctl`] module"]
#[doc(alias = "OBCTL")]
pub type Obctl = crate::Reg<obctl::ObctlSpec>;
#[doc = "Option byte control register"]
pub mod obctl;
#[doc = "DCRP_SADDR0 (rw) register accessor: DCRP start address register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrp_saddr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrp_saddr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrp_saddr0`] module"]
#[doc(alias = "DCRP_SADDR0")]
pub type DcrpSaddr0 = crate::Reg<dcrp_saddr0::DcrpSaddr0Spec>;
#[doc = "DCRP start address register 0"]
pub mod dcrp_saddr0;
#[doc = "DCRP_EADDR0 (rw) register accessor: DCRP end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrp_eaddr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrp_eaddr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrp_eaddr0`] module"]
#[doc(alias = "DCRP_EADDR0")]
pub type DcrpEaddr0 = crate::Reg<dcrp_eaddr0::DcrpEaddr0Spec>;
#[doc = "DCRP end address register"]
pub mod dcrp_eaddr0;
#[doc = "WP0 (rw) register accessor: Write protection area 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`wp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wp0`] module"]
#[doc(alias = "WP0")]
pub type Wp0 = crate::Reg<wp0::Wp0Spec>;
#[doc = "Write protection area 0 register"]
pub mod wp0;
#[doc = "WP1 (rw) register accessor: Write protection area 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`wp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wp1`] module"]
#[doc(alias = "WP1")]
pub type Wp1 = crate::Reg<wp1::Wp1Spec>;
#[doc = "Write protection area 1 register"]
pub mod wp1;
#[doc = "DCRP_SADDR1 (rw) register accessor: DCRP start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrp_saddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrp_saddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrp_saddr1`] module"]
#[doc(alias = "DCRP_SADDR1")]
pub type DcrpSaddr1 = crate::Reg<dcrp_saddr1::DcrpSaddr1Spec>;
#[doc = "DCRP start address register"]
pub mod dcrp_saddr1;
#[doc = "DCRP_EADDR1 (rw) register accessor: DCRP end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrp_eaddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrp_eaddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrp_eaddr1`] module"]
#[doc(alias = "DCRP_EADDR1")]
pub type DcrpEaddr1 = crate::Reg<dcrp_eaddr1::DcrpEaddr1Spec>;
#[doc = "DCRP end address register"]
pub mod dcrp_eaddr1;
#[doc = "SCR (rw) register accessor: Secure user area register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Secure user area register"]
pub mod scr;
