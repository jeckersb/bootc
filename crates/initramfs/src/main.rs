//! Code for bootc that goes into the initramfs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod mount;

use anyhow::Result;

use clap::Parser;
use mount::{gpt_workaround, setup_root, Args};

fn main() -> Result<()> {
    let args = Args::parse();
    gpt_workaround()?;
    setup_root(args)
}
