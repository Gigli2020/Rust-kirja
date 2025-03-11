# Automaattisten testien kirjoittaminen

Edsger W. Dijkstra kirjoitti vuonna 1972 esseessään *The Humble Programmer*:  
*"Ohjelmatestaus voi olla erittäin tehokas tapa osoittaa virheiden olemassaolo, mutta se on täysin riittämätön osoittamaan niiden puuttumista."*  
Tämä ei kuitenkaan tarkoita, ettemme voisi yrittää testata niin paljon kuin mahdollista!

Ohjelmamme **oikeellisuus** tarkoittaa sitä, kuinka hyvin koodi toteuttaa haluamamme toiminnallisuuden. Rust on suunniteltu huomioimaan ohjelmien oikeellisuus tarkasti, mutta sen varmistaminen on monimutkaista eikä helppoa todistaa. Rustin **tyyppijärjestelmä** kantaa suuren osan tästä vastuusta, mutta sekään ei voi estää kaikkia virheitä. Siksi Rust sisältää kattavan tuen **automaattisten testien kirjoittamiseen**.

Oletetaan, että kirjoitamme funktion `add_two`, joka lisää annetulle luvulle kaksi. Funktion allekirjoitus määrittää, että se ottaa **kokonaisluvun** parametrina ja palauttaa **kokonaisluvun** tuloksena. Kun toteutamme ja käännämme tämän funktion, Rust suorittaa tyyppitarkistuksen ja lainantarkistuksen varmistaakseen, että emme esimerkiksi yritä antaa sille **`String`-tyyppistä** arvoa tai kelpaamatonta viittausta.

Mutta Rust **ei voi tarkistaa**, tekeekö funktio juuri sitä, mitä tarkoitimme – eli palauttaako se parametrin **plus 2** eikä esimerkiksi **plus 10** tai **miinus 50**! Tällaisissa tilanteissa testit ovat ratkaisevia.

Voimme kirjoittaa testejä varmistaaksemme, että kun annamme funktiolle `add_two` arvon `3`, se palauttaa arvon `5`. Voimme ajaa nämä testit aina, kun teemme muutoksia koodiin, varmistaaksemme, että aiemmin toiminut käyttäytyminen ei ole muuttunut.

Testaaminen on monimutkainen taito. Emme voi kattaa yhden luvun aikana kaikkea, mitä hyvien testien kirjoittamiseen liittyy, mutta tässä luvussa käsittelemme Rustin testausjärjestelmän mekaniikkaa. Käymme läpi **annotaatiot ja makrot**, joita voit käyttää testien kirjoittamiseen, testien **ajamisen oletuskäyttäytymisen ja asetukset**, sekä **testien organisoinnin** yksikkötesteihin ja integraatiotesteihin.
