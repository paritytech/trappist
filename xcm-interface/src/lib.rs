// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{dispatch::DispatchInfo, weights::{GetDispatchInfo, PostDispatchInfo}};
use scale_info::TypeInfo;
use sp_runtime::{traits::Dispatchable, DispatchResultWithInfo};
use xcm::prelude::*;

#[derive(Clone, Decode, Encode, PartialEq, Eq, Debug, TypeInfo)]
pub enum SupportedInboundXcmTransactions<RuntimeCall> {
    Raw(RuntimeCall),
    // DefiDialect(VersionedDefiDialect),
}

impl<RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo>> Dispatchable for SupportedInboundXcmTransactions<RuntimeCall> {
    type Origin = <RuntimeCall as Dispatchable>::Origin;
    type Config = ();
    type Info = ();
    type PostInfo = PostDispatchInfo;

    fn dispatch(self, origin: Self::Origin) -> DispatchResultWithInfo<Self::PostInfo> {
        match self {
            Self::Raw(call) => call.dispatch(origin),
        }
    }
}

impl<RuntimeCall: GetDispatchInfo> GetDispatchInfo for SupportedInboundXcmTransactions<RuntimeCall> {
    fn get_dispatch_info(&self) -> DispatchInfo {
        match self {
            Self::Raw(call) => call.get_dispatch_info(),
        }
    }
}
