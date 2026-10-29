# Every bounded sequence of real numbers has a convergent subsequence

**Theorem.** Let $(a_n)$ be a sequence of real numbers that is bounded: there is $M$ with $|a_n| \le M$ for every $n$. Then $(a_n)$ has a convergent subsequence: there are a strictly increasing $\varphi : \mathbb{N} \to \mathbb{N}$ and $L \in \mathbb{R}$ with $a_{\varphi(n)} \to L$.

**Proof.**

**Step 1.** Since $(a_n)$ is bounded, there is $M$ with $|a_n| \le M$ for every $n$, so every term lies in the closed interval $[-M, M]$, a bounded set.

**Step 2.** The interval $[-M, M]$ is compact, so a sequence in it has a convergent subsequence: there are a strictly increasing $\varphi : \mathbb{N} \to \mathbb{N}$ and $L \in \mathbb{R}$ with $a_{\varphi(n)} \to L$.

**Step 3.** Hence $(a_{\varphi(n)})$ is a convergent subsequence of $(a_n)$, as claimed.

∎
