//! [ElGamal encryption] and related cryptographic protocols with pluggable crypto backends.
//!
//! # ⚠ Warning
//!
//! While the logic in this crate relies on standard cryptographic assumptions
//! (complexity of discrete log and computational Diffie – Hellman problems in certain groups),
//! it has not been independently verified for correctness or absence of side-channel attack
//! vectors. **Use at your own risk.**
//!
//! # Overview
//!
//! - [`Encryption`] provides ElGamal encryption, i.e., public-key encryption using a prime-order
//!   group with discrete log / computation Diffie – Hellman assumptions. This and other protocols
//!   use [`PublicKey`], [`SecretKey`] and [`Keypair`] to represent participants' keys.
//! - Besides basic encryption, `Encryption` also provides zero-knowledge proofs of
//!   [zero encryption](Encryption::encrypt_zero()) and of
//!   [Boolean value encryption](Encryption::encrypt_bool()). These are useful in higher-level
//!   protocols, e.g., re-encryption.
//! - [`EncryptedChoice`] provides a way to encrypt a choice of one of `n` variants so that
//!   encryption is additively homomorphic and has a zero-knowledge proof of correctness.
//! - [`sharing`](crate::sharing) module exposes a shared ElGamal encryption scheme, including
//!   distributed key generation and shared decryption. Some crypto primitives used in the module
//!   can be used independently, e.g., [`ProofOfPossession`].
//!
//! # Backends
//!
//! [`group`](crate::group) module exposes a generic framework for plugging a [`Group`]
//! implementation into crypto primitives. It also provides several implementations:
//!
//! - [`Ristretto`] and [`Curve25519Subgroup`] implementations based on Curve25519 using
//!   [`curve25519-dalek`].
//! - [`Generic`] implementation allowing to plug in any elliptic curve group conforming to
//!   the traits specified by the [`elliptic-curve`] crate. For example,
//!   the secp256k1 curve can be used via the [`k256`] crate.
//!
//! [ElGamal encryption]: https://en.wikipedia.org/wiki/ElGamal_encryption
//! [`Group`]: crate::group::Group
//! [`Ristretto`]: crate::group::Ristretto
//! [`Curve25519Subgroup`]: crate::group::Curve25519Subgroup
//! [`curve25519-dalek`]: https://docs.rs/curve25519-dalek/
//! [`Generic`]: crate::group::Generic
//! [`elliptic-curve`]: https://docs.rs/elliptic-curve/
//! [`k256`]: https://docs.rs/k256/

// Linter settings.
#![warn(missing_debug_implementations, missing_docs, bare_trait_objects)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
    clippy::doc_markdown
)]

mod encryption;
pub mod group;
mod keys;
mod proofs;
pub mod sharing;

pub use crate::{
    encryption::{DiscreteLogTable, EncryptedChoice, Encryption},
    keys::{Keypair, PublicKey, SecretKey},
    proofs::{LogEqualityProof, ProofOfPossession, RingProof, RingProofBuilder},
};
