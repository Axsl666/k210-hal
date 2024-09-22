//! (TODO) Hardware AES calculator (AES)
use crate::{pac::AES, sysctl};
use core::marker::PhantomData;

pub struct Aes<MODE, KLEN> {
    aes: AES,
    _mode: PhantomData<MODE>,
    _klen: PhantomData<KLEN>,
}

pub struct Ecb;

pub struct Cbc;

pub struct Gcm;

pub struct K128;

pub struct K192;

pub struct K256;

#[allow(unused)] // todo: remove
impl<MODE, KLEN> Aes<MODE, KLEN> {
    pub fn ecb128(aes: AES) -> Aes<Ecb, K128> {
        todo!()
    }

    pub fn ecb192(aes: AES) -> Aes<Ecb, K192> {
        todo!()
    }

    pub fn ecb256(aes: AES) -> Aes<Ecb, K256> {
        todo!()
    }

    pub fn cbc128(aes: AES) -> Aes<Cbc, K128> {
        todo!()
    }

    pub fn cbc192(aes: AES) -> Aes<Cbc, K192> {
        todo!()
    }

    pub fn cbc256(aes: AES) -> Aes<Cbc, K256> {
        todo!()
    }

    pub fn gcm128(aes: AES) -> Aes<Gcm, K128> {
        todo!()
    }

    pub fn gcm192(aes: AES) -> Aes<Gcm, K192> {
        todo!()
    }

    pub fn gcm256(aes: AES) -> Aes<Gcm, K256> {
        todo!()
    }
}

impl<MODE, KLEN> Aes<MODE, KLEN> {
    // todo: clock
    pub fn free(self) -> AES {
        self.aes
    }

    pub fn clk_init(&self) {
        sysctl::clk_en_peri().modify(|_r, w| w.aes_clk_en().set_bit());
        sysctl::peri_reset().modify(|_r, w| w.aes_reset().set_bit());
        sysctl::peri_reset().modify(|_r, w| w.aes_reset().clear_bit());
    }

    pub fn write_add(&self,add_data:u32){
        unsafe { self.aes.aad_data.write(|w| w.bits(add_data)); }
    }

    pub fn get_data_in_flag(&self) -> u32 {
        self.aes.data_in_flag.read().bits()
    }

    pub fn get_data_out_flag(&self) -> u32 {
        self.aes.data_out_flag.read().bits()
    }

    pub fn get_tag_in_flag(&self) -> u32 {
        self.aes.tag_in_flag.read().bits()
    }

    pub fn get_out_data(&self) -> u32 {
        self.aes.out_data.read().bits()
    }

    pub fn get_tag_chk(&self) -> u32 {
        self.aes.tag_chk.read().bits()
    }

    pub fn clear_chk_tag(&self) {
        unsafe { self.aes.tag_clear.write(|w| w.bits(0)) }
    }
}

#[allow(unused)] // todo: remove
impl<MODE, KLEN> Aes<MODE, KLEN> {
    // entrypt block in-place
    pub fn encrypt_block(&self, block: &mut [u8], key: &[u8]) {
        todo!()
    }
    // decrypt block in-place
    pub fn decrypt_block(&self, block: &mut [u8], key: &[u8]) {
        todo!()
    }
}
