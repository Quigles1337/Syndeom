# A bounded monotone sequence of real numbers converges

**Theorem.** Let $(a_n)$ be a monotone (non-decreasing) sequence of real numbers that is bounded above. Then $(a_n)$ converges: there is $L \in \mathbb{R}$ with $a_n \to L$.

**Proof.**

**Step 1.** Since $(a_n)$ is bounded above, the set $\{a_n : n \in \mathbb{N}\}$ is non-empty and bounded above, so by the completeness of $\mathbb{R}$ it has a least upper bound; let $L = \sup_n a_n$.

**Step 2.** Since $(a_n)$ is monotone, $a_n \to L$: given $\varepsilon > 0$, by the definition of the supremum there is $N$ with $L - \varepsilon < a_N$, and then $L - \varepsilon < a_N \le a_n \le L$ for all $n \ge N$.

∎
