# Luettelotyypit (Enums) ja mallintaminen

Tässä luvussa tarkastelemme _luettelotyyppejä_, joita kutsutaan myös _enum_-tyypeiksi. Enumit mahdollistavat tyypin määrittelyn luettelemalla sen mahdolliset _variantit_. Ensin määrittelemme ja käytämme enumia osoittaaksemme, kuinka enum voi tallentaa sekä merkitystä että dataa. 

Seuraavaksi tutkimme erityisen hyödyllistä `Option`-enumia, joka ilmaisee, että arvo voi olla joko jotain tai ei mitään. Sitten perehdymme `match`-lausekkeeseen, joka tekee erilaisten enum-arvojen käsittelystä helppoa ja selkeää. Lopuksi käsittelemme `if let` -rakennetta, joka on toinen kätevä tapa käsitellä enum-arvoja yksinkertaisemmin.
