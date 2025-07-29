// types/deposit_checker.rs
// Shared types and error enums for deposit verification in Domex

/// Enum representing possible errors during native deposit validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepositVerificationError {
    /// Deposit transaction not found on the native chain
    TxNotFound,

    /// Transaction was found, but recipient address mismatched
    AddressMismatch {
        expected: String,
        found: String,
    },

    /// Transaction occurred, but amount was below minimum onboarding requirement
    AmountTooLow {
        required: u64,
        found: u64,
    },

    /// Unable to derive native address from public key (e.g., malformed key)
    AddressDerivationFailed,
}
