-- SYNDEOM Lean library root (ADR-0002). Only this project's modules are built by the gate;
-- Mathlib arrives from its cache at the pinned commit. The S2 seed witnesses (ADR-0011 D7) are
-- library modules so the gate builds them; generated artifacts are never built here. A seed whose
-- statement is false has a witness that proves the negation (s2_template 0.2.0).
import Syndeom.Fixtures.GoldenB
import Syndeom.Seeds.NatLeSq
import Syndeom.Seeds.NSqModFour
import Syndeom.Seeds.SqPlusOneGeTwoX
import Syndeom.Seeds.ContinuityIsNotUniform
import Syndeom.Seeds.XOverAbsX
import Syndeom.Seeds.RolleAbs
import Syndeom.Seeds.Horses
import Syndeom.Seeds.NLtTwoPow
import Syndeom.Seeds.Sqrt2Irrational
import Syndeom.Seeds.BoundedMonotoneConverges
import Syndeom.Seeds.BolzanoWeierstrass
import Syndeom.Seeds.BoundedContinuousSup
