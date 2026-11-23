<p align="center">
  <img src="../../ui/public/app-mark.svg" width="112" height="112" alt="Logo van Twitch Miner">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Mint Twitch Drops, badges en kanaalpunten met zoveel accounts als je wilt, zonder browser en zonder open Twitch-tabblad.</strong>
</p>

<p align="center">
  Een desktop-app voor Linux, Windows en macOS, of een Docker-image met hetzelfde webdashboard, om te draaien op een thuisserver of op een Raspberry Pi. Elk account gebruikt ongeveer 3 MB geheugen en bijna geen CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Nieuwste release"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="Downloads"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Platforms: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Docker-image voor amd64 en arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT-licentie"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Weergaven">
</p>

<p align="center">
  <a href="#download"><strong>Downloaden</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new">Bug melden</a>
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
  <strong>Nederlands</strong> ·
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
  <a href="README.fi.md">Suomi</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.zh-TW.md">繁體中文</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.vi.md">Tiếng&nbsp;Việt</a> ·
  <a href="README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="../media/nl/presentation.avif" alt="Video-overzicht van Twitch Miner">
</p>

## <a name="download"></a>Downloaden

Download het bestand voor je systeem uit de
[nieuwste release](https://github.com/SunkwiBOT/twitch-miner/releases/latest).

| Systeem | Pakket |
|---|---|
| **Windows** · x64 | [Installatieprogramma](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_x64-setup.exe) of [portable versie](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_x64_portable.exe) |
| **Windows** · ARM64 | [Installatieprogramma](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_arm64-setup.exe) of [portable versie](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_arm64_portable.exe) |
| **macOS** 13 of nieuwer · universal: Intel en Apple Silicon | [Schijfkopie](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_macos_universal.dmg) (`.dmg`) |
| **Linux** · x86-64 | [`.AppImage`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.AppImage), [`.deb`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.deb) (Debian, Ubuntu) of [`.rpm`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.rpm) (Fedora, openSUSE) |
| **Linux** · ARM64 | [`.AppImage`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_arm64.AppImage), [`.deb`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_arm64.deb) (Debian, Ubuntu, Raspberry Pi OS) of [`.rpm`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_arm64.rpm) (Fedora, openSUSE) |
| **Linux-server** · x86-64 of ARM64 | Alleen de server, zonder Docker ([x86-64](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner-server_linux_x64) of [ARM64](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner-server_linux_arm64)): zie [Zonder Docker](#systemd) |

Op Linux kun je ook installeren vanuit een terminal. Vervang op ARM64 `x64` door `arm64`
in de commando's.

Debian, Ubuntu, Raspberry Pi OS:

```bash
curl -LO https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.deb
sudo apt install ./twitch-miner_linux_x64.deb
```

Fedora, openSUSE:

```bash
curl -LO https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.rpm
sudo dnf install ./twitch-miner_linux_x64.rpm
# openSUSE: sudo zypper install ./twitch-miner_linux_x64.rpm
```

AppImage, op elke distributie:

```bash
curl -LO https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.AppImage
chmod +x twitch-miner_linux_x64.AppImage
./twitch-miner_linux_x64.AppImage
```

Dezelfde commando's werken een bestaande installatie bij: de bestandsnamen veranderen
niet.

De macOS-build is niet genotariseerd. Open de app de eerste keer: macOS blokkeert hem. Ga
dan naar **Systeeminstellingen › Privacy en beveiliging** en klik op **Open toch**.

Updates zijn ondertekend. De app zoekt bij het starten naar een nieuwe versie, en daarna
elk uur (hoe vaak stel je in onder **Instellingen › Over**). Hij installeert er nooit zelf
een: je installeert die met één klik, via de melding die verschijnt of via
**Instellingen › Over**.

De gegevens staan in de map met gebruikersgegevens:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="features"></a>Functies

### Accounts en streamers

- **Zoveel accounts als je wilt in één app.** Elk account mint zelfstandig, in zijn eigen modus:
  **Drops en punten** (één kanaal voor de beste campagne, het andere voor je streamers),
  **Alleen Drops** of **Alleen punten**.

  <img src="../media/nl/accounts-mining.avif" alt="Meerdere accounts die tegelijk minen, op de pagina Home">

- **Een wizard voor elk nieuw account.** Begin na het aanmelden met de standaardinstellingen
  of met die van een ander account. Kies de modus, de campagnes en de streamers, en daarna
  “Beginnen met minen” of “Opslaan en later starten”.

  <img src="../media/nl/account-setup.avif" alt="De instelwizard van een nieuw account, stap voor stap">

- **Eén wijziging met één klik op andere accounts toepassen.** Nadat je iets op een account
  hebt gewijzigd, biedt de bevestigingsmelding “Toepassen op andere accounts” aan. De
  wijziging wordt herhaald op de accounts die je aanvinkt, en die behouden de rest van hun
  instellingen.

  <img src="../media/nl/apply-to-accounts.avif" alt="Een wijziging die met “Toepassen op andere accounts” op andere accounts wordt herhaald">

- **Een streamer toevoegen.** Hij start meteen met de standaardopties: die van de app, of
  die je voor dit account hebt gekozen. Pas ze zo nodig meteen aan: bonuskisten, raids,
  momenten, kijkreeks, chat, voorspellingen.

  <img src="../media/nl/streamer-add.avif" alt="Een streamer toevoegen via de zoekfunctie van Twitch, daarna zijn opties">

- **Een streamer bewerken.** “Bewerken” opent zijn opties: bonuskisten, raids, momenten,
  kijkreeks, chat, voorspellingen… Met een schakelaar zet je hem aan of uit, zonder hem uit
  de lijst te halen.

  <img src="../media/nl/streamer-edit.avif" alt="De opties van een streamer, geopend vanuit zijn rij">

- **Meerdere streamers tegelijk bewerken.** Vink ze aan en kies “De N geselecteerde
  streamers bewerken”. Alleen de opties die je wijzigt, worden toegepast. Een optie die
  tussen hen verschilt, toont “Gemengd”, en de schakelaar wordt twee knoppen, “Ja” en
  “Nee”. Daarna stelt de app voor om er de standaardopties voor nieuwe streamers van te
  maken.

  <img src="../media/nl/streamers-group-edit.avif" alt="Meerdere streamers bewerken, met opties op “Gemengd”">

- **Streamers importeren.** Plak een lijst (tot 500 namen), of neem de lijst van een ander
  account of de kanalen die het account volgt over. De lijst wordt gecontroleerd, zodat
  alleen kanalen die echt bestaan worden toegevoegd.
  Live-updates gelden voor 46 streamers per account: de volgende in de lijst krijgen geen
  raids, momenten of voorspellingen.

  <img src="../media/nl/streamers-import.avif" alt="Streamers importeren: geplakte lijst, ander account, gevolgde kanalen">

- **Spam Points.** Wissel een aangepaste beloning van een kanaal meerdere keren achter
  elkaar in. Kies een kanaal uit je Puntenlijst, of een willekeurig kanaal dat je via de
  Twitch-zoekfunctie vindt: de beloningen verschijnen met het saldo van het account, live
  opnieuw gelezen. Stel het aantal in (tot 25, binnen het saldo), daarna “Inwisselen”. Het
  resultaat verschijnt aan het einde van de reeks, en elke reeks krijgt een eigen regel
  in het Journaal.

  <img src="../media/nl/spam-points.avif" alt="Spam Points: een kanaal gekozen, 10 × een beloning met bericht ingewisseld, de inwisselingen verschijnen in de Twitch-chat van het kanaal, daarna de regel in het Journaal">

### Drops

- **Alle lopende Drops-campagnes.** Elke campagne heeft haar eigen details: beloningen,
  koppeling van het uitgeversaccount, “Eerst minen”. De lijst komt van het project
  [Twitch Drops API](https://drops.sunkwi.com) en wordt elke 5 minuten ververst.

  <img src="../media/nl/drops-catalogue.avif" alt="De Catalogus als kalender, de zoekfunctie en de details van een campagne">

- **Jouw minevolgorde.** Sleep de campagnes in de volgorde die je wilt. Nieuwe campagnes
  vinden zelf hun plek: eerst je prioriteitscategorieën, daarna het dichtstbijzijnde einde of de
  minste resterende minuten. Sluit categorieën, woorden, een campagne of één enkele beloning uit,
  en sla de campagnes over die niet op tijd af kunnen. “Eerst minen”, in de details van een
  campagne, zet haar bovenaan.

  <img src="../media/nl/drops-order.avif" alt="De minevolgorde, de prioriteitscategorieën en de uitsluitingen">

- **Inventaris en “Alles claimen”.** De pagina Inventaris verdeelt de beloningen over
  “Te claimen”, “Lopend” en “Geclaimd”. Claim ze een voor een of allemaal tegelijk, of zet
  “Drops automatisch claimen” aan.
  Na “Uitgeversaccount koppelen” wordt het account opnieuw gelezen zodra je terugkomt in de
  app, zodat de koppeling meteen te zien is.

  <img src="../media/nl/drops-inventory.avif" alt="De inventaris en de knop “Alles claimen”">

- **“Mijn streamers eerst voor Drops en badges”.** Voor de huidige campagne gaat een streamer uit je
  lijst wiens stream voor die campagne telt, voor op andere kanalen. Hetzelfde kanaal
  brengt de drop verder en levert je zijn kanaalpunten op. Standaard aan.

  <img src="../media/nl/drops-streamers-first.avif" alt="Een streamer uit de lijst, bekeken voor de Drops en voor zijn punten">

- **Beloningsprioriteit.** Staan twee campagnes gelijk, dan kiest de miner eerst die met
  beloningen van het soort dat jij het liefst hebt (badge, emote, game-item). Hij gaat
  nooit tegen jouw mining-volgorde in.

- **Badges (bèta).** De pagina Badges toont alle globale chatbadges van Twitch en hoe je
  ze krijgt. Elk account heeft een eigen Badges-pagina: de lopende badges met hun
  voortgang, de badges die het nog kan krijgen en de badges die het al heeft. De miner
  gaat voor de badges uit Drops-campagnes en Twitch-quests, en slaat de badges over die
  het account al heeft. Een badge die je elke dag moet kijken, wordt elke dag opnieuw
  gemined tot de laatste (“20 min per dag, 3 dagen”), en een kanaal in een van de
  categorieën van de campagne telt mee. Een kist geeft “1 willekeurige badge uit N”. Een
  badge waarvoor een abonnement nodig is, wordt gemined zodra je tijdens de campagne een
  abonnement hebt genomen of cadeau hebt gedaan op een kanaal in de categorie en Twitch
  het telt. Een lopend abonnement, een Prime-abonnement of een verlenging telt niet. Vóór
  de start van de campagne wordt niets gemined. Sleep in “Wat te minen, op volgorde”
  prioriteitscategorieën, badges en andere Drops-campagnes in de volgorde die je wilt, of
  zet er een uit. Een Twitch-quest wordt alleen gemined op een kanaal waar Twitch hem in
  zijn Drops-paneel aanbiedt. Een evolutie (Pichu → Pikachu) wordt gemined zodra het
  account de badge heeft waaruit ze evolueert: de miner zet die badge vlak ervoor zelf op
  en zet de oude niet terug; “Badge-evoluties minen”, in de badge-instellingen van het
  account, zet dit uit. Een klik op een badge opent de details: de volledige voorwaarde,
  alle categorieën en kanalen, de collectie, de evolutie en de voortgang van elk account.
  Deze functie is in bèta.

  <img src="../media/nl/badges.avif" alt="De pagina Badges, de badges van een account en dan “Wat te minen, op volgorde” met Badges bovenaan gesleept">

### Bijhouden

- **Een journaal van alles, en elk verzoek.** Het Journaal houdt elke gebeurtenis bij:
  drops, verdiende en uitgegeven punten, kisten, weddenschappen, raids, streams die live
  gaan… Filter het op account, op tabblad (Drops, Badges, Punten, Overig) en op type.
  **Technisch logboek › Verzoeken** toont elk verzoek aan Twitch en aan andere diensten,
  met het resultaat en hoe lang het duurde. Logboek en verzoeken bevatten geen tokens of
  geheimen: “Kopiëren voor een bugrapport” kopieert ze, klaar om te plakken in een
  bugrapport op
  [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) of
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <img src="../media/nl/journal-requests.avif" alt="Het gefilterde Journaal, daarna Technisch logboek › Verzoeken">

- **Een Statistieken-pagina.** Verdiende punten, geclaimde drops, kijktijd en gewonnen
  voorspellingen over 7, 30 of 90 dagen, met de topstreamer of -categorie achter elk
  cijfer. Een puntencurve per streamer toont het saldo in de tijd, met markeringen voor
  kijkreeksen, raids, momenten en voorspellingen en een tooltip op elk.

  <img src="../media/nl/stats.avif" alt="De Statistieken-tegels over 30 dagen, daarna een streamer-puntencurve met een markering">

- **Pauze en dagelijkse pauze.** Eén pauzeknop, voor één account of voor alle accounts:
  pauzeer tot je hervat, of voor 30 minuten, 1 uur, 4 uur, tot morgen 09:00 of een
  aangepaste datum en tijd; tijdens de pauze staat op dezelfde knop “Hervatten”. Een badge
  toont wanneer een tijdelijke pauze eindigt, en elk account in de zijbalk toont zijn
  status: actief, gepauzeerd of een probleem om naar te kijken. Een dagelijkse pauze stopt
  het minen van alle accounts elke dag tussen twee tijdstippen, in de gekozen tijdzone,
  en hervat het zodra die pauze eindigt, met een eigen badge in de bovenste balk.
- **CSV-export en aantallen per categorie.** Het Journaal exporteert de geclaimde drops
  als CSV-bestand (datum, account, categorie, campagne, drop, minuten), gefilterd op
  account, en een dialoogvenster toont hoeveel drops in elke categorie geclaimd zijn. De
  desktop-app bewaart het bestand via een opslaan-dialoog; het webdashboard downloadt het.

### Op de desktop of op een server

- **Overal dezelfde interface.** De desktop-app (Linux, Windows, macOS) en de Docker-image
  tonen hetzelfde dashboard. Docker draait op een VPS, een NAS of een Raspberry Pi, zonder
  scherm of browser op die machine: het dashboard opent in elke browser.
  Op een telefoon past het dashboard zich aan het scherm aan, en via HTTPS installeer je het als een app.
- **Licht.** Ongeveer 3 MB geheugen per account, bijna geen CPU, en een Docker-image
  van ongeveer 12 MB.

### Beveiliging en updates

- **Je sessies, beschermd.** Geen Twitch-wachtwoord: je meldt je aan op
  `twitch.tv/activate`, vanuit je eigen browser. Sessies staan versleuteld op schijf en je
  kunt ze intrekken in je [Twitch-instellingen](https://www.twitch.tv/settings/connections).
  Op de desktop vergrendelt een optioneel app-wachtwoord de app: er wordt niets gemined tot
  het is ingevoerd. Met Docker heeft het dashboard een eigen wachtwoord, vergrendelt het
  zichzelf na een paar minuten zonder activiteit en draait het via HTTPS achter een proxy.
  Vijf verkeerde wachtwoorden op rij blokkeren het IP-adres 30 seconden lang.

  <img src="../media/nl/security-lock.avif" alt="Het slotje: de app vergrendeld, daarna ontgrendeld met het wachtwoord">

- **Ondertekende updates, nooit automatisch.** In de bovenste balk verschijnt een badge
  “Update” met het versienummer, en een desktopmelding laat het ook weten, standaard
  ingeschakeld (een schakelaar in **Instellingen › Meldingen** zet hem uit; in een browser
  klik je op “Toestaan” als erom gevraagd wordt). “Installeren en herstarten” vraagt
  eerst om bevestiging, daarna installeert de app de versie en start ze opnieuw op. Met
  een app-wachtwoord start ze vergrendeld op: voer het opnieuw in om verder te minen.
  Alleen een door het project ondertekende versie wordt geïnstalleerd. Het
  Docker-dashboard en de systemd-binary laten het ook weten, met de commando's die je
  moet uitvoeren.

  <img src="../media/nl/updates.avif" alt="De badge “Update”, de bevestiging, daarna de installatie">

### En verder

- Net als andere miners: bonuskisten, raids, momenten, communitydoelen, voorspellingen
  (strategieën, moment van inzetten, voorwaarden), kijkreeksen en geabonneerde kanalen
  eerst, aanwezigheid in de chat (uit te zetten), standaardopties voor nieuwe streamers,
  meldingen via Discord, Telegram en webhook met afbeeldingen en een knop “Testen” voor
  elk (op de server kies je hun taal onder **Instellingen › Meldingen**), een globale
  proxy of een proxy per account.
- 29 talen, donker en licht thema, bureaubladmeldingen, een pictogram in het systeemvak,
  starten met het systeem.

  <img src="../media/nl/basics.avif" alt="Talen, licht en donker thema, een bureaubladmelding en het pictogram in het systeemvak">

- **Accounts toevoegen door tokens te plakken.** Al elders ingelogd? Plak een lijst met toegangstokens, één per regel (maximaal 50); elk token wordt live gecontroleerd, met een teller “Gecontroleerd: n/N” en een nieuwe poging voor elke regel die is mislukt, waarna één instellingenstap op de hele batch wordt toegepast. De import loopt op de achtergrond, met live voortgang per account, kan halverwege worden geannuleerd, en een latere nieuwe poging stuurt alleen de mislukte accounts opnieuw. Bij het verwijderen van een op deze manier geïmporteerd account wordt ook aangeboden het token op Twitch in te trekken.

> [!WARNING]
> Dit project is niet gelieerd aan Twitch. Een account automatiseren kan in strijd zijn
> met de voorwaarden van Twitch of de regels van een campagne.

## Inhoud

- [Vergelijking](#comparison)
- [Docker](#docker)
- [Hoe het werkt](#how-it-works)
- [Veelgestelde vragen](#faq)
- [Gerelateerde projecten](#related-projects)
- [Beveiliging](#security)
- [Bouwen vanuit de broncode](#build)
- [Licentie](#license)

## <a name="comparison"></a>Vergelijking

Hoe [Twitch Miner](https://github.com/SunkwiBOT/twitch-miner)
zich verhoudt tot [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(kanaalpunten) en [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Gecontroleerd aan de hand van hun code en README's in november 2026.

| | Dit project | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops-campagnes: vindt, mint en claimt ze | ✅ | ❌ ¹ | ✅ |
| Twitch-badges: de lijst van alle badges, de badges van elk account, badge-Drops en Twitch-quests gemined in jouw volgorde (bèta) | ✅ | ❌ | deels ² |
| Kanaalpunten: kisten, raids, momenten, voorspellingen | ✅ | ✅ | ❌ |
| Spam Points: een aangepaste beloning van een kanaal meerdere keren achter elkaar ingewisseld | ✅ | ❌ | ❌ |
| Drops en kanaalpunten tegelijk op één account | ✅ | deels ¹ | ❌ |
| Zoveel accounts als je wilt in één app | ✅ | ❌ ³ | ❌ |
| Instellingen van meerdere accounts tegelijk wijzigen | ✅ | ❌ | ❌ |
| Grafische interface | ✅ desktop en web ⁴ | ❌ ⁵ | ✅ desktop |
| Webdashboard voor een thuisserver (Docker), met wachtwoord, bereikbaar via HTTPS | ✅ | ❌ ⁵ | ❌ |
| Sessies versleuteld op schijf, optioneel app-wachtwoord | ✅ | ❌ | ❌ |
| Interface in 29 talen | ✅ | ❌ | deels ⁶ |
| Ondertekende updates vanuit de app (desktop) | ✅ | ❌ | ❌ |
| Instelwizard voor elk nieuw account | ✅ | ❌ ⁵ | ❌ |
| Streamers toevoegen en bewerken in de interface, één voor één of meerdere tegelijk | ✅ | ❌ ⁵ | — |
| Streamers importeren via zoeken, een lijst, een ander account of gevolgde kanalen | ✅ | deels ⁷ | — |
| Meldingen via Discord, Telegram en webhook, met afbeeldingen | ✅ | deels ⁸ | ❌ |
| Kalender van alle Drops-campagnes, met zoekfunctie | ✅ | ❌ | deels ⁹ |
| Minevolgorde per campagne, uitgesloten woorden, genegeerde campagne of beloning | ✅ | ❌ | ❌ ¹⁰ |
| Streamers uit je lijst eerst bekeken voor de Drops van een campagne | ✅ | deels ¹¹ | ❌ |
| Filterbaar gebeurtenissenjournaal en een lijst van elk verzoek, in de interface | ✅ | ❌ ⁵ | ❌ |
| Ongeveer 3 MB geheugen per account (Docker-server) | ✅ | — ¹² | — ¹² |

¹ Het kijkt alleen naar de streamers op zijn lijst: het brengt de Drops van hun streams
verder en claimt ze, zonder een campagne te kiezen.\
² Een instelling om de Drops-campagnes waarvan de beloningen badges of emotes zijn te
minen of over te slaan; geen Twitch-quests en geen lijst van badges.\
³ Eén proces per account.\
⁴ Op je pc als desktop-app, en/of op een server (VPS, NAS, Raspberry Pi) met Docker,
zonder grafische omgeving en zonder browser op die machine: het dashboard opent in elke
browser.\
⁵ Ingesteld in een Python-bestand; de webpagina toont puntengrafieken en het logbestand.\
⁶ 21 talen.\
⁷ Het kan de kanalen volgen die het account volgt.\
⁸ Alleen tekst, zonder afbeeldingen. Het heeft ook Matrix, Pushover en Gotify, die dit
project niet heeft.\
⁹ Een lijst van campagnes met filters, zonder kalender of zoekfunctie.\
¹⁰ Het heeft prioriteitscategorieën en uitgesloten categorieën, gekozen op naam van de categorie.\
¹¹ Standaard kijkt het eerst naar de streamers op zijn lijst wier stream Drops heeft (zijn
prioriteit `DROPS`), maar nooit naar een ander kanaal.\
¹² Niet gemeten.

## <a name="docker"></a>Docker

De server toont hetzelfde dashboard in een browser. De image is
`ghcr.io/sunkwibot/twitch-miner`, voor `amd64` en `arm64`. Is er een nieuwe
versie, dan meldt het dashboard dat: `docker compose pull`, daarna `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # bij elke start de nieuwste image ophalen
    restart: unless-stopped
    ports: ["8080:8080"]   # achter een HTTPS-reverse-proxy: weghalen, of "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # verplicht, minstens 12 tekens
      # MINER_PUBLIC_URL: "https://miner.example.com"   # alleen achter een HTTPS-reverse-proxy
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Open <http://localhost:8080> (of het adres van de machine op je netwerk) en voer het
wachtwoord in.

### Variabelen

| Variabele | Standaard | Doel |
|---|---|---|
| `MINER_PASSWORD` | geen | Dashboardwachtwoord, verplicht: zonder wachtwoord weigert de server te starten. Minstens 12 tekens (bij een korter wachtwoord verschijnt bij het starten een waarschuwing). De voorbeeldwaarde `change-me` wordt geweigerd. |
| `MINER_PASSWORD_FILE` | geen | Leest `MINER_PASSWORD` uit een bestand, zoals een Docker-secret. Het een of het ander, niet allebei. |
| `MINER_PUBLIC_URL` | geen | Alleen achter een HTTPS-reverse-proxy (Caddy, nginx, Traefik, Cloudflare Tunnel…): het `https://`-adres van het dashboard, zonder pad, zoals `https://miner.example.com`. De proxy moet `X-Forwarded-Proto`, `X-Forwarded-For` en websockets doorgeven, wat gangbare proxy's standaard doen, en de poort van de container mag niet direct bereikbaar zijn. Het dashboard weigert dan alles wat niet via de proxy over HTTPS binnenkomt, en blokkeert een adres 30 seconden lang na 5 verkeerde wachtwoorden. De proxy moet op dezelfde machine of in het lokale netwerk ervan draaien: een verbinding vanaf een openbaar adres wordt geweigerd. |
| `MINER_TRUSTED_HOPS` | `1` | Alleen met `MINER_PUBLIC_URL`: hoeveel proxy's een vermelding toevoegen aan `X-Forwarded-For`, van 1 tot 10. `2` met twee proxy's achter elkaar, zoals Cloudflare vóór Traefik. |
| `MINER_PROXY_NETWORKS` | geen | Alleen met `MINER_PUBLIC_URL`: de netwerken waaruit de proxy verbindt, gescheiden door komma's: één adres of een netwerk zoals `192.168.1.0/24` of `2001:db8:1::/64`. Zodra deze is ingesteld, wordt de proxy alleen nog vanaf het loopback-adres en deze netwerken geaccepteerd, niet meer vanuit het hele privé lokale netwerk. |
| `MINER_VAULT_PASSPHRASE` | geen | Alleen voor een gegevensmap die is gekopieerd uit de desktop-app met een ingesteld app-wachtwoord: ontgrendelt die bij het starten. Een server die vanaf nul is opgezet, heeft hem nooit nodig. |
| `MINER_VAULT_PASSPHRASE_FILE` | geen | Leest `MINER_VAULT_PASSPHRASE` uit een bestand. Het een of het ander, niet allebei. |
| `DATA_DIR` | `/data` in de image, anders `data` in de werkmap | Database, sleutelbestand en logboek. |
| `LOG_DIR` | `$DATA_DIR/logs` | Logbestanden: één per dag, de laatste 14 blijven bewaard. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Luisteradres: een IP-adres en een poort van 1 tot 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Logniveau. |

De server leest en controleert de variabelen één keer, bij het starten. Een ongeldige
waarde stopt de server met een melding, in plaats van stil terug te vallen op iets anders.

Zet achter een reverse proxy `MINER_PUBLIC_URL`: zonder die variabele schrijft het eerste
verzoek dat via een proxy binnenkomt een waarschuwing in het logboek, want aanmeldpogingen
worden dan op het adres van de proxy geteld, en de foute wachtwoorden van één bezoeker
zouden iedereen buitensluiten. De live verbinding stuurt elke 30 seconden een ping, zodat
een proxy haar niet sluit als er niets gebeurt. Als Twitch of het netwerk faalt, antwoordt
de server `424`, nooit `502` of `504`: achter Cloudflare zie je de echte fout in plaats van
de pagina van Cloudflare.

### <a name="systemd"></a>Zonder Docker (systemd)

Op een Linux-server zonder Docker bevat de release de server ook als één enkel bestand,
met het dashboard erin: `twitch-miner-server_<versie>_linux_x64` of
`twitch-miner-server_<versie>_linux_arm64`. Dit is hetzelfde programma dat de
Docker-image draait, statisch gelinkt: er is verder niets te installeren, behalve de
CA-certificaten van het systeem die hij voor HTTPS leest (de Docker-image kopieert ze om
dezelfde reden). Hij gebruikt dezelfde variabelen.

```bash
# ARM64: twitch-miner-server_<versie>_linux_arm64
sudo install -m 755 twitch-miner-server_<versie>_linux_x64 /usr/local/bin/twitch-miner
sudo install -m 600 /dev/null /etc/twitch-miner.env
```

Schrijf je eigen wachtwoord, minstens 12 tekens, in `/etc/twitch-miner.env`, dat alleen
root kan lezen:

```ini
MINER_PASSWORD=change-me
```

Daarna de service, in `/etc/systemd/system/twitch-miner.service`:

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

systemd maakt `/var/lib/twitch-miner` aan voor de data en draait de server onder een
eigen gebruiker, nooit als root. Open `http://<adres van de server>:8080`; de logs
staan in `journalctl -u twitch-miner`.

Dit bestand werkt zichzelf niet bij. Zodra het dashboard een nieuwe versie toont, download
je het nieuwe bestand uit de release, vervang je het oude en start je opnieuw op:

```bash
# ARM64: twitch-miner-server_<versie>_linux_arm64
sudo install -m 755 twitch-miner-server_<versie>_linux_x64 /usr/local/bin/twitch-miner
sudo systemctl restart twitch-miner
```

## <a name="how-it-works"></a>Hoe het werkt

Elke 20 seconden kiest elk account zijn twee kanalen: het beste live kanaal voor de
bovenste campagne in zijn wachtrij, daarna de streamers van zijn puntenlijst. Een campagne
zonder live kanaal wordt overgeslagen ten gunste van de volgende die er een heeft (“Geen
kanaal live” in zijn wachtrij); live kanalen worden elke 90 seconden opnieuw gecontroleerd,
en het account keert ernaar terug zodra een van zijn streamers live gaat.

Twitch telt per account **één Drops-campagne** en **twee kanalen voor punten** tegelijk
mee. Sinds oktober 2026 kan een drop alleen worden geclaimd als het uitgeversaccount gekoppeld
is. Niet-gekoppelde campagnes maken toch voortgang, en het dashboard toont de koppellink.

Als Twitchs eigen voortgang op de bovenste campagne na 30 minuten echt kijken niet
beweegt, telt het die campagne niet mee: het account zet haar opzij, de volgende
campagne in de rij krijgt de plek, en er wordt automatisch 6 uur later opnieuw
geprobeerd — eerder als er intussen voortgang terugkomt, al kan de herprobeerde
campagne nog steeds moeten wachten achter degene die wordt gemined. Elke voortgang zet
de wachttijd terug naar nul, en dit geldt voor elk type campagne (Drops, badges,
missies). Als Twitch in plaats daarvan een andere campagne uit dezelfde categorie meetelt en die campagne wordt voltooid of opgehaald, wordt het opzijzetten meteen opgeheven in plaats van de volle 6 uur af te wachten. Het herstarten van de app of de server vergeet alle opzijgezette campagnes:
een vastzittende campagne krijgt gewoon weer 30 nieuwe minuten.

De campagnelijst komt van het project [Twitch Drops API](https://drops.sunkwi.com) (elke
5 minuten ververst), omdat Twitch die verbergt voor de tv-client.

## <a name="faq"></a>Veelgestelde vragen

### Risico's en Twitch

**Kan mijn account worden geblokkeerd?**\
Niemand kan beloven dat het niet gebeurt. Dit project is niet gelieerd aan Twitch, en een
account automatiseren kan in strijd zijn met de voorwaarden van Twitch of de regels van
een campagne. De miner kijkt naar hoogstens twee kanalen per account, de limiet van
Twitch, zonder video te downloaden. De keuze is aan jou.

**Wordt de video echt bekeken?**\
Nee, er wordt geen video gedownload. Voor Drops leest de miner elke 15 seconden de
afspeellijst van de stream in de laagste kwaliteit en vraagt hij van elk nieuw segment
alleen de header op. Voor punten stuurt hij elke minuut het Twitch-signaal “minuut
bekeken”. Elk bekeken kanaal toont hoeveel segmenten er zijn opgevraagd.

**Waarom hoogstens twee kanalen?**\
Dat is de limiet van Twitch: per account telt het maar twee tegelijk bekeken kanalen mee,
en maar één Drops-campagne tegelijk. In de modus **Drops en punten** dient het ene kanaal
de campagne en het andere je streamers. De instelling “Eén kanaal”, in
**Accountinstellingen**, valt minder op.

### Drops

**Een campagne wordt niet gemined, of een drop maakt geen voortgang.**\
Open de pagina **Drops** van het account. Onder “Minevolgorde” staat bij elke wachtende
campagne waarom: “Geen kanaal live” of “Wacht (één campagne tegelijk)”. “Twitch telt het
niet — nieuwe poging …” betekent dat 30 minuten echt kijken geen voortgang opleverden aan
de kant van Twitch: de campagne wordt opzijgezet en automatisch opnieuw geprobeerd binnen
de getoonde tijd, of eerder als er intussen voortgang terugkomt. Een campagne die niet in
die lijst staat, is buiten beschouwing gelaten: door “Wat te minen, op volgorde”, door de
instelling “Welke campagnes”, door een regel op het tabblad **Uitsluitingen**, omdat ze
niet op tijd af kan, of omdat alle beloningen die ze nog heeft een abonnement vereisen.
Een beloning waarvoor een abonnement nodig is, wacht tot je tijdens de campagne een
abonnement neemt of cadeau doet op een kanaal in de categorie (een lopend abonnement, een
Prime-abonnement of een verlenging telt niet), een badge die je elke dag moet kijken,
wacht op de volgende dag, vóór de start van de campagne wordt niets gemined, en een drop
die op een andere volgt, wacht tot de eerste klaar is. In de modus **Alleen punten** kiest
de miner geen campagne. “Eerst minen”, in de details van een campagne, zet haar bovenaan;
een genegeerde campagne komt terug met “Weer minen”, op het tabblad **Uitsluitingen**. Het
tabblad **Campagnes** toont dezelfde redenen in de groep **Geblokkeerd**.

**Moet ik het uitgeversaccount koppelen?**\
Om te claimen wel: sinds oktober 2026 laat Twitch je een drop alleen claimen als het
uitgeversaccount gekoppeld is. Standaard maakt een niet-gekoppelde campagne toch voortgang en
wacht het claimen op de koppeling. De badge “Niet gekoppeld” geeft dat aan, en
“Uitgeversaccount koppelen”, in de details van de campagne, opent de koppelpagina. Wil je
alleen gekoppelde categorieën minen, kies dan “Alleen categorieën met een gekoppeld uitgeversaccount”.

### Installatie en gegevens

**Desktop-app of Docker?**\
De desktop-app mint zolang je pc aanstaat, met een pictogram in het systeemvak, een
optioneel wachtwoord en updates met één klik. Docker mint dag en nacht op een machine die
altijd aanstaat (VPS, NAS, Raspberry Pi); je opent het in een browser, beveiligd met
`MINER_PASSWORD`. De interface is dezelfde.

**Waar staan mijn gegevens?**\
In de gegevensmap: `~/.local/share/twitch-miner` op Linux,
`%APPDATA%\twitch-miner` op Windows,
`~/Library/Application Support/twitch-miner` op macOS, en het volume `/data`
bij Docker. Op de desktop opent **Instellingen › Bureaublad** die map met één klik. Er
staan `miner.db` (accounts, instellingen, journaal), `master.key` (de sessiesleutel, als
er geen app-wachtwoord is ingesteld) en het logboek in.

**Hoe werk ik bij?**\
De desktop-app zoekt bij het starten naar een nieuwe versie, en daarna elk uur (aan te
passen in **Instellingen › Over**), en kan je waarschuwen als er een gevonden is. Klik
op de badge “Update” en daarna op “Installeren en herstarten”. Bij Docker meldt het
dashboard het: voer `docker compose pull` uit en daarna `docker compose up -d`. Zonder
Docker meldt het dashboard het op dezelfde manier: zie [Zonder Docker](#systemd) om het
bestand te vervangen en de dienst opnieuw te starten.

**Ik ben het app-wachtwoord vergeten.**\
Het kan niet worden hersteld: juist dit wachtwoord versleutelt je Twitch-sessies. Sluit de
app af, verwijder de gegevensmap en voeg je accounts opnieuw toe; hun instellingen moet je
opnieuw invoeren. Bij Docker wijzig je het dashboardwachtwoord in `MINER_PASSWORD` en voer
je daarna `docker compose up -d` uit.

### Punten en voorspellingen

**Hoeveel punten per uur?**\
Dat hangt af van Twitch en van elk kanaal. De miner verdient wat Twitch een kijker geeft:
punten voor kijktijd, bonuskisten (ongeveer één per 15 minuten) en de kijkreeksbonus. Een
abonnement vermenigvuldigt de punten, van ×1,2 tot ×2, afhankelijk van het niveau, en met
“Geabonneerde kanalen eerst”, standaard aan, komen die kanalen vóór de rest van je lijst.
De pagina Home toont de verdiende punten, en het Journaal elke opbrengst.

**Zijn weddenschappen riskant?**\
Ja, je kunt punten verliezen. Voorspellingen staan standaard uit, per streamer. Je stelt
de strategie in, het moment van inzetten, de inzet (standaard 5% van het saldo, hoogstens
50.000 punten), een minimumsaldo en, als je wilt, een voorwaarde, zoals een quotering van
minstens 1,5. Houd de inzet klein.
Een inzet onder 10 punten wordt nooit geplaatst.

**Een streamer wordt niet bekeken.**\
Alleen streamers die aanstaan en live zijn, worden bekeken, en alleen op een vrij kanaal:
in de modus **Drops en punten** dient een van de twee de huidige campagne; in de modus
**Alleen Drops** wordt geen enkele streamer bekeken; een gepauzeerd account bekijkt niets.
De plaatsen worden in deze volgorde verdeeld, getoond als badge op elk bekeken kanaal:
“Reeks in gevaar”, “Reeksbonus”, “Geabonneerd”, daarna “Lijstvolgorde”. Zet de streamer
hoger in de lijst, of zet “Kijkreeksen eerst” en “Geabonneerde kanalen eerst” uit.

## <a name="related-projects"></a>Gerelateerde projecten

Andere projecten van dezelfde auteur:

- [**Twitch Drops API**](https://drops.sunkwi.com): een gratis openbare API met alle lopende
  Twitch Drops- en beloningscampagnes, elke minuut ververst. Deze app haalt er zijn
  campagnecatalogus uit.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): een Discord-bot die livestreams en
  Drops-campagnes op Twitch aankondigt in je server.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): een Discord-bot die livestreams en
  Drops-campagnes op Kick aankondigt in je server.

## <a name="security"></a>Beveiliging

- Tokens worden verzegeld met AES-256-GCM. De sleutel is ofwel een willekeurige `master.key`
  naast de database (standaard, niets te doen), ofwel afgeleid van het app-wachtwoord met
  Argon2id.
- De gegevensmap en de logmap zijn privé (`0700`): alleen de gebruiker die de miner
  draait, kan ze openen.
- Het HTTP-dashboard controleert het wachtwoord in constante tijd en gebruikt cookies met
  HttpOnly en SameSite=Strict (`Secure`, met het voorvoegsel `__Host-`, achter HTTPS). De
  server bewaart van elke sessie alleen een SHA-256-hash. Hij beëindigt een sessie na
  30 dagen of bij het afmelden. Hij controleert de sessie voordat hij een body leest en
  beperkt bodies tot 256 KiB.
- Na 5 verkeerde wachtwoorden op rij vanaf één IP-adres (aanmeldformulier of bearer) moet
  dat adres 30 seconden wachten. De server antwoordt dan `429` met `Retry-After`, zonder
  naar het wachtwoord te kijken. Op elk verkeerd wachtwoord komt bovendien pas na een halve
  seconde antwoord.
  Na 30 foute wachtwoorden in een minuut vanaf hetzelfde verbindingsadres, een gedeelde
  proxy inbegrepen, krijgt dat adres nog maar één poging per 2 seconden: raden gaat
  trager en niemand wordt buitengesloten.
- Zet het dashboard buiten je netwerk achter een HTTPS-reverse-proxy met
  `MINER_PUBLIC_URL` (zie [Docker](#docker)).
- De miner praat alleen met `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `drops.sunkwi.com` en de meldingsdoelen die je instelt. De app (desktop en
  Docker) zoekt naar nieuwe versies op `github.com`, en de desktop-app downloadt ze uit de
  GitHub-releases van het project. Ze zijn ondertekend: alleen de getoonde versie wordt
  geïnstalleerd.

## <a name="build"></a>Bouwen vanuit de broncode

**Vereisten:** Rust, Node.js en de
[vereisten van Tauri](https://v2.tauri.app/start/prerequisites/) voor je systeem.

```bash
npm ci
npm run build          # dashboard
cargo build --release -p miner-server   # server: target/release/twitch-miner
npm run bundle:linux   # desktopbundels (de Linux-afhankelijkheden van Tauri zijn vereist)
```

Codecontroles:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (met typecontrole), svelte-check en de inlang i18n-controle
```

De TypeScript-typen van de interface worden met
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) uit de Rust-typen gegenereerd, in
`ui/src/lib/bindings`. Voer na een wijziging aan een type dat de interface ziet
`npm run bindings` uit.

## <a name="license"></a>Licentie

[MIT](../../LICENSE). Geïnspireerd door [TwitchDropsMiner van DevilXD](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
en hun forks.
