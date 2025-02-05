// Copyright 2018 The Vault713 Developers
// Modifications Copyright 2019 The Gotts Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::error::{ErrorKind, Result};
use rustc_serialize::hex::{FromHex, ToHex};

pub mod bech32;
pub mod crypto;
pub mod secp;

/// Encode the provided bytes into a hex string
pub fn to_hex(bytes: Vec<u8>) -> String {
	bytes.to_hex()
}

/// Decode a hex string into bytes (no '0x' prefix).
pub fn from_hex(hex_str: String) -> Result<Vec<u8>> {
	hex_str
		.from_hex()
		.map_err(|_| ErrorKind::NumberParsingError.into())
}
