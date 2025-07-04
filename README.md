# 📘 NEAR ProofType Smart Contract

This smart contract allows the registration of verifiable credential attestations on the **NEAR** blockchain. It is used as part of a decentralized identity (DID) and verifiable credentials (VC) flow, providing on-chain traceability of the issuer, subject, and the referenced content via a `cid`.

---

## 🚀 Functionality

### ✅ `issue_credential`

Registers a credential issued in the contract. Can only be called by the authorized issuer (signer in the app).

- **Parameters**:
  ```json
  {
    "subject_did": "did:near:usuario.testnet",
    "cid": "bafybeigdyrzi...",
    "expires_at": null
  }
  ```

- **cid definition**: The `cid` represents the **SHA-256 hash** (Base64-encoded) of the verifiable credential JSON. It is used as a unique identifier for the credential on-chain.

- **Requirements**:
  - The `cid` must not have been previously issued for that `subject_did`.
  - The `signer` of the transaction will be recorded as the issuer.

- **Recommended Gas**:
  `30 TGas` (`30000000000000`)

---

### 🔍 `is_valid`

Checks if a credential (`subject_did`, `cid`) has been issued and has not expired.

- **Parameters**:
  ```json
  {
    "subject_did": "did:near:usuario.testnet",
    "cid": "bafybeigdyrzi..."
  }
  ```

- **Returns**: `true | false`

---

## 🧪 Usage Examples

### Using `near-cli`

```bash
near call neardtiprooftype.testnet issue_credential \
  '{"subject_did": "did:near:usuario.testnet", "cid": "bafybeigdyrzi...", "expires_at": null}' \
  --accountId issuer.testnet --gas 30000000000000
```

```bash
near view neardtiprooftype.testnet is_valid \
  '{"subject_did": "did:near:usuario.testnet", "cid": "bafybeigdyrzi..."}'
```

---

## 🛠 SDK Integration

This contract integrates with the `ProofTypeNear` class:

```ts
const proof = await proofType.generateProof(vcPayload, { wallet, cid });
const valid = await proofType.verifyProof(proof); // true or false
```

Where `vcPayload` includes `issuer`, `credentialSubject.id`, and other VC fields.

---

## 🔐 Security

- Each `subject_did + cid` pair can only be issued once.
- Overwriting an existing record is not allowed.
- The issuer is tied to the signer of the transaction.

---

## 🧾 Specifications

- Network: `testnet`
- Contract: `neardtiprooftype.testnet`
- Language: Rust (`near-sdk`)
- Storage: Optimized by DID and list of CIDs
- License: MIT

---

## 📦 Compile & Deploy

```bash
cargo build --target wasm32-unknown-unknown --release
near deploy --wasmFile target/wasm32-unknown-unknown/release/prooftype.wasm --accountId neardtiprooftype.testnet
```

---

## 🧩 Relationship with DID Registry

This contract complements the **Near DID Registry**, which resolves DID documents. The ProofType acts as the **attestation capability**.
