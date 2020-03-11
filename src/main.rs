// -*- mode: rust; -*-
//
// This file is part of subhd/solo, substrate.
// Copyright (c) 2017-2019 Chester Li and extropies.com
// See LICENSE for licensing information.
//
// Authors:
// - Chester Li<chester@lichester.com>

//! C API to RUST for solo
extern {
    fn usend( data:*const u8, len:usize ) -> u32;
    fn uget( data:*const u8, len:usize ) -> u32;
}

fn main(){
    
}