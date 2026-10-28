# A continuous function equal to one at zero stays above one half nearby

**Theorem.** Let $f : \mathbb{R} \to \mathbb{R}$ be continuous with $f(0) = 1$. Then there is $\delta > 0$ such that $f(x) > 1/2$ for every $x$ with $|x| < \delta$.

**Proof.**

**Step 1.** Since $f$ is continuous, for $\varepsilon = 1/2$ there is $\delta > 0$ such that $|f(x) - f(y)| < 1/2$ for all $x, y$ with $|x - y| < \delta$.

**Step 2.** Take this $\delta$, and let $|x| < \delta$. Applying it with $y = 0$ gives $|f(x) - 1| < 1/2$, because $f(0) = 1$.

**Step 3.** Hence $f(x) > 1 - 1/2 = 1/2$.

∎
