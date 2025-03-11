# Yleiset kokoelmat

Rustin standardikirjasto sisältää joukon erittäin hyödyllisiä tietorakenteita, joita kutsutaan **kokoelmiksi** (*collections*). Useimmat muut tietotyypit edustavat yksittäistä arvoa, mutta kokoelmat voivat sisältää useita arvoja. Toisin kuin sisäänrakennetut taulukot (*arrays*) ja tuplet (*tuples*), näiden kokoelmien osoittama data tallennetaan **kekoon** (*heap*). Tämä tarkoittaa, että tietomäärää ei tarvitse tietää kääntäjän aikana, ja se voi kasvaa tai pienentyä ohjelman suorituksen aikana. Jokaisella kokoelmalla on omat ominaisuutensa ja kustannuksensa, ja sopivan kokoelman valitseminen kuhunkin tilanteeseen on taito, jonka kehität ajan myötä. Tässä luvussa käsittelemme kolmea Rust-ohjelmissa usein käytettyä kokoelmaa:

- **Vektori** (*vector*) mahdollistaa muuttuvan määrän arvoja, jotka sijaitsevat vierekkäin muistissa.
- **Merkkijono** (*string*) on kokoelma merkkejä. Olemme aiemmin maininneet `String`-tyypin, mutta tässä luvussa perehdymme siihen tarkemmin.
- **Hajautustaulu** (*hash map*) mahdollistaa arvojen yhdistämisen tiettyihin avaimiin. Se on erityinen toteutus yleisemmästä tietorakenteesta, jota kutsutaan **mapiksi**.

Jos haluat oppia lisää Rustin tarjoamista muista kokoelmista, katso [dokumentaatio](../std/collections/index.html).

Tässä luvussa käymme läpi, miten luodaan ja päivitetään vektoreita, merkkijonoja ja hajautustauluja sekä mikä tekee kustakin erityisen.

