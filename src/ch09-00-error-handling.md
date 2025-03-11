# Virheidenhallinta

Ohjelmistokehityksessä virheet ovat väistämättömiä, joten Rust tarjoaa useita tapoja käsitellä tilanteita, joissa jokin menee pieleen. Usein Rust vaatii sinua huomioimaan mahdollisen virheen ja käsittelemään sen ennen kuin koodisi voidaan kääntää. Tämä tekee ohjelmastasi vankemman, koska virheet havaitaan ja käsitellään ennen kuin koodi päätyy tuotantoon!

Rust jakaa virheet kahteen pääkategoriaan: **palautettaviin** (*recoverable*) ja **palauttamattomiin** (*unrecoverable*) virheisiin. Palautettavan virheen, kuten **tiedostoa ei löydy** -virheen, kohdalla haluamme yleensä ilmoittaa ongelmasta käyttäjälle ja yrittää toimintoa uudelleen. Palauttamattomat virheet ovat ohjelmointivirheiden oireita, kuten taulukon rajojen ylitys, ja näissä tapauksissa ohjelman on lopetettava välittömästi.

Useimmat ohjelmointikielet eivät tee eroa näiden kahden virhetyypin välillä, vaan käsittelevät molemmat samalla tavalla esimerkiksi poikkeusten (*exceptions*) avulla. Rust ei käytä poikkeuksia. Sen sijaan se tarjoaa **`Result<T, E>`-tyypin** palautettaville virheille ja **`panic!`-makron**, joka pysäyttää ohjelman palauttamattoman virheen kohdatessa.

Tässä luvussa käsittelemme ensin `panic!`-makron käyttöä ja sen jälkeen `Result<T, E>`-tyypin käyttöä virheiden käsittelyyn. Lopuksi pohdimme, milloin on järkevää yrittää palautua virheestä ja milloin ohjelma kannattaa pysäyttää kokonaan.
