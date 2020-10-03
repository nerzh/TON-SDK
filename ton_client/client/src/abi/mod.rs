/*
* Copyright 2018-2020 TON DEV SOLUTIONS LTD.
*
* Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
* this file except in compliance with the License.
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific TON DEV software governing permissions and
* limitations under the License.
*/

use crate::dispatch::DispatchTable;

#[cfg(test)]
mod tests;

mod abi;
mod decode;
pub(crate) mod encode;
mod errors;
mod internal;
mod signing;

pub use abi::{Abi, AbiHandle, FunctionHeader};
pub use decode::{decode_message, DecodedMessageBody, DecodedMessageType, ParamsOfDecodeMessage};
pub use encode::{
    attach_signature, encode_message, CallSet, DeploySet, ParamsOfAttachSignature,
    ParamsOfEncodeMessage, ResultOfAttachSignature, ResultOfEncodeMessage,
};
pub use errors::{Error, ErrorCode};
pub use signing::Signer;

pub const DEFAULT_WORKCHAIN: i32 = 0;

/// Functions for encoding and decoding messages due to ABI
/// specification.
#[derive(TypeInfo)]
#[type_info(name = "abi")]
struct AbiModule;

pub(crate) fn register(handlers: &mut DispatchTable) {
    handlers.register_module::<AbiModule>(|reg| {
        reg.t::<Abi>();
        reg.t::<AbiHandle>();
        reg.t::<FunctionHeader>();
        reg.t::<CallSet>();
        reg.t::<DeploySet>();

        reg.async_f(encode_message, encode::encode_message_info);
        reg.f(attach_signature, encode::attach_signature_info);
        reg.f(decode_message, decode::decode_message_info);
    });
}
