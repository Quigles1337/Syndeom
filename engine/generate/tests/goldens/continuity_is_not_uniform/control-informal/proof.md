# A continuous function equal to one at zero stays above one half nearby

**Theorem.** Let $f : \mathbb{R} \to \mathbb{R}$ be continuous with $f(0) = 1$. Then there is $\delta > 0$ such that $f(x) > 1/2$ for every $x$ with $|x| < \delta$.

**Proof.**

**Step 1.** Since $f$ is continuous at $0$, for $\varepsilon = 1/2$ there is $\delta > 0$ such that $|f(x) - f(0)| < 1/2$ for all $x$ with $|x - 0| < \delta$.

**Step 2.** Take this $\delta$, and let $|x| < \delta$. Then $|f(x) - 1| < 1/2$, because $f(0) = 1$.

**Step 3.** Hence $f(x) > 1 - 1/2 = 1/2$.

∎
