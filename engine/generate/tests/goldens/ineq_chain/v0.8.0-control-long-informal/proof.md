# An inequality in two real variables

**Theorem.** For all real numbers $x$ and $y$, $6xy + 2x + 8y + 3|x + y| \le 9x^2 - 2xy + 18y^2 - 8y + 15 + 3|x| + 3|y|$.

**Proof.**

**Step 1.** Lemma 1. For all real numbers $x$ and $y$, $6xy + y^2 + 2x + 10y + 1 + 3|x + y| \le x^2 + 6xy + y^2 + 10y + 2 + 3|x + y|$; it follows from $(x - 1)^2 \ge 0$, since squares are non-negative.

**Step 2.** Lemma 2. For all real numbers $x$ and $y$, $5x^2 + 4xy + 9y^2 + 2x + 4y + 8 + 2|x| + 2|y| + |x + y| \le 5x^2 + 4xy + 10y^2 + 2x + 2y + 9 + 2|x| + 2|y| + |x + y|$; it follows from $(y - 1)^2 \ge 0$, since squares are non-negative.

**Step 3.** Since squares are non-negative, $(y + 1)^2 \ge 0$, so $6xy + 2x + 8y + 3|x + y| \le 6xy + y^2 + 2x + 10y + 1 + 3|x + y|$.

**Step 4.** By Lemma 1, $6xy + y^2 + 2x + 10y + 1 + 3|x + y| \le x^2 + 6xy + y^2 + 10y + 2 + 3|x + y|$.

**Step 5.** Since squares are non-negative, $(x + y)^2 \ge 0$, so $x^2 + 6xy + y^2 + 10y + 2 + 3|x + y| \le 2x^2 + 8xy + 2y^2 + 10y + 2 + 3|x + y|$.

**Step 6.** Since squares are non-negative, $(y - 1)^2 \ge 0$, so $2x^2 + 8xy + 2y^2 + 10y + 2 + 3|x + y| \le 2x^2 + 8xy + 3y^2 + 8y + 3 + 3|x + y|$.

**Step 7.** Since squares are non-negative, $(x - y)^2 \ge 0$, so $2x^2 + 8xy + 3y^2 + 8y + 3 + 3|x + y| \le 3x^2 + 6xy + 4y^2 + 8y + 3 + 3|x + y|$.

**Step 8.** By the two-term AM-GM inequality, $2y \le y^2 + 1$, so $3x^2 + 6xy + 4y^2 + 8y + 3 + 3|x + y| \le 3x^2 + 6xy + 5y^2 + 6y + 4 + 3|x + y|$.

**Step 9.** By the triangle inequality, $|x + y| \le |x| + |y|$, so $3x^2 + 6xy + 5y^2 + 6y + 4 + 3|x + y| \le 3x^2 + 6xy + 5y^2 + 6y + 4 + |x| + |y| + 2|x + y|$.

**Step 10.** By the triangle inequality, $|x + y| \le |x| + |y|$, so $3x^2 + 6xy + 5y^2 + 6y + 4 + |x| + |y| + 2|x + y| \le 3x^2 + 6xy + 5y^2 + 6y + 4 + 2|x| + 2|y| + |x + y|$.

**Step 11.** Since squares are non-negative, $(y + 1)^2 \ge 0$, so $3x^2 + 6xy + 5y^2 + 6y + 4 + 2|x| + 2|y| + |x + y| \le 3x^2 + 6xy + 6y^2 + 8y + 5 + 2|x| + 2|y| + |x + y|$.

**Step 12.** Since squares are non-negative, $(x + 1)^2 \ge 0$, so $3x^2 + 6xy + 6y^2 + 8y + 5 + 2|x| + 2|y| + |x + y| \le 4x^2 + 6xy + 6y^2 + 2x + 8y + 6 + 2|x| + 2|y| + |x + y|$.

**Step 13.** By the two-term AM-GM inequality, $2xy \le x^2 + y^2$, so $4x^2 + 6xy + 6y^2 + 2x + 8y + 6 + 2|x| + 2|y| + |x + y| \le 5x^2 + 4xy + 7y^2 + 2x + 8y + 6 + 2|x| + 2|y| + |x + y|$.

**Step 14.** Since squares are non-negative, $(y - 1)^2 \ge 0$, so $5x^2 + 4xy + 7y^2 + 2x + 8y + 6 + 2|x| + 2|y| + |x + y| \le 5x^2 + 4xy + 8y^2 + 2x + 6y + 7 + 2|x| + 2|y| + |x + y|$.

**Step 15.** Since squares are non-negative, $(y - 1)^2 \ge 0$, so $5x^2 + 4xy + 8y^2 + 2x + 6y + 7 + 2|x| + 2|y| + |x + y| \le 5x^2 + 4xy + 9y^2 + 2x + 4y + 8 + 2|x| + 2|y| + |x + y|$.

**Step 16.** By Lemma 2, $5x^2 + 4xy + 9y^2 + 2x + 4y + 8 + 2|x| + 2|y| + |x + y| \le 5x^2 + 4xy + 10y^2 + 2x + 2y + 9 + 2|x| + 2|y| + |x + y|$.

**Step 17.** By the two-term AM-GM inequality, $2y \le y^2 + 1$, so $5x^2 + 4xy + 10y^2 + 2x + 2y + 9 + 2|x| + 2|y| + |x + y| \le 5x^2 + 4xy + 11y^2 + 2x + 10 + 2|x| + 2|y| + |x + y|$.

**Step 18.** By the two-term AM-GM inequality, $2y \le y^2 + 1$, so $5x^2 + 4xy + 11y^2 + 2x + 10 + 2|x| + 2|y| + |x + y| \le 5x^2 + 4xy + 12y^2 + 2x - 2y + 11 + 2|x| + 2|y| + |x + y|$.

**Step 19.** Since squares are non-negative, $(x - y)^2 \ge 0$, so $5x^2 + 4xy + 12y^2 + 2x - 2y + 11 + 2|x| + 2|y| + |x + y| \le 6x^2 + 2xy + 13y^2 + 2x - 2y + 11 + 2|x| + 2|y| + |x + y|$.

**Step 20.** By the two-term AM-GM inequality, $2y \le y^2 + 1$, so $6x^2 + 2xy + 13y^2 + 2x - 2y + 11 + 2|x| + 2|y| + |x + y| \le 6x^2 + 2xy + 14y^2 + 2x - 4y + 12 + 2|x| + 2|y| + |x + y|$.

**Step 21.** Since squares are non-negative, $(y - 1)^2 \ge 0$, so $6x^2 + 2xy + 14y^2 + 2x - 4y + 12 + 2|x| + 2|y| + |x + y| \le 6x^2 + 2xy + 15y^2 + 2x - 6y + 13 + 2|x| + 2|y| + |x + y|$.

**Step 22.** By the two-term AM-GM inequality, $2x \le x^2 + 1$, so $6x^2 + 2xy + 15y^2 + 2x - 6y + 13 + 2|x| + 2|y| + |x + y| \le 7x^2 + 2xy + 15y^2 - 6y + 14 + 2|x| + 2|y| + |x + y|$.

**Step 23.** By the two-term AM-GM inequality, $2xy \le x^2 + y^2$, so $7x^2 + 2xy + 15y^2 - 6y + 14 + 2|x| + 2|y| + |x + y| \le 8x^2 + 16y^2 - 6y + 14 + 2|x| + 2|y| + |x + y|$.

**Step 24.** By the two-term AM-GM inequality, $2xy \le x^2 + y^2$, so $8x^2 + 16y^2 - 6y + 14 + 2|x| + 2|y| + |x + y| \le 9x^2 - 2xy + 17y^2 - 6y + 14 + 2|x| + 2|y| + |x + y|$.

**Step 25.** Since squares are non-negative, $(y - 1)^2 \ge 0$, so $9x^2 - 2xy + 17y^2 - 6y + 14 + 2|x| + 2|y| + |x + y| \le 9x^2 - 2xy + 18y^2 - 8y + 15 + 2|x| + 2|y| + |x + y|$.

**Step 26.** By the triangle inequality, $|x + y| \le |x| + |y|$, so $9x^2 - 2xy + 18y^2 - 8y + 15 + 2|x| + 2|y| + |x + y| \le 9x^2 - 2xy + 18y^2 - 8y + 15 + 3|x| + 3|y|$.

∎
