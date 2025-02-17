## Abstract

Recent development of protocols, that allow us to make Web2 data portable & verifiable such as [ZK Email](https://prove.email/) or [TLSNotary](https://tlsnotary.org/) opens new use-cases and opportunities for us. For example, we can make proof of ownership of some x.com username or email address and verify it on-chain with ZK Email. Projects like [OpenPassport](https://www.openpassport.app/), [Anon Aadhaar](https://github.com/anon-aadhaar/anon-aadhaar) (and others) are also the case.

We can also do more complex things, e.g. forum where holders of @ethereum.org email addresses will be able to post anonymously, using zk proofs of membership. 

Projects like [Semaphore](https://semaphore.pse.dev/) helps us to build pseudonymous[^1] systems with membership proofs for "Web3 identities". 

In Semaphore users have their $\text{public_id} = \text{hash(secret, nullifier)}$, and $\text{nullifier}$ actually serves as an id of user - we still don't know who exactly used the system, but we'll be able to find out if they used it more than once. But the thing is **we don't have any nullifiers** in ZK Email/TLS, etc. - that's why it's not possible to create such systems for Web2 identities out of the box. The solution for that is vOPRF.

vOPRFs (verifiable Oblivious PseudoRandom Functions) - are protocols that allow a client to generate deterministic random based on their input, while keeping it private. So, there're two parties in the protocol - first one as I said is a client, and second one is a OPRF network (usually [MPC](https://en.wikipedia.org/wiki/Secure_multi-party_computation) is used for that).

With OPRF we'll be able to generate nullifiers for Web2 ID's': users will just need to ask the MPC to generate it, e.g., based on their email address (without revealing plain text of course).

We can do many things based on that:
* anonymous votings with ported Web2 identities;
* anonymous airdrops - projects can just list github accounts, that are eligible for airdrop, and users will be able to claim (only once) with proof of github using ZK Email; 
* pseudonymous forums - I mentioned it before, but with OPRF we can have pseudonyms and limit user to only one account + it might be easier to track & ban spammers
* ... many more.

Read the [next section](#detailed-explanation) for more details.

## Detailed explanation

There are three parties involved in protocol:                                            $\DeclareMathOperator{cm}{commitment} \DeclareMathOperator{hash}{hash} \DeclareMathOperator{hashc}{hashToCurve} \DeclareMathOperator{chaum}{chaumPedersenVerify}$
* **User**, that is trying to do some action with their Web2 identity (e.g. google account) pseudonymously (e.g. anonymously participate in voting).
* **OPRF** Server/Network (will just call OPRF). 
    * We use MPC, because in the case of having only one node generating nullifiers for users - it'll be able to bruteforce and find out which Web2 identity corresponds given nullifier. Every node has to commit to their identity somehow - e.g., by storing their EC public key on a blockchain. 
    For simplicity I'll explain the case with one node OPRF first, and in [OPRF-MPC section](#oprf-mpc) I'll explain how we can extend it to multiple nodes.
* **Ethereum** (or any other smart-contract platform)
---

* Nodes must be run in a TEE to add additional security
* All of the EC operations are done on BabyJubJub Curve
* For the $\hash$ operation we use Poseidon Hash

---

### Main protocol

1. User makes ZK Email/TLS auth proof with salted commitment to UserID (or email, name, etc.) as a public output:
    $\cm_1 = \hash(\text{UserID}, \text{salt})$
I'll call it just **Auth proof**
    
2. User sends new commitment to their UserID to OPRF:
    $\cm_2 = r * G$ where $G = \hashc(\text{UserID})$, and $r$ is random scalar. <br>
We want to prevent users from sending arbitrary requests (because they would be able to attack the system by sending commitments to different user's identities), so user must additionally provide a small zk proof, that checks the relation between the commitments, where:
    * Public inputs: $\cm_1$, $\cm_2$
    * Private inputs: $\text{UserID}$, $\text{salt}$, $r$
    
    and constraints:
    * $\cm_1 = \hash(\text{UserID}, \text{salt})$ 
    * $G = \hashc(\text{UserID})$ 
    * $\cm_2 = r * G$

    *It's possible to do step 1 and 2 in one circuit, but that would require a lot of changes in i.e. ZK Email/TLS circuit, which is not desirable.*

3. OPRF replies with:
    * $\text{oprf_response} = s * \cm_2$                  
where $s$ is a private key of OPRF node; and also replies with proof of correctness of such multiplication, which is in this case might be a Chaum-Pedersen proof of discrete log equality (check [this blog post](https://muens.io/chaum-pedersen-protocol)) on that.
    
    
    <!--Chaum-Pedersen proof of $(\cm_2, \text{oprf_response}) \sim (Z, P)$.
    and I know such $r$ that $\cm_2 = r G$
    then clearly $sG = r^{-1}\text{oprf_response}$-->
    
4. User creates zk proof with the following parameters:
    * Public outputs: $\cm_1, \text{ nullifier}$
    * Private inputs: $r, \text{ UserID}, \text{ salt}, \text{ chaum_pedersen_proof}, \text{ oprf_response}$

    and validates that:
    * $\cm_1 = \hash(\text{UserID}, \text{ salt})$ 
    * $G \longleftarrow \hashc(\text{UserID})$ 
    * $\chaum (\text{oprf_response})$ 
    * $\text{nullifier} \longleftarrow r^{-1} * \text{ oprf_response}$
    
### On nullifiers
    
That's it, we have nullifier - and now users can use the system as in Semaphore.
If we go a bit further, it's worth to mention that users shouldn't reveal nullifier, because it's linked with their $\text{UserID}$; and if they use the same $\text{UserID}$ in different apps - it'll be possible to track them. 
We can do it a bit differently - instead of revealing nullifier we can reveal $\hash(\text{nullifier}, \text{AppID})$ - where $\text{AppID}$ is a unique identifier of the app, and that's gonna be our real nullifier.
    
## OPRF MPC

In the example above we used only one node OPRF, but we can easily extend it to multiple nodes. 
There're many ways to do that, I'll explain few:
1. N of N MPC:
    * 1.1. All nodes have their own pair of keys.
    * 1.2. Every node does step 3 individually: we get $\text{oprf_response}_i = s_i * r * G$
    * 1.3. On step 4 we verify $\text{chaum_pedersen_proof}$ for every node
    * 1.4 We calculate $\text{nullifier}_i = \text{oprf_response}_i * r^{-1}$
    * 1.5 We calculate $\sum_{i=1}^N\text{nullifier}_i = s * G$

    *Important to mention that we have to verify/calculate everything in the circuit*
2. M of N MPC using linear combination for Shamir Secret Sharing:
    * Similar to N of N MPC, but we need only M shares
3. Interactive M of N threshold MPC: similar to **2**, but nodes calculate one common response and one common Chaum-Pedersen proof. With that - we won't need to verify separate Chaum-Pedersen proofs in zk circuit, we'll only need to verify one, though with this scheme we'll need to care about coordination between MPC nodes.
4. Using BLS:
    * 3.1. Calculate common public key of all OPRF nodes by summing individual public keys
    * 3.2. The same as in N of N MPC case
    * 3.3., 3.4, 3.5. - The same as in N of N MPC case, **BUT** we can do it outside the circuit
    * 3.6. Verify BLS pairing in the circuit

There are techniques such as DKG protocol and Resharing protocol, that can be useful for production ready protocol. With them you can build a permissionless MPC network, where it'll be possible to change the MPC node set each epoch, while having the same common secret key. You can read more about them in [Nanak's](https://ethresear.ch/u/nanaknihal/summary) post on OPRF [here](https://ethresear.ch/t/a-threshold-network-for-human-keys-to-solve-privacy-and-custody-issues/20276/1).

*For the alpha release we'll stick to the N of N MPC setup, where N = 3, without DKG & Resharing*.

---

## Additional info

The same protocol can be applied to other Web2<->Web3 bridges, such as OpenPassport, TLS Notary, ZK JWT, etc.

## Footnotes

[^1]: Pseudonymous system - a privacy-preserving system, where users' transactions are linked to unique identifiers (pseudonyms), but not their actual identities.