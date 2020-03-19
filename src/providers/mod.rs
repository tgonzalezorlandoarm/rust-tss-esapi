// Copyright (c) 2019, Arm Limited, All Rights Reserved
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//          http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! Core inter-op with underlying hardware
//!
//! [Providers](https://parallaxsecond.github.io/parsec-book/parsec_service/providers.html)
//! are the real implementors of the operations that Parsec claims to support. They map to
//! functionality in the underlying hardware which allows the PSA Crypto operations to be
//! backed by a hardware root of trust.
use parsec_interface::requests::ProviderID;
use serde::Deserialize;

pub mod core_provider;

#[cfg(feature = "pkcs11-provider")]
pub mod pkcs11_provider;

#[cfg(feature = "mbed-crypto-provider")]
pub mod mbed_provider;

#[cfg(feature = "tpm-provider")]
pub mod tpm_provider;

#[derive(Deserialize, Debug)]
// For providers configs in parsec config.toml we use a format similar
// to the one described in the Internally Tagged Enum representation
// where "provider_type" is the tag field. For details see:
// https://serde.rs/enum-representations.html
#[serde(tag = "provider_type")]
pub enum ProviderConfig {
    MbedCrypto {
        key_id_manager: String,
    },
    Pkcs11 {
        key_id_manager: String,
        library_path: String,
        slot_number: usize,
        user_pin: Option<String>,
    },
    Tpm {
        key_id_manager: String,
        tcti: String,
        owner_hierarchy_auth: String,
    },
}

use self::ProviderConfig::{MbedCrypto, Pkcs11, Tpm};

impl ProviderConfig {
    pub fn key_id_manager(&self) -> &String {
        match *self {
            MbedCrypto {
                ref key_id_manager, ..
            } => key_id_manager,
            Pkcs11 {
                ref key_id_manager, ..
            } => key_id_manager,
            Tpm {
                ref key_id_manager, ..
            } => key_id_manager,
        }
    }
    pub fn provider_id(&self) -> ProviderID {
        match *self {
            MbedCrypto { .. } => ProviderID::MbedCrypto,
            Pkcs11 { .. } => ProviderID::Pkcs11,
            Tpm { .. } => ProviderID::Tpm,
        }
    }
}

use crate::authenticators::ApplicationName;
use parsec_interface::operations::{
    list_opcodes, list_providers, ping, psa_destroy_key, psa_export_public_key, psa_generate_key,
    psa_import_key, psa_sign_hash, psa_verify_hash,
};
use parsec_interface::requests::{ResponseStatus, Result};

/// Provider interface for servicing client operations
///
/// Definition of the interface that a provider must implement to
/// be linked into the service through a backend handler.
pub trait Provide {
    /// Return a description of the current provider.
    ///
    /// The descriptions are gathered in the Core Provider and returned for a ListProviders operation.
    fn describe(&self) -> Result<list_providers::ProviderInfo>;

    /// List the providers running in the service.
    fn list_providers(&self, _op: list_providers::Operation) -> Result<list_providers::Result> {
        Err(ResponseStatus::PsaErrorNotSupported)
    }

    /// List the opcodes supported by the current provider.
    fn list_opcodes(&self, _op: list_opcodes::Operation) -> Result<list_opcodes::Result>;

    /// Execute a Ping operation to get the wire protocol version major and minor information.
    ///
    /// # Errors
    ///
    /// This operation will only fail if not implemented. It will never fail when being called on
    /// the `CoreProvider`.
    fn ping(&self, _op: ping::Operation) -> Result<ping::Result> {
        Err(ResponseStatus::PsaErrorNotSupported)
    }

    /// Execute a CreateKey operation.
    fn psa_generate_key(
        &self,
        _app_name: ApplicationName,
        _op: psa_generate_key::Operation,
    ) -> Result<psa_generate_key::Result> {
        Err(ResponseStatus::PsaErrorNotSupported)
    }

    /// Execute a ImportKey operation.
    fn psa_import_key(
        &self,
        _app_name: ApplicationName,
        _op: psa_import_key::Operation,
    ) -> Result<psa_import_key::Result> {
        Err(ResponseStatus::PsaErrorNotSupported)
    }

    /// Execute a ExportPublicKey operation.
    fn psa_export_public_key(
        &self,
        _app_name: ApplicationName,
        _op: psa_export_public_key::Operation,
    ) -> Result<psa_export_public_key::Result> {
        Err(ResponseStatus::PsaErrorNotSupported)
    }

    /// Execute a DestroyKey operation.
    fn psa_destroy_key(
        &self,
        _app_name: ApplicationName,
        _op: psa_destroy_key::Operation,
    ) -> Result<psa_destroy_key::Result> {
        Err(ResponseStatus::PsaErrorNotSupported)
    }

    /// Execute a SignHash operation. This operation only signs the short digest given but does not
    /// hash it.
    fn psa_sign_hash(
        &self,
        _app_name: ApplicationName,
        _op: psa_sign_hash::Operation,
    ) -> Result<psa_sign_hash::Result> {
        Err(ResponseStatus::PsaErrorNotSupported)
    }

    /// Execute a VerifyHash operation.
    fn psa_verify_hash(
        &self,
        _app_name: ApplicationName,
        _op: psa_verify_hash::Operation,
    ) -> Result<psa_verify_hash::Result> {
        Err(ResponseStatus::PsaErrorNotSupported)
    }
}
