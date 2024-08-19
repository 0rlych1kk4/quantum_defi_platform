use pqcrypto_ntru::ntruhps2048509::{keypair, sign, PublicKey, SecretKey}; // Adjust imports if needed
use solana_sdk::signature::{Keypair, Signature};
use solana_sdk::transaction::Transaction;

pub struct QuantumWallet {
    pub public_key: PublicKey,
    secret_key: SecretKey,
}

impl QuantumWallet {
    // Create a new QuantumWallet with a generated keypair
    pub fn new() -> Self {
        let (public_key, secret_key) = keypair();
        QuantumWallet { public_key, secret_key }
    }

    // Sign a transaction and return the Signature
    pub fn sign_transaction(&self, transaction: &Transaction) -> Signature {
        let tx_data = transaction.message().serialize();
        let signature = sign(&tx_data, &self.secret_key);
        Signature::new(signature.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::transaction::Transaction;
    use solana_sdk::signature::Signer;
    use pqcrypto_ntru::ntruhps2048509::{keypair, sign, Signature};

    #[test]
    fn test_new_wallet() {
        let wallet = QuantumWallet::new();
        assert_eq!(wallet.public_key.to_bytes().len(), 32); // Check the length of the public key
        assert_eq!(wallet.secret_key.to_bytes().len(), 32); // Check the length of the secret key
    }

    #[test]
    fn test_sign_transaction() {
        let wallet = QuantumWallet::new();
        let transaction = Transaction::new_unsigned(
            solana_sdk::instruction::Instruction::new_with_bytes(
                Pubkey::new_unique(),
                &[],
                vec![],
            ),
        );
        let signature = wallet.sign_transaction(&transaction);
        assert_eq!(signature.to_bytes().len(), 64); // Check the length of the signature
    }
}
