# A natural number plus one is at most two to that power

**Theorem.** For every natural number $n$, $n + 1 \le 2^n$.

**Proof.**

**Step 1.** Lemma 1. For every natural number $n$, $n < 2^n$. Proof of the lemma: by induction on $n$. For $n = 0$ we have $0 < 1 = 2^0$. If $k < 2^k$, then $k + 1 \le 2^k < 2 \cdot 2^k = 2^{k+1}$.

**Step 2.** By Lemma 1, $n < 2^n$; since both sides are natural numbers, $n + 1 \le 2^n$.

∎
