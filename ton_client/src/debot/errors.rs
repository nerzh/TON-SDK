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

use crate::error::ClientError;
use std::fmt::Display;
const DEBOT: isize = ClientError::DEBOT; // 800

pub enum ErrorCode {
    DebotStartFailed = DEBOT + 1,
    DebotFetchFailed,
    DebotExecutionFailed,
    DebotInvalidHandle,
}
pub struct Error;

pub fn error(code: ErrorCode, message: String) -> ClientError {
    ClientError::with_code_message(code as u32, message)
}

impl Error {
    pub fn start_failed(err: impl Display) -> ClientError {
        error(
            ErrorCode::DebotStartFailed,
            format!("Debot start failed: {}", err),
        )
    }

    pub fn fetch_failed(err: impl Display) -> ClientError {
        error(
            ErrorCode::DebotFetchFailed,
            format!("Debot fetch failed: {}", err),
        )
    }

    pub fn execute_failed(err: impl Display) -> ClientError {
        error(
            ErrorCode::DebotExecutionFailed,
            format!("Debot execution failed: {}", err),
        )
    }

    pub fn invalid_handle(handle: u32) -> ClientError {
        error(
            ErrorCode::DebotInvalidHandle,
            format!("Invalid debot handle: {}", handle),
        )
    }
}
