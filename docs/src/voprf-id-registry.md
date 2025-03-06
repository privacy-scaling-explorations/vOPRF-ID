# Web2 Nullifiers using vOPRF

*Recommended to read [Overview](./voprf-id.md) for understanding.*

## Abstract

Idea of having nullifiers for Web2 IDs is very promising. There's an [explanation](./overview.md) of the idea and how we can achieve this. 
As it was said - we can build a lot of applications on top of that, and this would require users to go through the same [process](./overview.md#main-protocol) for all apps. But can we do better? YES!

What we can do - is to create a one big global system that will hold registered identities, and people will be able to reuse them across different apps (of course while preserving nullifiers & anonimity) - kinda like global Semaphore for Web2 Identities.

In the next section I'll explain how we can do that.

## How it works

In the end of [the overview](./overview.md) explanation I said:           $\DeclareMathOperator{hash}{hash}                           \DeclareMathOperator{cm}{commitment}                        \DeclareMathOperator{hashc}{hashToCurve}$
> instead of revealing nullifier we can reveal $\hash(\text{nullifier}, \text{AppID})$ - where $\text{AppID}$ is a unique identifier of the app, and that's gonna be our real nullifier.

That's actually the key for building such system.

We can deploy one registry smart-contract. We'll set $\text{AppID} = 0$. We also gonna keep $\text{pseudonym} = \hash(s * G, \text{ AppID})$, where, $s$ is "private key" of OPRF MPC, and $G = \hashc(\text{UserID})$.
All the identities will be stored in Merkle Tree, and $\text{pubkey} = \hash(\text{pseudonym}, \cm_1)$ will be stored in its leaves. 
For those who forgot, $\cm_1 = \hash(\text{UserID}, \text{ salt})$.

Now, let's say we registered our github identity in our global system and there's an app that gives airdrop to their contributors (of course we want to claim airdrop anonymously). The airdrop app will need to set their own $\text{AppID}$, different from 0, because 0 is already taken; this can get checked by registry smart-contract. It will also need to create a merkle tree of github usernames that are eligible for airdrop (our github username is in that list).

**Now, to claim airdrop anonymously we'll need to create zk proof with the following parameters**:
* Public: $\text{AppID}, \text{ nullifier}, \text{ registryRoot}, \text{ appRoot}$
* Private: $\text{UserID, } sG, \text{ salt}$

and the constraints:
$$\text{pseudonym} \longleftarrow \hash(sG, \text{ 0})$$ $$\cm_1 \longleftarrow \hash(\text{UserID}, \text{ salt})$$ $$\text{pubkey} \longleftarrow \hash(\text{pseudonym}, \cm_1)$$ $$\text{registryRoot} = \text{merkleTreeVerify(pubkey)}$$ $$\text{appRoot} = \text{merkleTreeVerify(UserID)}$$ $$\text{nullifier} = \hash(sG, \text{ AppID})$$

<br>

As you can see, the pair $(sG, \text{ salt})$ will serve as an **action key**, and $sG$ itself will be a **viewing key**.

## Additional comments

While reading this, you might ask the same questions we have right now. For example, how we can prevent users spamming OPRF? It also makes sense to make system even more general and give people an option to specify which OPRF provider they gonna use and of what id-provider they will use (there can be many even for the same id-provider, e.g. bridged by zkEmail, zkTLS & OpenPassport).
While they are important, they are not in the scope of this blog post. 

Follow the updates & new posts on this and thanks for your attention! 

