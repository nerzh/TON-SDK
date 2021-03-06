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

use crate::boc::internal::deserialize_cell_from_base64;
use crate::client::ClientContext;
use crate::error::ClientResult;

#[derive(Serialize, Deserialize, Clone, ApiType)]
pub struct ParamsOfGetBocHash {
    /// BOC encoded as base64
    pub boc: String,
}

#[derive(Serialize, Deserialize, Clone, ApiType)]
pub struct ResultOfGetBocHash {
    /// BOC root hash encoded with hex
    pub hash: String,
}

/// Calculates BOC root hash
#[api_function]
pub fn get_boc_hash(
    _context: std::sync::Arc<ClientContext>,
    params: ParamsOfGetBocHash,
) -> ClientResult<ResultOfGetBocHash> {
    let (_, cell) = deserialize_cell_from_base64(&params.boc, "")?;

    Ok(ResultOfGetBocHash {
        hash: cell.repr_hash().to_hex_string(),
    })
}
