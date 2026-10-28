# All horses have the same colour

**Theorem.** For every $n \ge 1$, in any set of $n$ horses all the horses have the same colour.

**Proof.**

**Step 1.** We argue by induction on $n$. Base case $n = 1$: a set of one horse is trivially monochromatic.

**Step 2.** Induction step: let $S$ be a set of $n + 1$ horses and let $a, b \in S$ (if $a = b$ there is nothing to show). Removing $b$ leaves $n$ horses, which share a colour by the induction hypothesis; so do the $n$ horses left after removing $a$.

**Step 3.** These two sets of $n$ horses overlap: together they omit only $a$ and $b$, so their overlap has $n - 1 \ge 1$ horses. Take a horse $z$ in both.

**Step 4.** Then $a$ and $z$ share a colour, and $z$ and $b$ share a colour; hence $a$ and $b$ share a colour, so all $n + 1$ horses agree.

∎
