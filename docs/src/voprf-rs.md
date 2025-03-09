This spec focuses on implementing the networking part for vOPRF protocol. It describes how nodes generate keys, process evaluation requests, and coordinate to provide secure vOPRF functionality.

# Web2 Nullifiers using vOPRF - Network/MPC Specification

## Overview

This specification describes the networking and MPC components of the vOPRF system. The implementation uses a non-coordinated N-of-N setup with N=3 nodes, where each node operates independently without inter-node communication.

## Node Setup

### Key Generation
Each node must:
1. Generate a BabyJubJub keypair:
   ```
   private_key = random_scalar()
   public_key = private_key * G  // G is BabyJubJub base point
   ```
2. Make their public key available for verification (store it in a public registry, e.g. Ethereum)

## API Specification

### Endpoint: `/api/v1/evaluate`

Processes vOPRF evaluation requests and returns the result with a proof of correctness.

#### Request
- Method: `POST`
- Content-Type: `application/json`
- Body:
  ```json
  {
    "commitment1": "0x...", // scalar (Poseidon hash - commitment)
    "commitment2": {
      "x": "0x...",  // hex-encoded x coordinate
      "y": "0x..."   // hex-encoded y coordinate
    },
    "proof": {
      // ZK proof that commitment is valid
      // (as specified in [OPRF Commitment Circuit](./zk-circuits.md#circuit-1-oprf-commitment-circuit))
    }
  }
  ```

#### Response
- Status: 200 OK
- Content-Type: `application/json`
- Body:
  ```json
  {
    "result": {
      "x": "0x...",  // hex-encoded x coordinate
      "y": "0x..."   // hex-encoded y coordinate
    },
    "dleq_proof": {
      "c": "0x...",  // challenge
      "s": "0x..."   // response
    }
  }
  ```

### Processing Steps

1. **Input Validation**
   - Verify that commitment point is on BabyJubJub curve
   - Verify the accompanying ZK proof

2. **OPRF Evaluation**
   ```
   result = private_key * commitment2
   ```

3. **DLEQ Proof Generation**
   Generate Chaum-Pedersen proof to prove that:
   ```
   (G, public_key) ~ (commitment2, result)
   ```
   where `~` denotes "same discrete logarithm"

   Reference: https://github.com/holonym-foundation/mishti-crypto/blob/main/src/lib.rs#L109

## Client Integration

Clients must:
1. Generate commitment using Circuit 1 (OPRF Commitment Circuit)
2. Contact all N nodes independently
3. Verify DLEQ proofs from all nodes
4. Sum all responses to get final OPRF result
5. Generate nullifier using Circuit 2 (Nullifier Generation Circuit)

## Error Handling

Return appropriate HTTP status codes:
- 400 Bad Request: Invalid input format or point not on curve
- 401 Unauthorized: Invalid proof
- 429 Too Many Requests: Rate limit exceeded
- 500 Internal Server Error: Node error

Error response format:
```json
{
  "error": {
    "code": "INVALID_POINT",
    "message": "Provided point is not on BabyJubJub curve"
  }
}
```
```

This specification can be implemented in Rust. Reusing parts of [Holonym Foundation's vOPRF](https://github.com/holonym-foundation/mishti-crypto) might be helpful.