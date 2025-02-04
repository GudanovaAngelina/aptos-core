// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

pub mod account;
pub mod common;
pub mod move_tool;
pub mod op;

use crate::common::types::{CliResult, Error};
use clap::Parser;

/// CLI tool for interacting with the Aptos blockchain and nodes
///
#[derive(Parser)]
#[clap(name = "aptos", author, version, propagate_version = true)]
pub enum Tool {
    #[clap(subcommand)]
    Account(account::AccountTool),
    Init(common::init::InitTool),
    #[clap(subcommand)]
    Move(move_tool::MoveTool),
    #[clap(subcommand)]
    Op(op::OpTool),
}

impl Tool {
    pub async fn execute(self) -> CliResult {
        match self {
            Tool::Account(tool) => tool.execute().await,
            Tool::Init(tool) => tool.execute().await,
            Tool::Move(tool) => tool.execute().await,
            Tool::Op(tool) => tool.execute().await,
        }
    }
}
