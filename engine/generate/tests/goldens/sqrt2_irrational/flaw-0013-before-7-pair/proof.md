# The square root of two is irrational

**Theorem.** For all natural numbers $p$ and $q$ with $\gcd(p, q) = 1$, $p^2 \ne 2q^2$: the square root of two is irrational.

**Proof.**

**Step 1.** Recall that $p$ and $q$ are coprime when $\gcd(p, q) = 1$, which is exactly the condition that the fraction $p/q$ is in lowest terms.

**Step 2.** Recall the definition of parity: a natural number $n$ is even when $2 \mid n$, that is, when $n \bmod 2 = 0$, and odd otherwise, that is, when $n \bmod 2 = 1$; every natural number is one or the other.

**Step 3.** Recall the definition of divisibility: for natural numbers $a$ and $b$, $a \mid b$ means that $b = ac$ for some natural number $c$; in particular $2 \mid b$ means $b = 2c$ for some $c$, and such a $b$ is called even.

**Step 4.** Recall that a rational number is a quotient $p/q$ of integers with $q \ne 0$, that every positive rational number has a representation in lowest terms with $p$ and $q$ natural numbers, and that $\gcd(p, q)$, the greatest common divisor, is the largest natural number dividing both $p$ and $q$; in particular it divides both.

**Step 5.** A third derivation: writing $p^2 = p \cdot p$, the residue of a product modulo $2$ is the product of the residues, so if $p \bmod 2 = 1$ then $p \cdot p \bmod 2 = 1 \cdot 1 = 1$, which is impossible when $2 \mid p^2$; hence $p$ is even.

**Step 6.** We check once more that if $2 \mid p^2$ then $2 \mid p$: the residue of $p^2$ modulo $2$ is the residue of $(p \bmod 2)^2$, so if $p \bmod 2 = 1$ then $p^2 \bmod 2 = 1$ and $2 \nmid p^2$; hence $p \bmod 2 = 0$.

**Step 7.** Here is a second derivation of the same fact: if $p$ were odd, say $p = 2m + 1$, then $p^2 = 4m^2 + 4m + 1 = 2(2m^2 + 2m) + 1$ would be odd, so $2 \nmid p^2$; therefore $2 \mid p^2$ forces $p$ to be even.

**Step 8.** Suppose, for contradiction, that $p^2 = 2q^2$ for natural numbers $p$ and $q$ with $\gcd(p, q) = 1$.

**Step 9.** Then $p^2 = 2q^2$ is even, being twice the natural number $q^2$.

**Step 10.** Hence $p$ is even: if $p$ were odd, $p^2$ would be odd, because the square of an odd number is odd. Write $p = 2k$.

**Step 11.** Substituting, $4k^2 = 2q^2$, so $q^2 = 2k^2$ is even, and by the same argument $q$ is even. Write $q = 2m$.

**Step 12.** Thus $2$ divides both $p$ and $q$, so $2$ divides $\gcd(p, q) = 1$, which is absurd. Hence no such $p$ and $q$ exist, and $\sqrt{2}$ is irrational.

∎
