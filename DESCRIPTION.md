# vOPRF-ID Spec

*Recommended reading: [Web2 Nullifiers using vOPRF](https://curryrasul.com/blog/web2-nullifiers/)*

## Abstract

Recent development of protocols, that allow us to make Web2 data portable & verifiable such as ZK Email or TLSNotary opens new use-cases and opportunities for us. For example, we can make proof of ownership of some x.com username or email address and verify it on-chain with ZK Email. Projects like OpenPassport, Anon Aadhaar (and others) are also the case.

We can also do more complex things, e.g. forum where holders of @ethereum.org email addresses will be able to post anonymously, using zk proofs of membership.

Projects like Semaphore helps us to build pseudonymous systems with membership proofs for "Web3 identities".

In Semaphore users have their `public_id = hash(secret, nullifier)`, and `nullifier` actually serves as an id of user - we still don't know who exactly used the system, but we'll be able to find out if they used it more than once. But the thing is we don't have any nullifiers in ZK Email/TLS, etc. - that's why it's not possible to create such systems for Web2 identities out of the box. The solution for that is vOPRF.

vOPRFs (verifiable Oblivious PseudoRandom Functions) - are protocols that allow a client to generate deterministic random based on their input, while keeping it private. So, there're two parties in the protocol - first one as I said is a client, and second one is a OPRF network (usually MPC is used for that).

With OPRF we'll be able to generate nullifiers for Web2 ID's': users will just need to ask the MPC to generate it, e.g., based on their email address (without revealing plain text of course).

We can do many things based on that:

* anonymous votings with ported Web2 identities;
* anonymous airdrops - projects can just list github accounts, that are eligible for airdrop, and users will be able to claim (only once) with proof of github using ZK Email;
* pseudonymous forums - I mentioned it before, but with OPRF we can have pseudonyms and limit user to only one account + it might be easier to track & ban spammers
* ... many more.

## Detailed explanation

### Parties involved in the protocol

There are three parties involved in protocol: 

* User, that is trying to do some action with their Web2 identity (e.g. google account) pseudonymously (e.g. anonymously participate in voting).
* OPRF Server/Network (will just call OPRF).
    * We use MPC, because in the case of having only one node generating nullifiers for users - it'll be able to bruteforce and find out which Web2 identity corresponds given nullifier. Every node has to commit to their identity somehow - e.g., by storing their EC public key on a blockchain. For simplicity I'll explain the case with one node OPRF first, and in OPRF-MPC section I'll explain how we can extend it to multiple nodes.
* Ethereum (or any other smart-contract platform)

### Primitives used in the protocol

### The protocol

### Minimum Viable Product

## Additional comments
