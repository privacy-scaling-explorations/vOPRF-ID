This spec focuses on how a user commits to a userID with a random scalar `r` and proves consistency with their existing commitment (which comes from a separate ZK Email/TLSNotary proof). The idea is to ensure that a user cannot arbitrarily switch userIDs while reusing the same randomness.

# Web2 Nullifiers using vOPRF — ZK Circuit Specification

## Overview

This specification describes two ZK circuits that work together with an existing Auth Proof:

1. **OPRF Commitment Circuit**: Proves that a user's commitment to the OPRF server is consistent with their previously verified identity.
2. **Nullifier Generation Circuit**: Proves that a nullifier is correctly derived from the OPRF response and consistent with the user's identity.

The Auth Proof (e.g., ZK Email/TLSNotary proof) is an external component that provides `commitment1`, a commitment to the user's identity.

## High-Level Flow

1. User already has an **Auth Proof** with salted commitment to UserID as a public output:
   ```
   commitment1 = hash(UserID, salt)
   ```

2. User creates an **OPRF Commitment Proof** to send to the OPRF server:
   ```
   commitment2 = r * G where G = hashToCurve(UserID)
   ```
   where r is random scalar. This proof ensures `commitment2` is consistent with the same UserID in `commitment1`.

3. OPRF replies with:
   ```
   oprf_response = s * commitment2
   ```
   where `s` is a private key of OPRF node; and also replies with proof of correctness (e.g., a Chaum-Pedersen proof).

4. User creates a **Nullifier Generation Proof** that:
   - Takes `commitment1` and produces `nullifier`
   - Verifies the OPRF response is valid
   - Computes `nullifier = r^-1 * oprf_response`

## Circuit 1: OPRF Commitment Circuit

This circuit proves that the commitment sent to the OPRF server is consistent with the user's verified identity.

### Public Inputs
1. `commitment1`  
   From the Auth Proof, computed as `hash(UserID, salt)`.

2. `commitment2`  
   The commitment to send to the OPRF, computed as `r * G` where `G = hashToCurve(UserID)`.

### Private Inputs
1. `UserID`  
   The user's identity string (email, TLSNotary-verified name, etc.).

2. `salt`  
   The salt used in the Auth Proof commitment.

3. `r`  
   A random scalar chosen by the user.

### Circuit Constraints
1. **Auth Proof Consistency**  
   ```
   commitment1 == hash(UserID, salt)
   ```

2. **OPRF Commitment Calculation**  
   ```
   G = hashToCurve(UserID)
   commitment2 == r * G
   ```

### Pseudocode
```
// OPRF Commitment Circuit

function ProveOPRFCommitment(
  // Public inputs
  commitment1,
  commitment2
) {
  // Private inputs
  user_id;
  salt;
  r;

  // 1. Verify consistency with Auth Proof
  computed_commitment1 = Hash(user_id, salt);
  Assert(computed_commitment1 == commitment1);

  // 2. Verify OPRF commitment
  G = HashToCurve(user_id);
  computed_commitment2 = ScalarMul(G, r);
  Assert(computed_commitment2 == commitment2);
}
```

## Circuit 2: Nullifier Generation Circuit

This circuit proves that the nullifier is correctly derived from the OPRF response and consistent with the user's identity.

### Public Inputs
1. `commitment1`  
   From the Auth Proof, computed as `hash(UserID, salt)`.

2. `nullifier`  
   The final nullifier value computed as `r^-1 * oprf_response`.

### Private Inputs
1. `UserID`  
   The user's identity string.

2. `salt`  
   The salt used in the Auth Proof commitment.

3. `r`  
   The random scalar used in the OPRF commitment.

4. `oprf_response`  
   The response from the OPRF server.

5. `chaum_pedersen_proof`  
   Proof of correctness for the OPRF response.

### Circuit Constraints
1. **Auth Proof Consistency**  
   ```
   commitment1 == hash(UserID, salt)
   ```

2. **OPRF Response Verification**  
   ```
   ChaumPedersenVerify(oprf_response, chaum_pedersen_proof)
   ```

3. **Nullifier Calculation**  
   ```
   nullifier == r^-1 * oprf_response
   ```

### Pseudocode
```
// Nullifier Generation Circuit

function ProveNullifier(
  // Public inputs
  commitment1,
  nullifier
) {
  // Private inputs
  user_id;
  salt;
  r;
  oprf_response;
  chaum_pedersen_proof;

  // 1. Verify consistency with Auth Proof
  computed_commitment1 = Hash(user_id, salt);
  Assert(computed_commitment1 == commitment1);

  // 2. Verify OPRF response
  Assert(ChaumPedersenVerify(oprf_response, chaum_pedersen_proof));

  // 3. Calculate nullifier
  r_inverse = InverseScalar(r);
  computed_nullifier = ScalarMul(oprf_response, r_inverse);
  Assert(computed_nullifier == nullifier);
}
```

These circuits can be implemented in various ZK proving systems such as Groth16, PLONK, Bulletproofs, or others, depending on the specific requirements of the application.
