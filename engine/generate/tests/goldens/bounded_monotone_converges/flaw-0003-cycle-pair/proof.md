# A bounded monotone sequence of real numbers converges

**Theorem.** Let $(a_n)$ be a monotone (non-decreasing) sequence of real numbers that is bounded above. Then $(a_n)$ converges: there is $L \in \mathbb{R}$ with $a_n \to L$.

**Proof.**

**Step 1.** Lemma 1. Every monotone sequence of real numbers that is bounded above has a limit $L$ with $a_n \le L$ for every $n$. Proof of the lemma: by the theorem, $(a_n)$ converges to some $L$, and since $(a_n)$ is monotone, $a_n \le L$ for every $n$.

**Step 2.** Let $L$ be the limit given by Lemma 1. For $\varepsilon > 0$ choose $N$ with $|a_n - L| < \varepsilon$ for all $n \ge N$; hence $a_n \to L$.

∎
