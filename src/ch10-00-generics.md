# Geneeriset tyypit, traitit ja eliniät

Jokaisella ohjelmointikielellä on työkaluja, joilla voidaan tehokkaasti vähentää toistuvuutta koodissa. Rustissa yksi tällainen työkalu on **geneerisyys** (*generics*): abstraktit paikkamerkit konkreettisille tyypeille tai muille ominaisuuksille. Voimme määrittää geneeristen tyyppien käyttäytymisen tai niiden suhteen toisiin geneerisiin tyyppeihin, tietämättä tarkalleen, mitä niiden tilalla tulee olemaan kääntö- ja suoritusaikana.

Funktiot voivat ottaa parametreja, joiden tyyppi on geneerinen, samalla tavalla kuin ne ottavat parametreja, joiden arvoja ei tunneta etukäteen. Olemme jo käyttäneet geneerisiä tyyppejä aiemmin, esimerkiksi:

- Luvussa 6 **`Option<T>`**
- Luvussa 8 **`Vec<T>`** ja **`HashMap<K, V>`**
- Luvussa 9 **`Result<T, E>`**

Tässä luvussa opit määrittelemään omia tyyppejä, funktioita ja metodeja geneerisillä tyypeillä!

Ensin tarkastelemme, miten voimme vähentää koodin toistuvuutta siirtämällä toistuvaa koodia erilliseen funktioon. Tämän jälkeen käytämme samaa tekniikkaa muuntaaksemme kaksi erillistä funktiota geneeriseksi funktioksi. Opimme myös, miten geneerisiä tyyppejä voidaan käyttää rakenteissa (*structs*) ja luetteloissa (*enums*).

Seuraavaksi käsittelemme **traitteja**, joilla voidaan määritellä geneeristä käyttäytymistä. Traitteja voi yhdistää geneeristen tyyppien kanssa, jolloin voidaan rajoittaa, millaiset tyypit ovat kelvollisia.

Lopuksi tarkastelemme **elinikiä** (*lifetimes*), jotka ovat geneeristen tyyppien erityismuoto. Ne antavat kääntäjälle tietoa siitä, miten viittaukset liittyvät toisiinsa, varmistaen, että viittaukset pysyvät kelvollisina useammissa tilanteissa.

---

## Toiston poistaminen siirtämällä koodi funktioon

Geneerisyys mahdollistaa tiettyjen tyyppien korvaamisen paikkamerkillä, joka voi edustaa useita eri tyyppejä. Ennen kuin syvennymme geneeriseen syntaksiin, tarkastelemme, kuinka voimme poistaa toistuvaa koodia ilman geneerisiä tyyppejä – siirtämällä se funktioon. Sen jälkeen käytämme samaa menetelmää muuttaaksemme funktiosta geneerisen.

Aloitetaan lyhyellä ohjelmalla, joka etsii suurimman luvun listasta.

### Esimerkki: Suurimman luvun etsiminen listasta

Alla oleva ohjelma (Listing 10-1) tallentaa listan kokonaislukuja muuttujaan `number_list`. Se asettaa ensimmäisen luvun suurimmaksi ja käy sitten kaikki listan luvut läpi. Jos nykyinen luku on suurempi kuin jo löydetty suurin luku, se päivitetään. Lopuksi `largest`-muuttujaan jää suurin luku, tässä tapauksessa 100.

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

Olemme nyt saaneet tehtäväksi löytää suurimman luvun **kahdesta eri listasta**. Voimme ratkaista tämän kopioimalla saman koodin kahteen eri paikkaan, kuten alla:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

Vaikka tämä toimii, koodin kopiointi on huono käytäntö, koska se tekee ohjelmasta vaikeammin ylläpidettävän ja virhealttiimman. Jos haluamme muuttaa logiikkaa myöhemmin, meidän on päivitettävä molemmat kopiot.

Parempi ratkaisu on luoda erillinen funktio, joka toimii minkä tahansa listan kanssa. Tämä tekee koodista selkeämmän ja abstraktimman.

### Abstraktin funktion määrittely

Alla olevassa esimerkissä (Listing 10-3) siirrämme suurimman luvun etsintälogiikan `largest`-nimiseen funktioon. Tämän jälkeen kutsumme sitä kahdella eri listalla.

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

Tässä `largest`-funktiolla on parametri `list`, joka voi edustaa mitä tahansa `i32`-kokonaislukujen viipaletta. Kun funktiota kutsutaan, se käyttää annettua listaa.

Muutos prosessissa:

1. **Tunnista toistuva koodi**.
2. **Siirrä toistuva koodi erilliseen funktioon** ja määritä sen parametrit ja palautusarvo.
3. **Kutsu funktiota niissä kohdissa, joissa aiemmin oli toistuvaa koodia**.

Seuraavaksi sovellamme samaa lähestymistapaa **geneerisiin tyyppeihin**, jotta voimme vähentää koodin toistoa entisestään. Samalla tavalla kuin funktiot voivat toimia abstraktilla `list`-parametrilla, ne voivat myös toimia **abstrakteilla tyypeillä**.

Esimerkiksi, jos meillä olisi kaksi funktiota:

- Yksi, joka löytää suurimman arvon `i32`-listasta.
- Toinen, joka löytää suurimman arvon `char`-listasta.

Kuinka voisimme yhdistää nämä funktiot geneerisen tyypin avulla? Siitä jatkamme seuraavaksi!
