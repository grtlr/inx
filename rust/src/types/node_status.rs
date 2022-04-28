// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, MilestoneInfo};
use crate::proto::{self, ProtocolParameters};

use bee_protocol_stardust::types::milestone_key_range::MilestoneKeyRange;

/// The [`NodeStatus`] type.
#[derive(PartialEq, Debug)]
pub struct NodeStatus {
    /// Signals if the node is healthy.
    pub is_healthy: bool,
    /// The latest milestone seen by the node.
    pub latest_milestone: MilestoneInfo,
    /// The last confirmed milestone.
    pub confirmed_milestone: MilestoneInfo,
    /// The pruning index of the node.
    pub pruning_index: u32,
    /// The ledger index of the node.
    pub ledger_index: u32,
}

/// The [`NodeConfiguration`] type.
#[derive(PartialEq, Debug)]
pub struct NodeConfiguration {
    /// The paramters of the protocol.
    pub protocol_parameters: inx::proto::ProtocolParameters, // TODO: Adapt this
    /// The number of milestone public keys.
    pub milestone_public_key_count: u32,
    /// The key ranges that are used to sign milestones.
    pub milestone_key_ranges: Vec<MilestoneKeyRange>,
    /// Information about the network's underlying token.
    pub base_token: BaseToken, 
}

/// Information about the network's underlying token.
#[derive(PartialEq, Debug)]
pub struct BaseToken {
    /// The name of the token.
    pub name: String,
    /// The ticker symbol of the token.
    pub tickerSymbol: String,
    /// The unit of the token.
    pub unit: String,
    /// The sub-unit of the token.
    pub sub_unit: String,
    /// The number of decimal places in the token.
    pub decimals: u32,
    /// Specifies if the token uses the decimal system.
    pub use_metrics_prefix: bool,
}

impl TryFrom<proto::NodeConfiguration> for NodeConfiguration {
    type Error = Error;

    fn try_from(value: proto::NodeConfiguration) -> Result<Self, Self::Error> {
        Ok(NodeConfiguration {
            protocol_parameters: value.protocol_parameters,
            milestone_public_key_count: value.milestone_public_key_count,
            milestone_key_ranges: value.milestone_key_ranges.map(Into::into).collect(),
            base_token: value.base_token,
        })
    }
}

impl From<proto::BaseToken> for BaseToken {
    fn from(value: proto::BaseToken) -> Self {
        Self {
            name: value.name,
            tickerSymbol: value.ticker_symbol,
            unit: value.unit,
            sub_unit: value.subunit,
            decimals: value.decimals,
            use_metrics_prefix: value.use_metric_prefix,
        }
    }
}

impl TryFrom<proto::NodeStatus> for NodeStatus {
    type Error = Error;

    fn try_from(value: proto::NodeStatus) -> Result<Self, Self::Error> {
        Ok(NodeStatus {
            is_healthy: value.is_healthy,
            latest_milestone: value
                .latest_milestone
                .ok_or(Error::MissingField("latest_milestone"))?
                .try_into()?,
            confirmed_milestone: value
                .confirmed_milestone
                .ok_or(Error::MissingField("confirmed_milestone"))?
                .try_into()?,
            pruning_index: value.pruning_index,
            ledger_index: value.ledger_index,
        })
    }
}
