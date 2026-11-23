<p align="center">
  <img src="../../ui/public/app-mark.svg" width="112" height="112" alt="Twitch Minerin logo">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Louhii Twitch Dropseja, merkkejä ja kanavapisteitä niin monella tilillä kuin haluat, ilman selainta ja ilman avointa Twitch-välilehteä.</strong>
</p>

<p align="center">
  Työpöytäsovellus Linuxille, Windowsille ja macOS:lle tai Docker-image, jossa on sama verkkohallintapaneeli ja jota voi ajaa kotipalvelimella tai Raspberry Pi:llä. Jokainen tili käyttää noin 3 Mt muistia eikä juuri lainkaan suoritinta.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Uusin julkaisu"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="Lataukset"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Alustat: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Docker-image alustoille amd64 ja arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT-lisenssi"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Katselukerrat">
</p>

<p align="center">
  <a href="#download"><strong>Lataa</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Ilmoita bugista</a>
</p>

<p align="center">
  <a href="../../README.md">English</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <a href="README.es-ES.md">Español&nbsp;(España)</a> ·
  <a href="README.es-419.md">Español&nbsp;(Latinoamérica)</a> ·
  <a href="README.pt-PT.md">Português&nbsp;(Portugal)</a> ·
  <a href="README.pt-BR.md">Português&nbsp;(Brasil)</a> ·
  <a href="README.it.md">Italiano</a> ·
  <a href="README.nl.md">Nederlands</a> ·
  <a href="README.pl.md">Polski</a> ·
  <a href="README.ru.md">Русский</a> ·
  <a href="README.uk.md">Українська</a> ·
  <a href="README.cs.md">Čeština</a> ·
  <a href="README.hu.md">Magyar</a> ·
  <a href="README.ro.md">Română</a> ·
  <a href="README.bg.md">Български</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.tr.md">Türkçe</a> ·
  <a href="README.sv.md">Svenska</a> ·
  <a href="README.da.md">Dansk</a> ·
  <a href="README.nb.md">Norsk&nbsp;bokmål</a> ·
  <strong>Suomi</strong> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.zh-TW.md">繁體中文</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.vi.md">Tiếng&nbsp;Việt</a> ·
  <a href="README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="../media/fi/presentation.avif" alt="Videoesittely Twitch Minerista">
</p>

## <a name="download"></a>Lataus

Lataa järjestelmällesi sopiva tiedosto
[uusimmasta julkaisusta](https://github.com/SunkwiBOT/twitch-miner/releases/latest).

| Järjestelmä | Paketti |
|---|---|
| **Windows** · x64 | [Asennusohjelma](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_x64-setup.exe) tai [siirrettävä suoritettava tiedosto](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_x64_portable.exe) |
| **Windows** · ARM64 | [Asennusohjelma](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_arm64-setup.exe) tai [siirrettävä suoritettava tiedosto](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_arm64_portable.exe) |
| **macOS** 13 tai uudempi · universaali: Intel ja Apple Silicon | [Levykuva](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_macos_universal.dmg) (`.dmg`) |
| **Linux** · x86-64 | [`.AppImage`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.AppImage), [`.deb`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.deb) (Debian, Ubuntu) tai [`.rpm`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.rpm) (Fedora, openSUSE) |
| **Linux** · ARM64 | [`.AppImage`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_arm64.AppImage), [`.deb`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_arm64.deb) (Debian, Ubuntu, Raspberry Pi OS) tai [`.rpm`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_arm64.rpm) (Fedora, openSUSE) |
| **Linux-palvelin** · x86-64 tai ARM64 | Pelkkä palvelin, ilman Dockeria ([x86-64](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner-server_linux_x64) tai [ARM64](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner-server_linux_arm64)): katso [Ilman Dockeria](#systemd) |

macOS-versiota ei ole notarisoitu. Avaa sovellus ensimmäisellä kerralla: macOS estää sen.
Siirry sitten kohtaan **Järjestelmäasetukset › Tietosuoja ja suojaus** ja napsauta
**Avaa kuitenkin**.

Päivitykset on allekirjoitettu. Sovellus tarkistaa käynnistyessään ja sen jälkeen tunnin
välein, onko uutta versiota (tiheyttä voi muuttaa kohdassa **Asetukset › Tietoja**). Se ei
koskaan asenna versiota itse: asennat sen yhdellä napsautuksella näkyviin tulevasta
ilmoituksesta tai kohdasta **Asetukset › Tietoja**.

Tiedot sijaitsevat käyttäjän datakansiossa:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="features"></a>Ominaisuudet

### Tilit ja streamaajat

- **Niin monta tiliä kuin haluat yhdessä sovelluksessa.** Jokainen tili louhii omillaan, omassa
  louhintatilassaan: **Drops ja pisteet** (yksi kanava parhaalle kampanjalle, toinen
  streamaajillesi), **Vain Drops** tai **Vain pisteet**.

  <img src="../media/fi/accounts-mining.avif" alt="Useita tilejä louhimassa samaan aikaan Etusivulla">

- **Ohjattu käyttöönotto jokaiselle uudelle tilille.** Kirjautumisen jälkeen voit aloittaa
  oletusasetuksista tai toisen tilin asetuksista. Valitse louhintatila, kampanjat ja
  streamaajat, ja sitten ”Aloita louhinta” tai ”Tallenna ja aloita myöhemmin”.

  <img src="../media/fi/account-setup.avif" alt="Uuden tilin ohjattu käyttöönotto vaihe vaiheelta">

- **Muutos muihin tileihin yhdellä napsautuksella.** Kun olet muuttanut jotain tilillä,
  vahvistusviesti tarjoaa vaihtoehdon ”Käytä muihin tileihin”. Muutos tehdään uudelleen
  valitsemillesi tileille, ja niiden muut asetukset säilyvät ennallaan.

  <img src="../media/fi/apply-to-accounts.avif" alt="Muutos tehdään uudelleen muille tileille toiminnolla ”Käytä muihin tileihin”">

- **Lisää streamaaja.** Se aloittaa heti oletusvalinnoilla: sovelluksen omilla tai niillä,
  jotka olet valinnut tälle tilille. Säädä niitä tarvittaessa saman tien: bonusarkut,
  raidit, hetket, katseluputki, chat, ennustukset.

  <img src="../media/fi/streamer-add.avif" alt="Streamaajan lisääminen Twitch-haulla ja sen jälkeen sen valinnat">

- **Muokkaa streamaajaa.** ”Muokkaa” avaa sen valinnat: bonusarkut, raidit, hetket,
  katseluputki, chat, ennustukset… Kytkin ottaa sen käyttöön tai poistaa sen käytöstä
  poistamatta sitä listalta.

  <img src="../media/fi/streamer-edit.avif" alt="Streamaajan valinnat avattuna sen riviltä">

- **Muokkaa useita streamaajia kerralla.** Valitse ne ja sitten ”Muokkaa N valittua
  streamaajaa”. Vain muuttamasi valinnat otetaan käyttöön. Valinta, joka vaihtelee niiden
  välillä, näyttää tekstin ”Vaihtelee”, ja sen kytkin muuttuu kahdeksi painikkeeksi,
  ”Kyllä” ja ”Ei”. Sovellus tarjoutuu sen jälkeen tallentamaan ne uusien streamaajien
  oletusvalinnoiksi.

  <img src="../media/fi/streamers-group-edit.avif" alt="Useiden streamaajien muokkaus, valintoina ”Vaihtelee”">

- **Tuo streamaajia.** Liitä lista (enintään 500 nimeä) tai käytä toisen tilin listaa tai
  tilin seuraamia kanavia. Lista tarkistetaan, jotta vain olemassa olevat kanavat
  lisätään.
  Reaaliaikaiset päivitykset kattavat 46 streamaajaa tiliä kohden: listan seuraavat eivät
  saa raideja, hetkiä eivätkä ennustuksia.

  <img src="../media/fi/streamers-import.avif" alt="Streamaajien tuonti: liitetty lista, toinen tili, seuratut kanavat">

- **Spam Points.** Lunasta kanavan oma palkinto useita kertoja peräkkäin. Valitse kanava
  Pisteet-listastasi tai mikä tahansa Twitch-haulla löytyvä kanava: sen palkinnot näkyvät
  tilin saldon kanssa, jota seurataan reaaliajassa. Aseta määrä (enintään 25, saldon
  rajoissa) ja valitse sitten ”Lunasta”. Tulos näkyy sarjan päätyttyä, ja jokaisesta
  sarjasta jää oma rivinsä Päiväkirjaan.

  <img src="../media/fi/spam-points.avif" alt="Spam Points: valittu kanava, 10 × lunastettu viestillinen palkinto, lunastukset ilmestyvät kanavan Twitch-chattiin ja sitten rivi Päiväkirjassa">

### Drops

- **Kaikki käynnissä olevat Drops-kampanjat.** Jokaisella kampanjalla on omat tietonsa:
  palkinnot, julkaisijatilin linkitys, ”Louhi ensin”. Lista tulee
  [Twitch Drops API](https://drops.sunkwi.com) -projektista ja päivittyy 5 minuutin
  välein.

  <img src="../media/fi/drops-catalogue.avif" alt="Luettelo kalenterina, haku ja kampanjan tiedot">

- **Oma louhintajärjestyksesi.** Vedä kampanjat haluamaasi järjestykseen. Uudet löytävät
  paikkansa itse: ensin prioriteettikategoriasi, sitten lähimpänä päättyvät tai ne, joissa on
  vähiten minuutteja jäljellä. Sulje pois kategorioita, sanoja, kampanja tai yksittäinen
  palkinto, ja ohita kampanjat, jotka eivät ehdi valmistua ajoissa. ”Louhi ensin”
  kampanjan tiedoissa nostaa sen kärkeen.

  <img src="../media/fi/drops-order.avif" alt="Louhintajärjestys, prioriteettikategoriat ja ohitukset">

- **Varasto ja ”Lunasta kaikki”.** Varasto-sivu jakaa palkinnot ryhmiin
  ”Lunastettavissa”, ”Kesken” ja ”Lunastetut”. Lunasta ne yksi kerrallaan tai kaikki
  kerralla, tai ota käyttöön ”Lunasta dropsit automaattisesti”.
  Kun olet valinnut ”Linkitä julkaisijatili”, tili luetaan uudelleen heti, kun palaat
  sovellukseen, joten linkitys näkyy saman tien.

  <img src="../media/fi/drops-inventory.avif" alt="Varasto ja ”Lunasta kaikki” -painike">

- **”Omat streamaajat ensin Dropseissa ja merkeissä”.** Nykyisessä kampanjassa listasi streamaaja,
  jonka lähetys lasketaan kampanjaan, menee muiden kanavien edelle. Sama kanava edistää
  dropia ja kerryttää sinulle kanavansa pisteitä. Oletuksena käytössä.

  <img src="../media/fi/drops-streamers-first.avif" alt="Listan streamaaja, jota katsotaan sekä Dropsien että pisteiden vuoksi">

- **Palkintojen tärkeysjärjestys.** Kun kaksi kampanjaa on samalla sijalla, louhija
  valitsee ensin sen, jonka palkinnot vastaavat suosimaasi tyyppiä (merkki, emote, pelin
  sisäinen esine). Se ei koskaan ohita louhintajärjestystäsi.

- **Merkit (beta).** Merkit-sivu luettelee kaikki Twitchin yleiset chat-merkit ja sen,
  miten ne saa. Jokaisella tilillä on oma Merkit-sivunsa: kesken olevat merkit
  edistymisineen, vielä saatavat merkit ja tilin omat merkit. Louhija tavoittelee
  Drops-kampanjoiden ja Twitchin questien merkkejä ja ohittaa ne, jotka tilillä jo on.
  Joka päivä katsottava merkki louhitaan uudelleen joka päivä viimeiseen asti (”20 min
  päivässä, 3 päivää”), ja mikä tahansa kanava kampanjan kategorioista kelpaa. Arkku antaa
  ”1 satunnainen merkki, vaihtoehtoja N”. Tilausta vaativa merkki louhitaan, kun olet
  tilannut tai lahjoittanut tilauksen kampanjan aikana jollakin kategorian kanavalla ja
  Twitch laskee sen. Jo käynnissä oleva tilaus, Prime-tilaus tai uusinta ei kelpaa. Mitään
  ei louhita ennen kampanjan alkua. Vedä kohdassa ”Mitä louhitaan, järjestyksessä”
  prioriteettikategoriat, merkit ja muut Drops-kampanjat haluamaasi järjestykseen tai
  kytke jokin niistä pois. Twitchin quest louhitaan vain kanavalla, jolla Twitch tarjoaa
  sitä Drops-paneelissaan. Kehitys (Pichu → Pikachu) louhitaan heti, kun tilillä on
  merkki, josta se kehittyy: louhija ottaa sen merkin itse käyttöön juuri ennen eikä
  palauta vanhaa; ”Louhi merkkien kehitykset” tilin merkkiasetuksissa kytkee tämän pois.
  Merkin napsauttaminen avaa sen tiedot: koko vaatimuksen, kaikki kategoriat ja kanavat,
  kokoelman, kehityksen ja jokaisen tilin edistymisen. Tämä ominaisuus on beta-vaiheessa.

  <img src="../media/fi/badges.avif" alt="Merkit-sivu, tilin merkit ja sitten ”Mitä louhitaan, järjestyksessä”, jossa Merkit on vedetty ensimmäiseksi">

### Seuranta

- **Päiväkirja kaikesta ja jokainen pyyntö.** Päiväkirja tallentaa jokaisen tapahtuman:
  dropit, ansaitut ja käytetyt pisteet, arkut, vedot, raidit, alkavat lähetykset…
  Suodata sitä tilin, välilehden (Drops, Merkit, Pisteet, Muut) ja tyypin mukaan.
  **Tekniset lokit › Pyynnöt** listaa jokaisen Twitchille ja muille palveluille lähetetyn
  pyynnön tuloksineen ja kestoineen. Lokit ja pyynnöt eivät sisällä tokeneita tai
  salaisuuksia: ”Kopioi bugiraporttia varten” kopioi ne valmiiksi liitettäviksi
  bugiraporttiin
  [GitHubissa](https://github.com/SunkwiBOT/twitch-miner/issues) tai
  [Discordissa](https://discord.gg/support-bots-891685895781113886).

  <img src="../media/fi/journal-requests.avif" alt="Suodatettu Päiväkirja ja sitten Tekniset lokit › Pyynnöt">

- **Tilastot-sivu.** Ansaitut pisteet, lunastetut dropsit, katsottu aika ja voitetut
  ennustukset 7, 30 tai 90 päivältä, kärjessä olevan streamaajan tai kategorian kanssa
  jokaisen luvun takana. Streamaajakohtainen pistekäyrä näyttää saldon ajan myötä,
  merkintöineen katseluputkista, raideista, hetkistä ja ennustuksista sekä
  työkaluvihjeellä jokaisessa.

  <img src="../media/fi/stats.avif" alt="Tilastot-sivun ruudut 30 päivältä, sitten streamaajan pistekäyrä merkillä">

- **Keskeytys ja päivittäinen tauko.** Yksi keskeytyspainike, yhdelle tilille tai kaikille:
  keskeytä, kunnes jatkat, tai 30 minuutiksi, 1 tunniksi, 4 tunniksi, huomiseen klo 09.00
  asti tai mukautettuun päivämäärään ja aikaan asti; keskeytyksen aikana sama painike on
  ”Jatka”. Merkki näyttää, milloin ajastettu keskeytys päättyy, ja jokainen tili
  sivupalkissa näyttää tilansa: käynnissä, keskeytetty tai ongelma, jota kannattaa katsoa.
  Päivittäinen tauko pysäyttää kaikkien tilien louhinnan joka päivä kahden kellonajan
  välillä valitsemassasi aikavyöhykkeessä ja jatkaa sitä sen päätyttyä, omalla merkillään
  yläpalkissa.
- **CSV-vienti ja määrät kategorioittain.** Päiväkirja vie lunastetut dropit
  CSV-tiedostona (päivämäärä, tili, kategoria, kampanja, drop, minuutit) tilin mukaan
  suodatettuna, ja valintaikkuna listaa, montako droppia lunastettiin kussakin
  kategoriassa. Työpöytäsovellus tallentaa tiedoston tallennusikkunan kautta;
  verkkokäyttöliittymä lataa sen.

### Työpöydällä tai palvelimella

- **Sama käyttöliittymä kaikkialla.** Työpöytäsovellus (Linux, Windows, macOS) ja
  Docker-image näyttävät saman hallintapaneelin. Docker toimii VPS:llä, NAS-laitteella tai
  Raspberry Pi:llä ilman näyttöä tai selainta kyseisellä koneella: hallintapaneeli
  avautuu missä tahansa selaimessa.
  Puhelimessa hallintapaneeli mukautuu näyttöön, ja HTTPS:n kautta sen voi asentaa kuin sovelluksen.
- **Kevyt.** Noin 3 Mt muistia per tili, tuskin lainkaan suoritinta ja noin 12 Mt:n
  Docker-image.

### Tietoturva ja päivitykset

- **Istuntosi suojattuina.** Ei Twitch-salasanaa: kirjaudut osoitteessa
  `twitch.tv/activate` omasta selaimestasi. Istunnot tallennetaan levylle salattuina, ja
  voit perua ne [Twitchin asetuksista](https://www.twitch.tv/settings/connections).
  Työpöydällä valinnainen sovelluksen salasana lukitsee sovelluksen: mitään ei louhita
  ennen kuin se on kirjoitettu. Dockerissa hallintapaneelilla on oma salasanansa, se
  lukittuu muutaman minuutin käyttämättömyyden jälkeen ja toimii HTTPS:n yli proxyn
  takana. Viisi väärää salasanaa peräkkäin estää IP-osoitteen 30 sekunniksi.

  <img src="../media/fi/security-lock.avif" alt="Riippulukko: sovellus lukittuna ja sitten avattuna salasanallaan">

- **Allekirjoitetut päivitykset, ei koskaan automaattisesti.** Yläpalkkiin ilmestyy
  ”Päivitys”-merkki ja versionumero, ja työpöytäilmoitus kertoo samasta asiasta, oletuksena
  käytössä (kytkin kohdassa **Asetukset › Ilmoitukset** sulkee sen; selaimessa napsauta
  ”Salli”, kun sitä kysytään). ”Asenna ja käynnistä uudelleen” pyytää ensin vahvistuksen,
  minkä jälkeen sovellus asentaa version ja käynnistyy uudelleen. Jos sovelluksen salasana
  on asetettu, se käynnistyy lukittuna: kirjoita salasana uudelleen jatkaaksesi louhintaa.
  Vain projektin allekirjoittama versio asennetaan. Docker-hallintapaneeli ja
  systemd-binääri ilmoittavat myös ja kertovat suoritettavat komennot.

  <img src="../media/fi/updates.avif" alt="”Päivitys”-merkki, vahvistus ja sitten asennus">

### Lisäksi

- Kuten muissa louhijoissa: bonusarkut, raidit, hetket, yhteisötavoitteet, ennustukset
  (strategiat, panostuksen ajoitus, ehdot), katseluputket ja tilatut kanavat ensin,
  läsnäolo chatissa (voi poistaa käytöstä), uusien streamaajien oletusvalinnat, Discord-,
  Telegram- ja webhook-ilmoitukset kuvineen ja kullekin oma ”Testaa”-painike (palvelimella
  niiden kieli valitaan kohdassa **Asetukset › Ilmoitukset**), yleinen tai tilikohtainen
  proxy.
- 29 kieltä, tumma ja vaalea teema, työpöytäilmoitukset, ilmoitusalueen kuvake,
  käynnistys järjestelmän mukana.

  <img src="../media/fi/basics.avif" alt="Kielet, vaalea ja tumma teema, työpöytäilmoitus ja ilmoitusalueen kuvake">

- **Tilien lisääminen liittämällä tokenit.** Oletko jo kirjautunut muualla? Liitä luettelo käyttötokeneista, yksi riviä kohden (enintään 50); jokainen tarkistetaan reaaliajassa, laskurina ”Tarkistettu: n/N”, ja epäonnistuneet rivit voi yrittää uudelleen; sen jälkeen yksi asetusvaihe koskee koko erää. Tuonti käynnistyy taustalla ja näyttää kunkin tilin edistymisen reaaliajassa; sen voi perua kesken kaiken, ja myöhempi uudelleenyritys lähettää uudelleen vain epäonnistuneet tilit. Tällä tavalla tuodun tilin poistaminen tarjoaa myös tokenin kumoamisen Twitchissä.

> [!WARNING]
> Tämä projekti ei ole yhteydessä Twitchiin. Tilin automatisointi voi rikkoa Twitchin
> käyttöehtoja tai kampanjan sääntöjä.

## Sisältö

- [Vertailu](#comparison)
- [Docker](#docker)
- [Näin se toimii](#how-it-works)
- [Usein kysytyt kysymykset](#faq)
- [Liittyvät projektit](#related-projects)
- [Tietoturva](#security)
- [Kääntäminen lähdekoodista](#build)
- [Lisenssi](#license)

## <a name="comparison"></a>Vertailu

Näin
[Twitch Miner](https://github.com/SunkwiBOT/twitch-miner)
vertautuu
[Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
-ohjelmaan (kanavapisteet) ja
[TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) -ohjelmaan (Drops).
Tarkistettu niiden koodista ja README-tiedostoista marraskuussa 2026.

| | Tämä projekti | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops-kampanjat: löytää, louhii ja lunastaa ne | ✅ | ❌ ¹ | ✅ |
| Twitch-merkit: kaikkien merkkien lista, kunkin tilin merkit, merkki-Dropsit ja Twitchin questit louhitaan haluamassasi järjestyksessä (beta) | ✅ | ❌ | osittain ² |
| Kanavapisteet: arkut, raidit, hetket, ennustukset | ✅ | ✅ | ❌ |
| Spam Points: kanavan oma palkinto lunastettuna useita kertoja peräkkäin | ✅ | ❌ | ❌ |
| Drops ja kanavapisteet samanaikaisesti yhdellä tilillä | ✅ | osittain ¹ | ❌ |
| Niin monta tiliä kuin haluat yhdessä sovelluksessa | ✅ | ❌ ³ | ❌ |
| Usean tilin asetusten muuttaminen kerralla | ✅ | ❌ | ❌ |
| Graafinen käyttöliittymä | ✅ työpöytä ja verkko ⁴ | ❌ ⁵ | ✅ työpöytä |
| Verkkohallintapaneeli kotipalvelimelle (Docker), salasanalla suojattu, käytettävissä HTTPS:n kautta | ✅ | ❌ ⁵ | ❌ |
| Levyllä salatut istunnot, valinnainen sovelluksen salasana | ✅ | ❌ | ❌ |
| Käyttöliittymä 29 kielellä | ✅ | ❌ | osittain ⁶ |
| Allekirjoitetut päivitykset suoraan sovelluksesta (työpöytä) | ✅ | ❌ | ❌ |
| Ohjattu käyttöönotto jokaiselle uudelle tilille | ✅ | ❌ ⁵ | ❌ |
| Streamaajien lisäys ja muokkaus käyttöliittymässä, yksi kerrallaan tai useita kerralla | ✅ | ❌ ⁵ | — |
| Streamaajien tuonti haulla, listasta, toiselta tililtä tai seuratuista kanavista | ✅ | osittain ⁷ | — |
| Discord-, Telegram- ja webhook-ilmoitukset kuvineen | ✅ | osittain ⁸ | ❌ |
| Kalenteri kaikista Drops-kampanjoista, haulla | ✅ | ❌ | osittain ⁹ |
| Louhintajärjestys kampanjoittain, poissuljetut sanat, ohitettu kampanja tai palkinto | ✅ | ❌ | ❌ ¹⁰ |
| Listasi streamaajia katsotaan ensin kampanjan Dropseja varten | ✅ | osittain ¹¹ | ❌ |
| Suodatettava tapahtumapäiväkirja ja lista jokaisesta pyynnöstä käyttöliittymässä | ✅ | ❌ ⁵ | ❌ |
| Noin 3 Mt muistia per tili (Docker-palvelin) | ✅ | — ¹² | — ¹² |

¹ Se katsoo vain listansa streamaajia: se edistää heidän lähetystensä Dropseja ja
lunastaa ne valitsematta kampanjaa.\
² Asetus, jolla louhitaan tai jätetään pois Drops-kampanjat, joiden palkintoina on
merkkejä tai hymiöitä; ei Twitchin questeja eikä merkkilistaa.\
³ Yksi prosessi tiliä kohden.\
⁴ Tietokoneellasi työpöytäsovelluksena ja/tai palvelimella (VPS, NAS, Raspberry Pi)
Dockerilla, ilman graafista ympäristöä ja ilman selainta kyseisellä koneella:
hallintapaneeli avautuu missä tahansa selaimessa.\
⁵ Määritetään Python-tiedostossa; sen verkkosivu näyttää pistekaavioita ja
lokitiedoston.\
⁶ 21 kieltä.\
⁷ Se voi seurata kanavia, joita tili seuraa.\
⁸ Pelkkää tekstiä ilman kuvia. Siinä on myös Matrix, Pushover ja Gotify, joita tässä
projektissa ei ole.\
⁹ Kampanjalista suodattimilla, ilman kalenteria ja hakua.\
¹⁰ Siinä on prioriteettikategoriat ja ohitetut kategoriat, jotka valitaan kategorian nimen mukaan.\
¹¹ Oletuksena se katsoo ensin niitä listansa streamaajia, joiden lähetyksessä on Dropseja
(sen prioriteetti `DROPS`), mutta ei koskaan muuta kanavaa.\
¹² Ei mitattu.

## <a name="docker"></a>Docker

Palvelin näyttää saman hallintapaneelin selaimessa. Image on
`ghcr.io/sunkwibot/twitch-miner` alustoille `amd64` ja `arm64`. Kun uusi
versio julkaistaan, hallintapaneeli kertoo sen: `docker compose pull` ja sitten
`docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # hakee uusimman imagen joka käynnistyksessä
    restart: unless-stopped
    ports: ["8080:8080"]   # HTTPS-käänteisproxyn takana: poista, tai "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # pakollinen, vähintään 12 merkkiä
      # MINER_PUBLIC_URL: "https://miner.example.com"   # vain HTTPS-käänteisproxyn takana
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Avaa <http://localhost:8080> (tai koneen osoite verkossasi) ja syötä salasana.

### Muuttujat

| Muuttuja | Oletus | Tarkoitus |
|---|---|---|
| `MINER_PASSWORD` | ei mitään | Hallintapaneelin salasana, pakollinen: ilman sitä palvelin kieltäytyy käynnistymästä. Vähintään 12 merkkiä (lyhyemmästä näytetään varoitus käynnistyksessä). Esimerkkiarvo `change-me` hylätään. |
| `MINER_PASSWORD_FILE` | ei mitään | Lukee `MINER_PASSWORD`-arvon tiedostosta, esimerkiksi Docker-secretistä. Vain toinen, ei molempia. |
| `MINER_PUBLIC_URL` | ei mitään | Vain HTTPS-käänteisproxyn takana (Caddy, nginx, Traefik, Cloudflare Tunnel…): hallintapaneelin `https://`-osoite ilman polkua, esimerkiksi `https://miner.example.com`. Proxyn on välitettävä `X-Forwarded-Proto`, `X-Forwarded-For` ja websocket-yhteydet, minkä yleiset proxyt tekevät oletuksena, eikä kontin porttiin saa päästä suoraan. Hallintapaneeli hylkää silloin kaiken, mikä ei tule proxyn kautta HTTPS:llä, ja estää osoitteen 30 sekunniksi 5 väärän salasanan jälkeen. Välityspalvelimen on toimittava samalla koneella tai sen lähiverkossa: yhteys julkisesta osoitteesta hylätään. |
| `MINER_TRUSTED_HOPS` | `1` | Vain `MINER_PUBLIC_URL`-muuttujan kanssa: kuinka moni proxy lisää merkinnän `X-Forwarded-For`-otsakkeeseen, 1–10. `2`, kun proxyja on kaksi peräkkäin, esimerkiksi Cloudflare Traefikin edessä. |
| `MINER_PROXY_NETWORKS` | ei mitään | Vain `MINER_PUBLIC_URL`-asetuksen kanssa: verkot, joista välityspalvelin yhdistää, pilkuilla erotettuina: yksittäinen osoite tai verkko kuten `192.168.1.0/24` tai `2001:db8:1::/64`. Kun se on asetettu, välityspalvelin hyväksytään vain loopback-osoitteesta ja näistä verkoista, ei enää koko yksityisestä lähiverkosta. |
| `MINER_VAULT_PASSPHRASE` | ei mitään | Vain työpöytäsovelluksesta kopioidulle datakansiolle, jossa on asetettu sovelluksen salasana: avaa sen lukituksen käynnistyksessä. Tyhjästä pystytetty palvelin ei koskaan tarvitse sitä. |
| `MINER_VAULT_PASSPHRASE_FILE` | ei mitään | Lukee `MINER_VAULT_PASSPHRASE`-arvon tiedostosta. Vain toinen, ei molempia. |
| `DATA_DIR` | imagessa `/data`, muuten `data` työhakemistossa | Tietokanta, avaintiedosto ja lokit. |
| `LOG_DIR` | `$DATA_DIR/logs` | Lokitiedostot: yksi päivässä, viimeiset 14 säilytetään. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Kuunteluosoite: IP-osoite ja portti väliltä 1–65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Lokitaso. |

Palvelin lukee ja tarkistaa muuttujat kerran, käynnistyksessä. Virheellinen arvo pysäyttää
palvelimen ja näyttää viestin sen sijaan, että palvelin siirtyisi hiljaa varavaihtoehtoon.

Määritä käänteisen proxyn takana `MINER_PUBLIC_URL`: ilman sitä ensimmäinen proxyn kautta
tuleva pyyntö kirjoittaa lokeihin varoituksen, koska kirjautumisyritykset lasketaan
silloin proxyn osoitteelle, ja yhden vierailijan väärät salasanat lukitsisivat kaikki
ulos. Reaaliaikainen yhteys lähettää pingin 30 sekunnin välein, jotta proxy ei sulje sitä,
kun mitään ei tapahdu. Kun Twitch tai verkko pettää, palvelin vastaa `424`, ei koskaan
`502` tai `504`: Cloudflaren takana näet oikean virheen Cloudflaren sivun sijaan.

### <a name="systemd"></a>Ilman Dockeria (systemd)

Linux-palvelimella ilman Dockeria julkaisu sisältää palvelimen myös yhtenä tiedostona,
hallintapaneeli mukana: `twitch-miner-server_<versio>_linux_x64` tai
`twitch-miner-server_<versio>_linux_arm64`. Se on sama ohjelma, jota Docker-image
ajaa, staattisesti linkitettynä: mitään muuta ei tarvitse asentaa, lukuun ottamatta
järjestelmän CA-varmenteita, joita se lukee HTTPS:ää varten (Docker-image kopioi ne samasta
syystä). Se käyttää samoja muuttujia.

```bash
# ARM64: twitch-miner-server_<versio>_linux_arm64
sudo install -m 755 twitch-miner-server_<versio>_linux_x64 /usr/local/bin/twitch-miner
sudo install -m 600 /dev/null /etc/twitch-miner.env
```

Kirjoita oma salasanasi, vähintään 12 merkkiä, tiedostoon `/etc/twitch-miner.env`, jota
vain root voi lukea:

```ini
MINER_PASSWORD=change-me
```

Sitten palvelu tiedostoon `/etc/systemd/system/twitch-miner.service`:

```ini
[Unit]
Description=Twitch Miner
Wants=network-online.target
After=network-online.target

[Service]
ExecStart=/usr/local/bin/twitch-miner
EnvironmentFile=/etc/twitch-miner.env
Environment=DATA_DIR=/var/lib/twitch-miner
DynamicUser=yes
StateDirectory=twitch-miner
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now twitch-miner
```

systemd luo kansion `/var/lib/twitch-miner` tietoja varten ja ajaa palvelinta omalla
käyttäjällään, ei koskaan roottina. Avaa `http://<palvelimen osoite>:8080`; lokit löytyvät
komennolla `journalctl -u twitch-miner`.

Tämä tiedosto ei päivitä itseään. Kun hallintapaneeli näyttää uuden version, lataa uusi
tiedosto julkaisusta, korvaa sillä vanha ja käynnistä uudelleen:

```bash
# ARM64: twitch-miner-server_<versio>_linux_arm64
sudo install -m 755 twitch-miner-server_<versio>_linux_x64 /usr/local/bin/twitch-miner
sudo systemctl restart twitch-miner
```

## <a name="how-it-works"></a>Näin se toimii

Joka 20. sekunti jokainen tili valitsee kaksi kanavaansa: parhaan live-kanavan jononsa
kärjessä olevalle kampanjalle ja sitten pistelistansa streamaajat. Kampanja, jolla ei ole
live-kanavaa, ohitetaan seuraavan hyväksi, jolla sellainen on (”Ei kanavaa livenä” sen
jonossa); live-kanavat tarkistetaan uudelleen 90 sekunnin välein, ja tili palaa kampanjaan
heti, kun jokin sen streamaajista aloittaa livelähetyksen.

Twitch hyvittää tiliä kohden kerrallaan **yhden Drops-kampanjan** ja
**kaksi kanavaa pisteille**. Lokakuusta 2026 lähtien dropin voi lunastaa vain, kun
julkaisijatili on linkitetty. Linkittämättömät kampanjat edistyvät silti, ja hallintapaneeli
näyttää linkin.

Jos Twitch ei edistä ykköskampanjaa 30 minuutin aidon katselun jälkeen, se ei hyvitä sitä:
tili siirtää kampanjan syrjään, seuraavaksi sijoittunut kampanja saa paikan, ja uusi yritys
tapahtuu automaattisesti 6 tunnin kuluttua — aiemmin, jos edistystä ilmenee ensin, vaikka
uudelleen yritetty kampanja saattaa yhä odottaa parhaillaan louhittavan kampanjan takana.
Mikä tahansa edistyminen nollaa odotuksen, ja tämä koskee jokaista kampanjatyyppiä (Drops,
merkit, tehtävät). Jos Twitch hyvittää sen sijaan toisen saman kategorian kampanjan ja se valmistuu tai lunastetaan, syrjäänsiirto puretaan heti sen sijaan, että odotettaisiin koko 6 tuntia. Sovelluksen tai palvelimen uudelleenkäynnistys unohtaa kaikki syrjään
siirretyt kampanjat: jumissa oleva kampanja saa yksinkertaisesti uudet 30 minuuttia.

Kampanjaluettelo tulee [Twitch Drops API](https://drops.sunkwi.com) -projektista
(päivittyy 5 minuutin välein), koska Twitch piilottaa sen TV-asiakkaalta.

## <a name="faq"></a>Usein kysytyt kysymykset

### Riskit ja Twitch

**Voiko tilini saada porttikiellon?**\
Kukaan ei voi luvata, ettei niin käy. Tämä projekti ei ole yhteydessä Twitchiin, ja tilin
automatisointi voi rikkoa Twitchin käyttöehtoja tai kampanjan sääntöjä. Louhija katsoo
enintään kahta kanavaa tiliä kohden, mikä on Twitchin raja, eikä lataa videota. Päätös on
sinun.

**Katsotaanko videota oikeasti?**\
Ei, videota ei ladata. Dropseja varten louhija lukee 15 sekunnin välein lähetyksen
soittolistan heikoimmalla laadulla ja pyytää jokaisesta uudesta segmentistä vain
otsakkeen. Pisteitä varten se lähettää joka minuutti Twitchin ”katsottu minuutti”
-signaalin. Jokainen katsottu kanava näyttää, kuinka monta segmenttiä on pyydetty.

**Miksi enintään kaksi kanavaa?**\
Se on Twitchin raja: Twitch hyvittää tiliä kohden vain kaksi samanaikaisesti katsottua
kanavaa ja vain yhden Drops-kampanjan kerrallaan. **Drops ja pisteet** -tilassa toinen
kanava palvelee kampanjaa ja toinen streamaajiasi. ”Yksi kanava” -asetus kohdassa
**Tilin asetukset** on huomaamattomampi.

### Drops

**Kampanjaa ei louhita, tai drop ei edisty.**\
Avaa tilin **Drops**-sivu. Kohdassa ”Louhintajärjestys” jokainen odottava kampanja kertoo
syyn: ”Ei kanavaa livenä” tai ”Odottaa (yksi kampanja kerrallaan)”. ”Twitch ei laske
sitä — uusi yritys …” tarkoittaa, että 30 minuuttia aitoa katselua ei tuonut edistystä
Twitchin puolella: kampanja siirretään syrjään ja sitä yritetään uudelleen automaattisesti
näytettynä aikana, tai aiemmin, jos edistystä ilmenee ensin. Kampanja, joka ei näy tässä
listassa, on jätetty pois: kohdan ”Mitä louhitaan, järjestyksessä” vuoksi, ”Mitkä
kampanjat” -asetuksella, **Ohitukset**-välilehden säännöllä, siksi, ettei se ehdi
valmistua ajoissa, tai siksi, että kaikki sen jäljellä olevat palkinnot vaativat
tilauksen. Tilausta vaativa palkinto odottaa, kunnes tilaat tai lahjoitat tilauksen
kampanjan aikana jollakin kategorian kanavalla (jo käynnissä oleva tilaus, Prime-tilaus
tai uusinta ei kelpaa), joka päivä katsottava merkki odottaa seuraavaa päivää, mitään ei
louhita ennen kampanjan alkua, ja toista seuraava drop odottaa, kunnes ensimmäinen on
valmis. **Vain pisteet** -tilassa louhija ei valitse kampanjaa. ”Louhi ensin” kampanjan
tiedoissa nostaa sen kärkeen; ohitetun kampanjan saa takaisin valinnalla ”Louhi taas”
**Ohitukset**-välilehdellä. **Kampanjat**-välilehden **Estetyt**-ryhmä näyttää samat syyt.

**Pitääkö julkaisijatili linkittää?**\
Lunastamista varten kyllä: lokakuusta 2026 lähtien Twitch antaa lunastaa dropin vain, kun
julkaisijatili on linkitetty. Oletuksena linkittämätön kampanja edistyy silti, ja lunastus
odottaa linkitystä. ”Ei linkitetty” -merkki kertoo tästä, ja ”Linkitä julkaisijatili”
kampanjan tiedoissa avaa linkityssivun. Jos haluat louhia vain linkitettyjä pelejä,
valitse ”Vain kategoriat, joiden julkaisijatili on linkitetty”.

### Asennus ja tiedot

**Työpöytäsovellus vai Docker?**\
Työpöytäsovellus louhii, kun tietokoneesi on päällä, ja siinä on ilmoitusalueen kuvake,
valinnainen salasana ja päivitykset yhdellä napsautuksella. Docker louhii yötä päivää
koneella, joka on aina päällä (VPS, NAS, Raspberry Pi); avaat hallintapaneelin
selaimessa, ja sitä suojaa `MINER_PASSWORD`. Käyttöliittymä on sama.

**Missä tietoni ovat?**\
Datakansiossa: Linuxilla `~/.local/share/twitch-miner`, Windowsilla
`%APPDATA%\twitch-miner`, macOS:llä
`~/Library/Application Support/twitch-miner` ja Dockerilla `/data`-taltiossa.
Työpöydällä **Asetukset › Työpöytä** avaa sen yhdellä napsautuksella. Siellä ovat
`miner.db` (tilit, asetukset, päiväkirja), `master.key` (istuntoavain, kun sovelluksen
salasanaa ei ole asetettu) ja lokit.

**Miten päivitän?**\
Työpöytäsovellus tarkistaa uuden version käynnistyessään ja sen jälkeen tunnin välein
(muutettavissa kohdassa **Asetukset › Tietoja**), ja se voi ilmoittaa siitä sinulle.
Napsauta ”Päivitys”-merkkiä ja sitten ”Asenna ja käynnistä uudelleen”. Dockerilla
hallintapaneeli ilmoittaa asiasta: suorita `docker compose pull` ja sitten
`docker compose up -d`. Ilman Dockeria hallintapaneeli ilmoittaa samalla tavalla: katso
[Ilman Dockeria](#systemd), miten tiedosto vaihdetaan ja palvelu käynnistetään uudelleen.

**Unohdin sovelluksen salasanan.**\
Sitä ei voi palauttaa: juuri se salaa Twitch-istuntosi. Sulje sovellus, poista datakansio
ja lisää tilisi uudelleen; niiden asetukset on tehtävä uudelleen. Dockerilla
hallintapaneelin salasana vaihdetaan `MINER_PASSWORD`-muuttujassa, minkä jälkeen ajetaan
`docker compose up -d`.

### Pisteet ja ennustukset

**Montako pistettä tunnissa?**\
Se riippuu Twitchistä ja kustakin kanavasta. Louhija ansaitsee sen, minkä Twitch antaa
katsojalle: katselusta kertyvät pisteet, bonusarkut (noin yksi 15 minuutin välein) ja
katseluputken bonuksen. Tilaus kasvattaa pisteet tasosta riippuen 1,2–2-kertaisiksi, ja
oletuksena päällä oleva ”Tilatut kanavat ensin” nostaa nämä kanavat muun listasi edelle.
Etusivu näyttää ansaitut pisteet ja Päiväkirja jokaisen ansion.

**Ovatko vedot riskialttiita?**\
Kyllä, pisteitä voi menettää. Ennustukset ovat oletuksena pois päältä, streamaajakohtaisesti.
Asetat strategian, panostuksen ajoituksen, panoksen (oletuksena 5 % saldosta, enintään
50 000 pistettä), vähimmäissaldon ja halutessasi ehdon, kuten vähintään 1,5:n kertoimen.
Pidä panos pienenä.
Alle 10 pisteen panosta ei koskaan aseteta.

**Streamaajaa ei katsota.**\
Vain käytössä olevia ja livenä olevia streamaajia katsotaan, ja vain vapaalla kanavalla:
**Drops ja pisteet** -tilassa toinen kahdesta palvelee nykyistä kampanjaa;
**Vain Drops** -tilassa streamaajia ei katsota lainkaan; tauolla oleva tili ei katso
mitään. Paikat jaetaan tässä järjestyksessä, joka näkyy merkkinä jokaisella katsotulla
kanavalla: ”Putki vaarassa”, ”Putkibonus”, ”Tilattu” ja sitten ”Listan järjestys”.
Siirrä streamaajaa ylemmäs listalla tai poista käytöstä ”Katseluputket ensin” ja
”Tilatut kanavat ensin”.

## <a name="related-projects"></a>Liittyvät projektit

Saman tekijän muita projekteja:

- [**Twitch Drops API**](https://drops.sunkwi.com): ilmainen julkinen API kaikista
  käynnissä olevista Twitch Drops- ja palkintokampanjoista, päivittyy minuutin välein.
  Tämä sovellus lukee kampanjaluettelonsa siitä.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): Discord-botti, joka ilmoittaa
  palvelimellasi Twitchin livelähetyksistä ja Drops-kampanjoista.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): Discord-botti, joka ilmoittaa
  palvelimellasi Kickin livelähetyksistä ja Drops-kampanjoista.

## <a name="security"></a>Tietoturva

- Tokenit sinetöidään AES-256-GCM:llä. Avain on joko satunnainen `master.key` tietokannan
  vieressä (oletus, ei vaadi toimia) tai johdetaan sovelluksen salasanasta Argon2id:llä.
- Datakansio ja lokikansio ovat yksityisiä (`0700`): vain louhijaa ajava käyttäjä voi
  avata ne.
- HTTP-hallintapaneeli tarkistaa salasanan vakioajassa ja käyttää HttpOnly- ja
  SameSite=Strict-evästeitä (HTTPS:n takana `Secure` ja `__Host-`-etuliite). Palvelin
  säilyttää jokaisesta istunnosta vain SHA-256-tiivisteen. Se päättää istunnon 30 päivän
  jälkeen tai uloskirjautuessa. Se tarkistaa istunnon ennen kuin lukee pyynnön runkoa ja
  rajoittaa rungot 256 KiB:hen.
- Kun samasta IP-osoitteesta tulee 5 väärää salasanaa peräkkäin (kirjautumislomake tai
  bearer), osoite joutuu odottamaan 30 sekuntia. Palvelin vastaa `429` ja `Retry-After`
  katsomatta salasanaa. Lisäksi vastaus jokaiseen väärään salasanaan kestää puoli
  sekuntia.
  Kun yhdestä yhteysosoitteesta, myös jaetusta välityspalvelimesta, tulee yli 30 väärää
  salasanaa minuutissa, osoite saa yhden yrityksen 2 sekunnin välein: arvaaminen hidastuu,
  eikä ketään lukita ulos.
- Oman verkkosi ulkopuolella sijoita hallintapaneeli HTTPS-käänteisproxyn taakse ja aseta
  `MINER_PUBLIC_URL` (katso [Docker](#docker)).
- Louhija on yhteydessä vain kohteisiin `twitch.tv`, `ttvnw.net`, `jtvnw.net`,
  `twitchcdn.net`, `drops.sunkwi.com` sekä määrittämiisi ilmoituskohteisiin.
  Sovellus (työpöytä ja Docker) tarkistaa uudet versiot `github.com`-palvelusta, ja
  työpöytäsovellus lataa ne projektin GitHub-julkaisuista. Ne on allekirjoitettu: vain
  näytetty versio asennetaan.

## <a name="build"></a>Kääntäminen lähdekoodista

**Esivaatimukset:** Rust, Node.js ja järjestelmällesi
[Taurin esivaatimukset](https://v2.tauri.app/start/prerequisites/).

```bash
npm ci
npm run build          # hallintapaneeli
cargo build --release -p miner-server   # palvelin: target/release/twitch-miner
npm run bundle:linux   # työpöytäpaketit (vaatii Taurin Linux-riippuvuudet)
```

Kooditarkistukset:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (tyyppitietoinen), svelte-check ja inlangin i18n-tarkistuksen
```

Käyttöliittymän TypeScript-tyypit generoidaan Rust-tyypeistä
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) -työkalulla hakemistoon
`ui/src/lib/bindings`. Kun muutat tyyppiä, jonka käyttöliittymä näkee, suorita
`npm run bindings`.

## <a name="license"></a>Lisenssi

[MIT](../../LICENSE). Inspiraationa
[DevilXD:n TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
ja niiden forkit.
