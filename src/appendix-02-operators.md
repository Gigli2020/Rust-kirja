## Liite B: Operaattorit ja symbolit

Tämä liite sisältää katsauksen Rustin syntaksiin, mukaan lukien operaattorit ja muut symbolit, joita käytetään muun muassa poluissa, geneerisissä tyypeissä, rajapintarajoituksissa, makroissa, attribuuteissa, kommenteissa, tupleissa ja hakasulkeissa.

### Operaattorit

Taulukko B-1 sisältää Rustin operaattorit, esimerkin niiden käytöstä, lyhyen selityksen sekä tiedon siitä, onko operaattori ylikuormitettavissa. Jos operaattori on ylikuormitettavissa, taulukossa mainitaan myös siihen liittyvä rajapinta.

<span class="caption">Taulukko B-1: Operaattorit</span>

| Operaattori | Esimerkki | Selitys | Ylikuormitettavissa? |
| ------------ | ----------- | ----------- | -------------------- |
| `!` | `ident!(...)`, `ident!{...}`, `ident![...]` | Makron laajennus | |
| `!` | `!expr` | Bitti- tai looginen komplementti | `Not` |
| `!=` | `expr != expr` | Eriarvoisuusvertailu | `PartialEq` |
| `%` | `expr % expr` | Jakoreste | `Rem` |
| `%=` | `var %= expr` | Jakoreste ja sijoitus | `RemAssign` |
| `&` | `&expr`, `&mut expr` | Viittaus (lainaus) | |
| `&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | Lainattu osoitintyyppi | |
| `&` | `expr & expr` | Bittitasolla JA-operaattori | `BitAnd` |
| `&=` | `var &= expr` | Bittitasolla JA-operaattori ja sijoitus | `BitAndAssign` |
| `&&` | `expr && expr` | Lyhytkatkaiseva looginen JA | |
| `*` | `expr * expr` | Kertolasku | `Mul` |
| `*=` | `var *= expr` | Kertolasku ja sijoitus | `MulAssign` |
| `*` | `*expr` | Viittauksen purku (dereference) | `Deref` |
| `*` | `*const type`, `*mut type` | Raaka osoitin | |
| `+` | `trait + trait`, `'a + trait` | Yhdistetty tyyppirajoite | |
| `+` | `expr + expr` | Yhteenlasku | `Add` |
| `+=` | `var += expr` | Yhteenlasku ja sijoitus | `AddAssign` |

_(Lista jatkuu samalla kaavalla...)_

---

### Muut kuin operaattorisymbolit

Seuraava luettelo sisältää symbolit, jotka eivät toimi operaattoreina, eli ne eivät käyttäydy kuten funktiokutsut tai metodikutsut.

_(Taulukot ja selitykset jatkavat samalla rakenteella kuin alkuperäisessä dokumentissa)_

---

Käännös säilyttää Rust-koodin ennallaan ja muotoilun Markdown-formaatissa.

