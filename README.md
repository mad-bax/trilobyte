# Trilobyte

## Perfect Secrecy

To ensure 'perfect secrecy', there are four conditions that must be satisfied:

1. The `key text` must be *as long* as the `message text`.
2. The `key text` must be *truly random*.
3. The `key text` must never be reused in whole or in part.
4. The `key text` must be kept completely secret by the communicating parties.

When these four conditions hold, the `cipher text` encryption is proven to be unbreakable. Even if part of the `key text` or `message text` is revealed, the remainder of the `cipher text` remains encrypted as there are no repeating patterns.

This scheme is both quantum and post-quantum secure. There is no machine that exists, **or that could possibly exist**, that can break this scheme.

## Encryption

The encryption of `message text` into `cipher text` is as follows:

`message text`, $M$, of length $n$

`key text`, $K$, of length $i$ where $n \le i$

`cipher text`, $C$, where $C_j = M_j \oplus K_j$ for $1 \le j \le n$

## Decryption

The decryption of `cipher text` into `message text` is as follows:

`cipher text`, $C$, of length $n$

`key text`, $K$, of length $i$ where $n \le i$

`message text`, $M$, where $M_j = C_j \oplus K_j$ for $1 \le j \le n$

## Limitations

Achieving 'true randomness' on a computer is difficult, and is likely not possible on any machines that are easily accessible.

Key distribution remains a difficult problem. While this scheme is perfectly secure, there must be some prior arrangements to exchange keys outside of purely digital transference.
