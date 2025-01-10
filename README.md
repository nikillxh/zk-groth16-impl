# Groth16 Protocol from Scratch

An implementation of the Groth16 zero-knowledge proof protocol from the ground up. It covers essential cryptographic components such as Rank-1 Constraint Systems (R1CS), Quadratic Arithmetic Programs (QAP), and the trusted setup process for zk-SNARKs.

## Key Features

- **Finite Field Arithmetic**: Custom data structures and implementations for 64-bit and 256-bit field elements.
- **R1CS**: Defines the constraints that must be satisfied for the proof.
- **QAP**: Converts R1CS into quadratic equations for proof generation.
- **Trusted Setup**: Implements the setup process using secret randomness and generator functions.
- **Vector Operations**: Provides vector manipulation functions for cryptographic operations.

## Files Description

- **r1cs.rs**: Contains the implementation of Rank-1 Constraint Systems (R1CS).
- **qap.rs**: Converts the R1CS to a Quadratic Arithmetic Program (QAP), which is used for proof generation and verification.
- **trustedsetup.rs**: Handles the trusted setup process, generating parameters for the protocol.
- **field.rs**: Implements finite field arithmetic for 64-bit and 256-bit field elements.
- **vector.rs**: Contains helper functions for vector operations in cryptographic contexts.
- **main.rs**: Demonstrates how to create R1CS constraints, convert them to QAP, and generate and verify proofs.

## Mechanism

1. **Define Constraints (R1CS)**: First, constraints are defined using `Element` and `Constraint` in the `main.rs` file.
2. **Convert to QAP**: These constraints are then transformed into a QAP, representing them as quadratic equations.
3. **Generate Trusted Setup**: The `trustedsetup` function generates the public parameters using secret randomness and generators.
4. **Proof Generation and Verification**: The prover generates a proof based on the setup, which is later verified to ensure its correctness.

## Running the Program

To run the example:

1. Clone the repository:
   ```bash
   git clone https://github.com/nikillxh/zk-groth16-impl
2. Build and run the project:
    ```bash
    cargo run
The example will define some constraints, convert them into a QAP, generate a trusted setup, and evaluate the proof. Finally, it will verify if the proof is valid.
