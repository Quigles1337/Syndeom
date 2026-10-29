# A bounded continuous function on the real line attains its supremum

**Theorem.** Let $f : \mathbb{R} \to \mathbb{R}$ be continuous and bounded: there is $M$ with $|f(x)| \le M$ for every $x$. Then $f$ attains its supremum: there is $x^* \in \mathbb{R}$ with $f(x) \le f(x^*)$ for every $x$.

**Proof.**

**Step 1.** Since $f$ is bounded, its set of values $\{f(x) : x \in \mathbb{R}\}$ is non-empty and bounded above, so $S = \sup_x f(x)$ exists.

**Step 2.** By the definition of the supremum there is a sequence $(x_n)$ of real numbers with $f(x_n) \to S$.

**Step 3.** As in the extreme value theorem on a compact interval, the points $x_n$ lie in the bounded set $[-M, M]$, so by the Bolzano–Weierstrass theorem there are a strictly increasing $\varphi$ and $x^* \in \mathbb{R}$ with $x_{\varphi(n)} \to x^*$.

**Step 4.** By continuity, $f(x_{\varphi(n)}) \to f(x^*)$; since also $f(x_{\varphi(n)}) \to S$ and limits are unique, $f(x^*) = S$, so $f(x) \le S = f(x^*)$ for every $x$.

∎
