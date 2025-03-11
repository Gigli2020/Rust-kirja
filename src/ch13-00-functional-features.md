# Funktionaalisen ohjelmoinnin ominaisuudet: Iteraattorit ja sulkeiset

Rustin suunnittelu on saanut vaikutteita monista eri kielistä ja ohjelmointitekniikoista.
Yksi merkittävä vaikutus on tullut _funktionaalisesta ohjelmoinnista_.

Funktionaalisessa ohjelmointityylissä käytetään usein funktioita arvoina siten, että:

- Funktiot voidaan antaa argumentteina muille funktioille.
- Funktiot voivat palauttaa muita funktioita.
- Funktiot voidaan tallentaa muuttujiin ja suorittaa myöhemmin.

Tässä luvussa emme keskity teoreettisiin määritelmiin siitä, mitä funktionaalinen ohjelmointi
on tai ei ole, vaan tarkastelemme Rustin ominaisuuksia, jotka muistuttavat funktionaalisten kielten ominaisuuksia.

Erityisesti käsittelemme:

- _Sulkeisia_ (*closures*), jotka ovat muuttujissa säilytettäviä funktiomaisia rakenteita.
- _Iteraattoreita_ (*iterators*), joiden avulla voidaan käsitellä peräkkäisiä elementtejä tehokkaasti.
- Sulkeisten ja iteraattoreiden käyttöä luvun 12 I/O-projektin parantamisessa.
- Sulkeisten ja iteraattoreiden suorituskykyä (Vihje: ne ovat nopeampia kuin saatat kuvitella!).

Olemme jo käsitelleet joitakin Rustin funktionaalisia ominaisuuksia, kuten **mallintamiseen perustuvaa ohjelmointia** (*pattern matching*) ja **enum-tyyppejä**.
Sulkeisten ja iteraattoreiden hallinta on tärkeää **idiomaattisen ja suorituskykyisen Rust-koodin** kirjoittamisessa, joten omistamme koko luvun näiden aiheiden syventämiseen.
