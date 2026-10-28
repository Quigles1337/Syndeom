-- SYNDEOM Lean library root (ADR-0002). Only this project's modules are built by the gate;
-- Mathlib arrives from its cache at the pinned commit. The S2 seed witnesses (ADR-0011 D7) are
-- library modules so the gate builds them; generated artifacts are never built here.
import Syndeom.Fixtures.GoldenB
import Syndeom.Seeds.NatLeSq
import Syndeom.Seeds.NSqModFour
import Syndeom.Seeds.SqPlusOneGeTwoX
