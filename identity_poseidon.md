# Quantum-Safe Identity with Poseidon Hash

Domex eliminates ECDSA-based wallets by introducing Poseidon hash-based identity commitments.

## Key Structure
- Each user generates a Poseidon commitment:


identity = Poseidon(private_key)
A vault action is authorized only if the submitted proof includes a valid identity hash.

## Use Cases
- Token onboarding  
- Trade proof submission  
- Claim rights and exit proofs

## Benefits
- Quantum-resistant  
- No wallet key exposure  
- Scalable identity commitments for large user bases
