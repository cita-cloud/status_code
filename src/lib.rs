// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num_derive::FromPrimitive;

/// The response status
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive)]
pub enum Status {
    /// Success: 0
    Success = 0,
    /// Convert int to status Error
    ConvertIntError,

    /// controller error, start from 100
    /// node in misbehave list
    MisbehaveNode = 100,
    /// node in ban list
    BannedNode,
    /// message not provide address
    NoProvideAddress,
    /// not get the block
    NoBlock,
    /// proposal is none
    NoneProposal,
    /// block body is none
    NoneBlockBody,
    /// block header is none
    NoneBlockHeader,
    /// early status received
    EarlyStatus,
    /// proto struct encode error
    EncodeError,
    /// proto struct encode error
    DecodeError,
    /// no candidate block
    NoCandidate,
    /// fork tree no block
    NoForkTree,
    /// find dup transaction
    DupTransaction,
    /// proposal too high
    ProposalTooHigh,
    /// proposal too low
    ProposalTooLow,
    /// proposal check error
    ProposalCheckError,
    /// internal error, todo
    InternalError,
    /// other errors, todo
    ExpectError,

    /// Consensus from 200
    /// check proposal proof error
    ProposlProofError = 200,
}

impl ::std::error::Error for Status {}

impl ::std::fmt::Display for Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Status::Success => write!(f, "Success"),
            Status::ConvertIntError => write!(f, "Convert int to Status err"),
            Status::MisbehaveNode => write!(f, "Node already in misbehave list"),
            Status::BannedNode => write!(f, "Node already in ban list"),
            Status::NoProvideAddress => write!(f, "No correct address provide"),
            Status::NoBlock => write!(f, "Not get the block"),
            Status::NoneProposal => write!(f, "Proposal should not be none"),
            Status::NoneBlockBody => write!(f, "BlockBody should not be None"),
            Status::NoneBlockHeader => write!(f, "BlockHeader should not be None"),
            Status::EarlyStatus => write!(f, "receive early status from same node"),
            Status::EncodeError => write!(f, "Proto struct encode error"),
            Status::DecodeError => write!(f, "Proto struct decode error"),
            Status::NoCandidate => write!(f, "No candidate block"),
            Status::ProposalTooHigh => write!(f, "Proposal height is higher than current height"),
            Status::ProposalTooLow => write!(f, "Proposal height is lower than current height"),
            Status::ProposalCheckError => write!(f, "Proposal check error"),
            Status::NoForkTree => write!(f, "Fork tree no block"),
            Status::DupTransaction => write!(f, "Found dup transaction"),
            Status::InternalError => write!(f, "Internal Status"),
            Status::ExpectError => write!(f, "Expect error"),
            Status::ProposlProofError => write!(f, "check proposal proof error"),
        }
    }
}

macro_rules! impl_int_from_status {
    ($myty : ty) => {
        impl From<Status> for $myty {
            fn from(v: Status) -> Self {
                v as $myty
            }
        }
    };
}

impl_int_from_status!(u16);
impl_int_from_status!(u32);
impl_int_from_status!(u64);
impl_int_from_status!(u128);

macro_rules! impl_status_from_int {
    ($int_t : ty,$from_int :ident) => {
        impl From<$int_t> for Status {
            fn from(v: $int_t) -> Self {
                let s = num::FromPrimitive::$from_int(v);
                s.unwrap_or(Status::ConvertIntError)
            }
        }
    };
}

impl_status_from_int!(u16, from_u16);
impl_status_from_int!(u32, from_u32);
impl_status_from_int!(u64, from_u64);
impl_status_from_int!(u128, from_u128);

#[cfg(test)]
mod tests {
    use crate::Status;

    #[test]
    fn it_works() {
        let num: u32 = Status::BannedNode.into();
        assert!(num == 101);

        let s = Status::from(102 as u64);
        assert!(Status::NoProvideAddress == s);

        let s = Status::from(65535 as u16);
        assert!(Status::ConvertIntError == s);
    }
}