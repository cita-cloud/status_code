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

/// The response status code
#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive)]
pub enum StatusCode {
    /// Success: 0
    Success = 0,
    /// Convert int to status Error
    ConvertIntError,
    /// status code is none
    NoneStatusCode,

    /// controller error, start from 100
    /// node in misbehave list
    MisbehaveNode = 100,
    /// node in ban list
    BannedNode,
    /// address not consistent with record origin
    AddressOriginCheckError,
    /// provide address len is not 20
    ProvideAddressError,
    /// message not provide address
    NoProvideAddress,
    /// not get the block
    NoBlock,
    /// not get the proof
    NoProof,
    /// not get height of block which wrap tx
    NoTxHeight,
    /// not get tx index
    NoTxIndex,
    /// not get transaction
    NoTransaction,
    /// not get the block height base on hash
    NoBlockHeight,
    /// not get the block hash base on height
    NoBlockHash,
    /// proposal is none
    NoneProposal,
    /// block body is none
    NoneBlockBody,
    /// block header is none
    NoneBlockHeader,
    /// chain status is none
    NoneChainStatus,
    /// early status received
    EarlyStatus,
    /// store data error
    StoreError,
    /// execute error
    ExecuteError,
    /// proto struct encode error
    EncodeError,
    /// proto struct encode error
    DecodeError,
    /// no candidate block
    NoCandidate,
    /// not get early status
    NoEarlyStatus,
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
    /// consensus check proposal error
    ConsensusProposalCheckError,
    /// block hash check error
    BlockCheckError,
    /// the sig of chain status init check error
    CSISigCheckError,
    /// chain version or chain id check error
    VersionOrIdCheckError,
    /// hash check error
    HashCheckError,
    /// hash len is not correct
    HashLenError,
    /// signature len is not correct
    SigLenError,
    /// the node in sync mode
    NodeInSyncMode,
    /// internal error, todo
    InternalError,

    /// Consensus from 200
    /// check proposal proof error
    ConsensusServerNotReady = 200,
    ProposalProofError,

    /// Kms from 300
    /// Kms server not ready
    KmsServerNotReady = 300,
    /// hash result is none
    NoneHashResult,

    /// Network from 400
    /// Network server not ready
    NetworkServerNotReady = 400,
}

impl ::std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ::std::error::Error for StatusCode {}

macro_rules! impl_int_from_status {
    ($myty : ty) => {
        impl From<StatusCode> for $myty {
            fn from(v: StatusCode) -> Self {
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
        impl From<$int_t> for StatusCode {
            fn from(v: $int_t) -> Self {
                let s = num::FromPrimitive::$from_int(v);
                s.unwrap_or(StatusCode::ConvertIntError)
            }
        }
    };
}

impl_status_from_int!(u16, from_u16);
impl_status_from_int!(u32, from_u32);
impl_status_from_int!(u64, from_u64);
impl_status_from_int!(u128, from_u128);

impl From<cita_cloud_proto::common::StatusCode> for StatusCode {
    fn from(status: cita_cloud_proto::common::StatusCode) -> Self {
        StatusCode::from(status.code)
    }
}

#[cfg(test)]
mod tests {
    use crate::StatusCode;

    #[test]
    fn it_works() {
        let num: u32 = StatusCode::BannedNode.into();
        assert_eq!(num, 101);

        let s = StatusCode::from(104 as u64);
        assert_eq!(StatusCode::NoProvideAddress, s);

        let s = StatusCode::from(65535 as u16);
        assert_eq!(StatusCode::ConvertIntError, s);

        let status = cita_cloud_proto::common::StatusCode { code: 0 };
        let s = StatusCode::from(status);
        assert_eq!(StatusCode::Success, s);
    }
}
