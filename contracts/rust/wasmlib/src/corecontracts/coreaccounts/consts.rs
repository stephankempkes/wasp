// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

// @formatter:off

#![allow(dead_code)]

use crate::*;

pub const SC_NAME:        &str = "accounts";
pub const SC_DESCRIPTION: &str = "Core chain account ledger contract";
pub const HSC_NAME:       ScHname = ScHname(0x3c4b5e02);

pub const PARAM_AGENT_ID: &str = "a";

pub const FUNC_DEPOSIT:      &str = "deposit";
pub const FUNC_WITHDRAW:     &str = "withdraw";
pub const VIEW_ACCOUNTS:     &str = "accounts";
pub const VIEW_BALANCE:      &str = "balance";
pub const VIEW_TOTAL_ASSETS: &str = "totalAssets";

pub const HFUNC_DEPOSIT:      ScHname = ScHname(0xbdc9102d);
pub const HFUNC_WITHDRAW:     ScHname = ScHname(0x9dcc0f41);
pub const HVIEW_ACCOUNTS:     ScHname = ScHname(0x3c4b5e02);
pub const HVIEW_BALANCE:      ScHname = ScHname(0x84168cb4);
pub const HVIEW_TOTAL_ASSETS: ScHname = ScHname(0xfab0f8d2);

// @formatter:on
