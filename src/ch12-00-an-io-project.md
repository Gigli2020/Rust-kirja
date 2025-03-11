# I/O-projekti: Komentorivityökalun rakentaminen

Tässä luvussa kertaamme monia aiemmin opittuja taitoja ja tutustumme muutamiin uusiin standardikirjaston ominaisuuksiin. Rakennamme komentorivityökalun, joka käyttää tiedostoja sekä komentorivin tulo- ja lähtötietoja, jotta voimme harjoitella Rustin käsitteitä, jotka olet jo oppinut.

Rustin nopeus, turvallisuus, yksittäisen binääritiedoston tuottaminen ja monialustatuki tekevät siitä erinomaisen kielen komentorivityökalujen rakentamiseen. Projektissamme toteutamme oman version klassisesta komentorivityökalusta `grep` (**g**lobally search a **r**egular **e**xpression and **p**rint).

Yksinkertaisimmassa tapauksessa `grep` etsii määritellystä tiedostosta määriteltyä merkkijonoa. Se saa argumentteina tiedostopolun ja etsittävän merkkijonon, lukee tiedoston, löytää rivit, jotka sisältävät annetun merkkijonon, ja tulostaa ne.

Projektin edetessä opimme tekemään komentorivityökalustamme käyttäjäystävällisemmän hyödyntämällä yleisiä terminaalin ominaisuuksia. Luemme ympäristömuuttujan arvon, jotta käyttäjä voi mukauttaa työkalun toimintaa. Lisäksi ohjaamme virheilmoitukset **standardivirta-ulostuloon** (`stderr`) tavallisen tulostusvirran (`stdout`) sijaan. Näin käyttäjä voi esimerkiksi ohjata onnistuneet tulokset tiedostoon samalla, kun virheilmoitukset näkyvät edelleen näytöllä.

Rust-yhteisön jäsen Andrew Gallant on jo luonut täysiverisen ja erittäin nopean `grep`-version nimeltä `ripgrep`. Meidän toteutuksemme on paljon yksinkertaisempi, mutta tämä luku antaa tarvittavan taustan, jotta voit ymmärtää oikean maailman projekteja, kuten `ripgrep`.

`grep`-projektimme yhdistää useita tähän mennessä oppimiamme konsepteja:

- Koodin organisointi ([Luku 7][ch7])
- Vektorien ja merkkijonojen käyttö ([Luku 8][ch8])
- Virheiden käsittely ([Luku 9][ch9])
- Traitit ja eliniät oikeissa tilanteissa ([Luku 10][ch10])
- Testien kirjoittaminen ([Luku 11][ch11])

Lisäksi tutustumme lyhyesti **sulkeisiin** (*closures*), **iteraattoreihin** ja **trait-olioihin**, joita käsitellään yksityiskohtaisemmin **Luvuissa 13 ja 18**.

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch18]: ch18-00-oop.html
