<p align="center">
  <img src="../../ui/public/app-mark.svg" width="112" height="112" alt="Logotipo de Twitch Miner">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Mina Drops, insignias y puntos de canal de Twitch en tantas cuentas como quieras, sin navegador y sin ninguna pestaña de Twitch abierta.</strong>
</p>

<p align="center">
  Una aplicación de escritorio para Linux, Windows y macOS, o una imagen de Docker con el mismo panel web, para ejecutarla en un servidor doméstico o en una Raspberry Pi. Cada cuenta usa unos 3 MB de memoria y casi nada de CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Última versión"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="Descargas"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Plataformas: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Imagen de Docker para amd64 y arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="Licencia MIT"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Visitas">
</p>

<p align="center">
  <a href="#download"><strong>Descargar</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Informar de un error</a>
</p>

<p align="center">
  <a href="../../README.md">English</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <strong>Español&nbsp;(España)</strong> ·
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
  <img src="../media/en/presentation.avif" alt="Presentación en vídeo de Twitch Miner">
</p>

## <a name="features"></a>Funciones

### Cuentas y streamers

- **Tantas cuentas como quieras en una sola app.** Cada cuenta mina de forma independiente, en su
  propio modo: **Drops y puntos** (un canal para la mejor campaña y el otro para tus
  streamers), **Solo Drops** o **Solo puntos**.

  <img src="../media/en/accounts-mining.avif" alt="Varias cuentas minando a la vez, en la página Inicio">

- **Un asistente para cada cuenta nueva.** Tras iniciar sesión, parte de los ajustes por
  defecto o de los de otra cuenta. Elige el modo, las campañas y los streamers y, al
  final, «Empezar a minar» o «Guardar y empezar más tarde».

  <img src="../media/en/account-setup.avif" alt="El asistente de configuración de una cuenta nueva, paso a paso">

- **Un cambio aplicado a otras cuentas con un clic.** Después de cambiar algo en una
  cuenta, el mensaje de confirmación ofrece «Aplicar a otras cuentas». El cambio se repite
  en las cuentas que marques, que conservan el resto de sus ajustes.

  <img src="../media/en/apply-to-accounts.avif" alt="Un cambio repetido en otras cuentas con «Aplicar a otras cuentas»">

- **Añadir un streamer.** Empieza al momento con las opciones por defecto: las de la app
  o las que hayas elegido para esta cuenta. Ajústalas en el acto si hace falta: cofres de
  bonificación, raids, momentos, racha de visualización, chat, predicciones.

  <img src="../media/en/streamer-add.avif" alt="Un streamer añadido mediante la búsqueda de Twitch y, después, sus opciones">

- **Editar un streamer.** «Editar» abre sus opciones: cofres de bonificación, raids,
  momentos, racha de visualización, chat, predicciones… Un interruptor lo activa o lo
  desactiva sin quitarlo de la lista.

  <img src="../media/en/streamer-edit.avif" alt="Las opciones de un streamer, abiertas desde su fila">

- **Editar varios streamers a la vez.** Márcalos y pulsa «Editar los N streamers
  seleccionados». Solo se aplican las opciones que cambies. Una opción que difiere entre
  ellos aparece como «Mixto», y su interruptor se convierte en dos botones, «Sí» y «No».
  Luego la app te propone convertirlas en las opciones por defecto para nuevos streamers.

  <img src="../media/en/streamers-group-edit.avif" alt="Edición de varios streamers, con opciones «Mixto»">

- **Importar streamers.** Pega una lista (hasta 500 nombres) o reutiliza la de otra cuenta
  o los canales que sigue la cuenta. La lista se comprueba para añadir solo canales que
  existen.
  Las actualizaciones en directo cubren 46 streamers por cuenta: los siguientes de la lista
  no reciben raids, momentos ni predicciones.

  <img src="../media/en/streamers-import.avif" alt="Importación de streamers: una lista pegada, otra cuenta, los canales seguidos">

- **Spam Points.** Canjea una recompensa personalizada de un canal varias veces seguidas.
  Elige un canal de tu lista de Puntos, o cualquier canal que encuentres con la búsqueda de
  Twitch: sus recompensas aparecen con el saldo de la cuenta, actualizado en tiempo real.
  Ajusta la cantidad (hasta 25, sin superar el saldo) y pulsa «Canjear». El resultado se
  muestra al terminar la ráfaga, y cada ráfaga tiene su línea en el Diario.

  <img src="../media/en/spam-points.avif" alt="Spam Points: un canal elegido, 10 × una recompensa con mensaje canjeada, los canjes llegan al chat de Twitch del canal y, después, la línea del Diario">

### Drops

- **Todas las campañas de Drops en curso.** Cada campaña tiene su ficha: recompensas,
  vinculación de la cuenta del editor, «Minar primero». La lista viene del proyecto
  [Twitch Drops API](https://drops.sunkwi.com) y se actualiza cada 5 minutos.

  <img src="../media/en/drops-catalogue.avif" alt="El Catálogo en forma de calendario, la búsqueda y la ficha de una campaña">

- **Tu orden de minado.** Arrastra las campañas al orden que quieras. Las nuevas encuentran
  su sitio solas: primero tus categorías prioritarias y después las que terminan antes o a las
  que les quedan menos minutos. Excluye categorías, palabras, una campaña o una sola
  recompensa, y omite las campañas que no pueden terminar a tiempo. «Minar primero», en la
  ficha de una campaña, la pone en cabeza.

  <img src="../media/en/drops-order.avif" alt="El orden de minado, las categorías prioritarias y las exclusiones">

- **Inventario y «Reclamar todo».** La página Inventario clasifica las recompensas en
  «Por reclamar», «En curso» y «Reclamadas». Reclámalas una a una, todas a la vez, o
  activa «Reclamar drops automáticamente».
  Después de «Vincular la cuenta del editor», la cuenta se vuelve a leer en cuanto vuelves
  a la app, así que la vinculación aparece al momento.

  <img src="../media/en/drops-inventory.avif" alt="El inventario y el botón «Reclamar todo»">

- **«Mis streamers primero para los Drops y las insignias».** En la campaña actual, un streamer de tu
  lista cuyo directo cuenta para esa campaña pasa por delante de los demás canales. El
  mismo canal hace avanzar el drop y te da sus puntos del canal. Activado por defecto.

  <img src="../media/en/drops-streamers-first.avif" alt="Un streamer de la lista, visto para los Drops y por sus puntos">

- **Prioridad de recompensas.** Cuando dos campañas tienen el mismo rango, el minero elige
  primero la que tiene recompensas del tipo que prefieres (insignia, emote, objeto del
  juego). Nunca pasa por encima de tu orden de minería.

- **Insignias (beta).** La página Insignias muestra todas las insignias globales del chat
  de Twitch y cómo conseguirlas. Cada cuenta tiene su propia página Insignias: las
  insignias en curso con su progreso, las que aún puede conseguir y las que tiene. El
  minero va a por las insignias de las campañas de Drops y las misiones de Twitch, y se
  salta las que la cuenta ya tiene. Una insignia que hay que ver cada día se vuelve a
  minar cada día hasta el último («20 min al día, 3 días»), y cuenta un canal de
  cualquiera de las categorías de la campaña. Un cofre da «1 insignia al azar entre N».
  Una insignia que requiere una suscripción se mina cuando te has suscrito o has regalado
  una suscripción durante la campaña, en un canal de la categoría, y Twitch la cuenta. Una
  suscripción ya en curso, una suscripción con Prime o una renovación no cuentan. No se
  mina nada antes de que empiece su campaña. En «Qué minar, en orden», arrastra las
  categorías prioritarias, las insignias y las demás campañas de Drops al orden que
  quieras, o desactiva alguno. Una misión de Twitch solo se mina en un canal donde Twitch
  la ofrece en su panel de Drops. Una evolución (Pichu → Pikachu) se mina en cuanto la
  cuenta tiene la insignia de la que evoluciona: el minero se pone él mismo esa insignia
  justo antes y no vuelve a poner la anterior; «Minar las evoluciones de insignias», en
  los ajustes de insignias de la cuenta, lo desactiva. Un clic en una insignia abre sus
  detalles: el requisito completo, todas las categorías y los canales, la colección, la
  evolución y el progreso de cada cuenta. Esta función está en beta.

  <img src="../media/en/badges.avif" alt="La página Insignias, las insignias de una cuenta y luego «Qué minar, en orden» con Insignias arrastradas al primer puesto">

### Seguimiento

- **Un diario de todo, y cada solicitud.** El Diario guarda cada evento: drops, puntos
  ganados y gastados, cofres, apuestas, raids, directos que empiezan… Fíltralo por cuenta,
  por pestaña (Drops, Insignias, Puntos, Otros) y por tipo. **Registros técnicos › Solicitudes**
  muestra cada solicitud enviada a Twitch y a otros servicios, con su resultado y su
  duración. Los registros y las solicitudes no contienen ningún token ni secreto:
  «Copiar para un informe de error» los copia, listos para pegarlos en un informe de error
  en [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) o
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <img src="../media/en/journal-requests.avif" alt="El Diario filtrado y, después, Registros técnicos › Solicitudes">

- **Una página de Estadísticas.** Puntos ganados, drops reclamados, tiempo visto y
  predicciones ganadas en 7, 30 o 90 días, con el streamer o la categoría principal
  detrás de cada cifra. Una curva de puntos por streamer muestra el saldo a lo largo del
  tiempo, con marcadores para rachas, raids, momentos y predicciones, y un tooltip en
  cada uno.

  <img src="../media/en/stats.avif" alt="Los mosaicos de Estadísticas en 30 días, y luego la curva de puntos de un streamer con un marcador">

- **Pausa y una pausa diaria.** Un botón de pausa, para una cuenta o para todas: pausa
  hasta que reanudes, o durante 30 minutos, 1 hora, 4 horas, hasta mañana a las 09:00, o
  hasta una fecha y hora personalizadas; mientras está en pausa, el mismo botón dice
  «Reanudar». Una insignia muestra la hora a la que termina una pausa temporizada, y cada
  cuenta en la barra lateral muestra su estado: en marcha, en pausa o con un problema que
  revisar. Una pausa diaria detiene la minería de todas las cuentas cada día entre dos
  horas, en la zona horaria elegida, y la reanuda al terminar, con su propia insignia en
  la barra superior.
- **Exportación CSV y recuentos por categoría.** El Diario exporta los drops reclamados
  como un archivo CSV (fecha, cuenta, categoría, campaña, drop, minutos), filtrados por
  cuenta, y un diálogo lista cuántos drops se reclamaron en cada categoría. La aplicación
  de escritorio guarda el archivo con un diálogo de guardado; el panel web lo descarga.

### En el escritorio o en un servidor

- **La misma interfaz en todas partes.** La app de escritorio (Linux, Windows, macOS) y la
  imagen de Docker muestran el mismo panel. Docker funciona en un VPS, un NAS o una
  Raspberry Pi, sin pantalla ni navegador en esa máquina: el panel se abre en cualquier
  navegador.
  En el móvil, el panel se adapta a la pantalla y, por HTTPS, se instala como una app.
- **Ligero.** Unos 3 MB de memoria por cuenta, casi nada de CPU y una imagen de
  Docker de unos 12 MB.

### Seguridad y actualizaciones

- **Tus sesiones, protegidas.** Sin contraseña de Twitch: inicias sesión en
  `twitch.tv/activate`, desde tu navegador. Las sesiones se guardan cifradas en el disco
  y puedes revocarlas en tus [ajustes de Twitch](https://www.twitch.tv/settings/connections).
  En el escritorio, una contraseña de la app opcional la bloquea: no se mina nada hasta
  que se escribe. Con Docker, el panel tiene su propia contraseña, se bloquea solo tras
  unos minutos sin actividad y funciona por HTTPS detrás de un proxy. Cinco contraseñas
  incorrectas seguidas bloquean la dirección IP durante 30 segundos.

  <img src="../media/en/security-lock.avif" alt="El candado: la app bloqueada y, después, desbloqueada con su contraseña">

- **Actualizaciones firmadas, nunca automáticas.** En la barra superior aparece una
  insignia «Actualización» con el número de versión, y una notificación de escritorio
  también te avisa, activada de forma predeterminada (un interruptor en
  **Ajustes › Notificaciones** la desactiva; en el navegador, haz clic en «Permitir»
  cuando te lo pida). «Instalar y reiniciar» primero te pide confirmación; luego la app
  instala la versión y se reinicia. Si tiene contraseña de la app, se reinicia bloqueada:
  vuelve a escribirla para seguir minando. Solo se instala una versión firmada por el
  proyecto. El panel de Docker y el binario de systemd también te avisan, con los
  comandos que hay que ejecutar.

  <img src="../media/en/updates.avif" alt="La insignia «Actualización», la confirmación y, después, la instalación">

### Y además

- Como otros mineros: cofres de bonificación, raids, momentos, metas de la comunidad,
  predicciones (estrategias, momento de la apuesta, condiciones), rachas de visualización
  y canales suscritos primero, presencia en el chat (desactivable), opciones por defecto
  para nuevos streamers, notificaciones de Discord, Telegram y webhook con imágenes y un
  botón «Probar» para cada una (en el servidor, su idioma se elige en **Ajustes ›
  Notificaciones**), proxy global o por cuenta.
- 29 idiomas, temas oscuro y claro, notificaciones de escritorio, icono en la bandeja,
  inicio con el sistema.

  <img src="../media/en/basics.avif" alt="Idiomas, temas claro y oscuro, una notificación de escritorio y el icono de la bandeja">

- **Añadir cuentas pegando una lista de tokens.** ¿Ya has iniciado sesión en otro sitio? Pega una lista de tokens de acceso, uno por línea (hasta 50); cada uno se comprueba en directo, con un contador «Comprobados: n/N» y un reintento para cualquier línea que haya fallado, y luego un solo paso de ajustes se aplica a todo el lote. La importación en sí se ejecuta en segundo plano con el progreso en directo por cuenta, se puede cancelar a mitad y un reintento solo reenvía las cuentas que fallaron. Al eliminar una cuenta importada así, se ofrece revocar también su token en Twitch.

> [!WARNING]
> Este proyecto no está afiliado a Twitch. Automatizar una cuenta puede incumplir las
> condiciones de Twitch o las reglas de una campaña.

## Contenido

- [Comparación](#comparison)
- [Descarga](#download)
- [Docker](#docker)
- [Cómo funciona](#how-it-works)
- [Preguntas frecuentes](#faq)
- [Proyectos relacionados](#related-projects)
- [Seguridad](#security)
- [Compilar desde el código fuente](#build)
- [Licencia](#license)

## <a name="comparison"></a>Comparación

Así queda [Twitch Miner](https://github.com/SunkwiBOT/twitch-miner)
frente a [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(puntos del canal) y [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Comprobado en su código y sus README en noviembre de 2026.

| | Este proyecto | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Campañas de Drops: las encuentra, las mina y las reclama | ✅ | ❌ ¹ | ✅ |
| Insignias de Twitch: la lista de todas las insignias, las de cada cuenta, Drops de insignias y misiones de Twitch minados en tu orden (beta) | ✅ | ❌ | en parte ² |
| Puntos del canal: cofres, raids, momentos, predicciones | ✅ | ✅ | ❌ |
| Spam Points: una recompensa personalizada de un canal canjeada varias veces seguidas | ✅ | ❌ | ❌ |
| Drops y puntos del canal a la vez en una misma cuenta | ✅ | en parte ¹ | ❌ |
| Tantas cuentas como quieras en una sola app | ✅ | ❌ ³ | ❌ |
| Cambiar los ajustes de varias cuentas a la vez | ✅ | ❌ | ❌ |
| Interfaz gráfica | ✅ escritorio y web ⁴ | ❌ ⁵ | ✅ escritorio |
| Panel web para un servidor doméstico (Docker), con contraseña y accesible por HTTPS | ✅ | ❌ ⁵ | ❌ |
| Sesiones cifradas en el disco, contraseña de la app opcional | ✅ | ❌ | ❌ |
| Interfaz en 29 idiomas | ✅ | ❌ | en parte ⁶ |
| Actualizaciones firmadas desde la propia app (escritorio) | ✅ | ❌ | ❌ |
| Asistente de configuración para cada cuenta nueva | ✅ | ❌ ⁵ | ❌ |
| Streamers añadidos y editados en la interfaz, de uno en uno o varios a la vez | ✅ | ❌ ⁵ | — |
| Streamers importados por búsqueda, una lista, otra cuenta o los canales seguidos | ✅ | en parte ⁷ | — |
| Notificaciones de Discord, Telegram y webhook con imágenes | ✅ | en parte ⁸ | ❌ |
| Calendario de todas las campañas de Drops, con búsqueda | ✅ | ❌ | en parte ⁹ |
| Orden de minado por campaña, palabras excluidas, campaña o recompensa ignorada | ✅ | ❌ | ❌ ¹⁰ |
| Los streamers de tu lista, vistos primero para los Drops de una campaña | ✅ | en parte ¹¹ | ❌ |
| Diario de eventos filtrable y lista de cada solicitud, en la interfaz | ✅ | ❌ ⁵ | ❌ |
| Unos 3 MB de memoria por cuenta (servidor Docker) | ✅ | — ¹² | — ¹² |

¹ Solo ve los streamers de su lista: hace avanzar los Drops de sus directos y los reclama,
sin elegir campaña.\
² Una opción para minar u omitir las campañas de Drops cuyas recompensas son insignias o
emotes; sin misiones de Twitch ni lista de insignias.\
³ Un proceso por cuenta.\
⁴ En tu PC como app de escritorio, y/o en un servidor (VPS, NAS, Raspberry Pi) con Docker,
sin entorno gráfico ni navegador en esa máquina: el panel se abre en cualquier navegador.\
⁵ Se configura en un archivo de Python; su página web muestra gráficos de puntos y el
archivo de registro.\
⁶ 21 idiomas.\
⁷ Puede seguir los canales que sigue la cuenta.\
⁸ Solo texto, sin imágenes. También tiene Matrix, Pushover y Gotify, que este proyecto no
tiene.\
⁹ Una lista de campañas con filtros, sin calendario ni búsqueda.\
¹⁰ Tiene categorías prioritarias y categorías excluidas, elegidas por el nombre de la categoría.\
¹¹ Por defecto, primero ve los streamers de su lista cuyo directo tiene Drops (su
prioridad `DROPS`), pero nunca otro canal.\
¹² No medido.

## <a name="download"></a>Descarga

Descarga el archivo para tu sistema desde la
[última versión](https://github.com/SunkwiBOT/twitch-miner/releases/latest).

| Sistema | Paquete |
|---|---|
| **Windows** · x64 | Instalador (`x64-setup.exe`) o [ejecutable portátil](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_x64_portable.exe) |
| **Windows** · ARM64 | Instalador (`arm64-setup.exe`) o [ejecutable portátil](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_arm64_portable.exe) |
| **macOS** 13 o posterior · universal: Intel y Apple Silicon | Imagen de disco (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) o `.rpm` (Fedora, openSUSE) |
| **Linux** · ARM64 | `.AppImage`, `.deb` (Debian, Ubuntu, Raspberry Pi OS) o `.rpm` (Fedora, openSUSE) |
| **Servidor Linux** · x86-64 o ARM64 | Solo el servidor, sin Docker (`twitch-miner-server_…_linux_x64` o `_linux_arm64`): consulta [Sin Docker](#systemd) |

La versión para macOS no está notarizada. La primera vez, abre la app: macOS la bloquea.
Luego ve a **Ajustes del Sistema › Privacidad y seguridad** y haz clic en
**Abrir igualmente**.

Las actualizaciones están firmadas. La app busca una versión nueva al arrancar y luego cada
hora (puedes cambiar la frecuencia en **Ajustes › Acerca de**). Nunca instala una por su
cuenta: la instalas tú con un clic, desde el mensaje que aparece o desde
**Ajustes › Acerca de**.

Los datos se guardan en la carpeta de datos del usuario:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

El servidor muestra el mismo panel en un navegador. La imagen es
`ghcr.io/sunkwibot/twitch-miner`, para `amd64` y `arm64`. Cuando sale una
versión nueva, el panel te lo indica: `docker compose pull` y luego
`docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # descarga la imagen más reciente en cada arranque
    restart: unless-stopped
    ports: ["8080:8080"]   # detrás de un proxy inverso HTTPS: quítalo, o "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # obligatoria, 12 caracteres o más
      # MINER_PUBLIC_URL: "https://miner.example.com"   # solo detrás de un proxy inverso HTTPS
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Abre <http://localhost:8080> (o la dirección de la máquina en tu red) e introduce la
contraseña.

### Variables

| Variable | Por defecto | Función |
|---|---|---|
| `MINER_PASSWORD` | ninguno | Contraseña del panel, obligatoria: sin ella, el servidor se niega a arrancar. De 12 caracteres o más (si es más corta, aparece una advertencia al arrancar). El valor de ejemplo `change-me` se rechaza. |
| `MINER_PASSWORD_FILE` | ninguno | Lee `MINER_PASSWORD` desde un archivo, por ejemplo un secreto de Docker. Una u otra, no las dos. |
| `MINER_PUBLIC_URL` | ninguno | Solo detrás de un proxy inverso HTTPS (Caddy, nginx, Traefik, Cloudflare Tunnel…): la dirección `https://` del panel, sin ruta, como `https://miner.example.com`. El proxy debe reenviar `X-Forwarded-Proto`, `X-Forwarded-For` y los websockets, algo que los proxies habituales hacen por defecto, y el puerto del contenedor no debe ser accesible directamente. Así, el panel rechaza todo lo que no llegue por el proxy en HTTPS y bloquea una dirección durante 30 segundos tras 5 contraseñas incorrectas. El proxy debe ejecutarse en la misma máquina o en su red local: una conexión desde una dirección pública se rechaza. |
| `MINER_TRUSTED_HOPS` | `1` | Solo con `MINER_PUBLIC_URL`: cuántos proxies añaden una entrada a `X-Forwarded-For`, de 1 a 10. `2` con dos proxies, uno detrás de otro, como Cloudflare delante de Traefik. |
| `MINER_PROXY_NETWORKS` | ninguno | Solo con `MINER_PUBLIC_URL`: las redes desde las que se conecta el proxy, separadas por comas: una sola dirección o una red como `192.168.1.0/24` o `2001:db8:1::/64`. Una vez definida, el proxy solo se acepta desde la dirección de loopback y estas redes, ya no desde toda la red local privada. |
| `MINER_VAULT_PASSPHRASE` | ninguno | Solo para una carpeta de datos copiada de la app de escritorio con una contraseña de la app: la desbloquea al arrancar. Un servidor creado desde cero nunca la necesita. |
| `MINER_VAULT_PASSPHRASE_FILE` | ninguno | Lee `MINER_VAULT_PASSPHRASE` desde un archivo. Una u otra, no las dos. |
| `DATA_DIR` | `/data` en la imagen; si no, `data` en el directorio de trabajo | Base de datos, archivo de clave y registros. |
| `LOG_DIR` | `$DATA_DIR/logs` | Archivos de registro: uno al día, y se conservan los 14 últimos. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Dirección de escucha: una dirección IP y un puerto del 1 al 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Nivel de registro. |

El servidor lee y comprueba las variables una sola vez, al arrancar. Un valor no válido
detiene el servidor con un mensaje, en lugar de recurrir en silencio a un valor
alternativo.

Detrás de un proxy inverso, define `MINER_PUBLIC_URL`: sin ella, la primera solicitud que
llega a través de un proxy escribe un aviso en los registros, porque los intentos de
inicio de sesión se cuentan entonces sobre la dirección del proxy, y las contraseñas
erróneas de un visitante bloquearían a todo el mundo. La conexión en directo envía un ping
cada 30 segundos, para que un proxy no la cierre cuando no pasa nada. Cuando Twitch o la
red fallan, el servidor responde `424`, nunca `502` ni `504`: detrás de Cloudflare, ves el
error real en lugar de la página de Cloudflare.

### <a name="systemd"></a>Sin Docker (systemd)

En un servidor Linux sin Docker, la versión también incluye el servidor como un único
archivo, con el panel dentro: `twitch-miner-server_<versión>_linux_x64` o
`twitch-miner-server_<versión>_linux_arm64`. Es el mismo programa que ejecuta la imagen
de Docker, enlazado estáticamente: no hay nada más que instalar, salvo los certificados
CA del sistema que lee para HTTPS (la imagen de Docker los copia por el mismo motivo).
Toma las mismas variables.

```bash
# ARM64: twitch-miner-server_<versión>_linux_arm64
sudo install -m 755 twitch-miner-server_<versión>_linux_x64 /usr/local/bin/twitch-miner
sudo install -m 600 /dev/null /etc/twitch-miner.env
```

Escribe tu propia contraseña, de 12 caracteres o más, en `/etc/twitch-miner.env`, que
solo root puede leer:

```ini
MINER_PASSWORD=change-me
```

Luego el servicio, en `/etc/systemd/system/twitch-miner.service`:

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

systemd crea `/var/lib/twitch-miner` para los datos y ejecuta el servidor con su propio
usuario, nunca como root. Abre `http://<dirección del servidor>:8080`; los registros
están en `journalctl -u twitch-miner`.

Este archivo no se actualiza solo. Cuando el panel muestre una versión nueva, descarga el
archivo nuevo de la versión, sustituye el anterior y reinicia:

```bash
# ARM64: twitch-miner-server_<versión>_linux_arm64
sudo install -m 755 twitch-miner-server_<versión>_linux_x64 /usr/local/bin/twitch-miner
sudo systemctl restart twitch-miner
```

## <a name="how-it-works"></a>Cómo funciona

Cada 20 segundos, cada cuenta elige sus dos canales: el mejor canal en directo para la
primera campaña de su cola y, después, los streamers de su lista de puntos. Una campaña
sin canal en directo se omite en favor de la siguiente que tenga uno («Ningún canal en
directo» en su cola); los canales en directo se vuelven a comprobar cada 90 segundos, y la
cuenta vuelve a ella en cuanto uno de sus streamers se pone en directo.

Twitch acredita **una campaña de Drops** y **dos canales para puntos** a la vez por
cuenta. Desde octubre de 2026, un drop solo se puede reclamar si la cuenta del editor está
vinculada. Las campañas sin vincular siguen avanzando, y el panel muestra el enlace para
vincularla.

Si Twitch no hace avanzar la campaña principal tras 30 minutos de visionado real, no la está
acreditando: la cuenta la deja de lado, la siguiente campaña de la cola ocupa su lugar, y se
reintenta automáticamente 6 horas después — antes si la progresión reaparece primero, aunque
la campaña reintentada puede seguir esperando detrás de la que se está minando. Cualquier
progreso reinicia la espera, y esto se aplica a todo tipo de campaña (Drops, insignias,
misiones). Si Twitch acredita otra campaña de la misma categoría en su lugar y esa
campaña termina o se reclama, el aplazamiento se levanta de inmediato en vez de esperar
las 6 horas completas. Reiniciar la aplicación o el servidor olvida todas las campañas dejadas de lado:
una campaña bloqueada simplemente empieza de nuevo con 30 minutos.

La lista de campañas viene del proyecto [Twitch Drops API](https://drops.sunkwi.com)
(se actualiza cada 5 minutos), porque Twitch la oculta al cliente de TV.

## <a name="faq"></a>Preguntas frecuentes

### Riesgos y Twitch

**¿Me pueden banear la cuenta?**\
Nadie puede garantizar que no. Este proyecto no está afiliado a Twitch, y automatizar una
cuenta puede incumplir sus condiciones o las reglas de una campaña. El minero ve como
mucho dos canales por cuenta, el límite de Twitch, sin descargar ningún vídeo. Tú decides.

**¿De verdad se ve el vídeo?**\
No, no se descarga ningún vídeo. Para los Drops, el minero lee cada 15 segundos la lista de
reproducción del directo en la calidad más baja y solo pide la cabecera de cada segmento
nuevo. Para los puntos, envía cada minuto la señal de «minuto visto» de Twitch. Cada canal
visto muestra cuántos segmentos se han pedido.

**¿Por qué dos canales como máximo?**\
Es el límite de Twitch: solo acredita dos canales vistos a la vez por cuenta, y una sola
campaña de Drops a la vez. En el modo **Drops y puntos**, un canal sirve a la campaña y el
otro a tus streamers. El ajuste «Un canal», en **Ajustes de la cuenta**, es más discreto.

### Drops

**Una campaña no se mina, o un drop no avanza.**\
Abre la página **Drops** de la cuenta. En «Orden de minado», cada campaña en espera indica
por qué: «Ningún canal en directo» o «En espera (una campaña a la vez)». «Twitch no lo
cuenta — próximo intento …» significa que 30 minutos de visionado real no han traído
progreso por parte de Twitch: la campaña se deja de lado y se reintenta automáticamente en
el tiempo indicado, o antes si la progresión reaparece primero. Una campaña que no aparece
en esa lista se ha descartado: por «Qué minar, en orden», por el ajuste «Qué campañas»,
por una regla de la pestaña **Exclusiones**, porque no puede terminar a tiempo o porque
todas las recompensas que le quedan requieren una suscripción. Una recompensa que requiere
una suscripción espera a que te suscribas o regales una suscripción durante la campaña, en
un canal de la categoría (una suscripción ya en curso, una suscripción con Prime o una
renovación no cuentan), una insignia que hay que ver cada día espera al día siguiente, no
se mina nada antes de que empiece su campaña, y un drop que va después de otro espera a
que el primero termine. En el modo **Solo puntos**, el minero no elige ninguna campaña.
«Minar primero», en la ficha de una campaña, la pone en cabeza; una campaña ignorada puede
volver con «Volver a minar», en la pestaña **Exclusiones**. La pestaña **Campañas**
muestra los mismos motivos en el grupo **Bloqueadas**.

**¿Tengo que vincular la cuenta del editor?**\
Para reclamar, sí: desde octubre de 2026, Twitch solo deja reclamar un drop si la cuenta
del editor está vinculada. Por defecto, una campaña sin vincular sigue avanzando y la
reclamación espera a la vinculación. La insignia «No vinculada» lo indica, y «Vincular la
cuenta del editor», en la ficha de la campaña, abre la página de vinculación. Para minar
solo las categorías vinculadas, elige «Solo categorías con la cuenta del editor vinculada».

### Instalación y datos

**¿App de escritorio o Docker?**\
La app de escritorio mina mientras tu PC está encendido, con su icono en la bandeja, una
contraseña opcional y actualizaciones con un clic. Docker mina día y noche en una máquina
siempre encendida (VPS, NAS, Raspberry Pi); lo abres en un navegador, protegido por
`MINER_PASSWORD`. La interfaz es la misma.

**¿Dónde están mis datos?**\
En la carpeta de datos: `~/.local/share/twitch-miner` en Linux,
`%APPDATA%\twitch-miner` en Windows,
`~/Library/Application Support/twitch-miner` en macOS, y el volumen `/data`
con Docker. En el escritorio, **Ajustes › Escritorio** la abre con un clic. Contiene
`miner.db` (cuentas, ajustes, diario), `master.key` (la clave de las sesiones, cuando no
hay contraseña de la app) y los registros.

**¿Cómo actualizo?**\
La app de escritorio busca una versión nueva al arrancar y luego cada hora (puedes
cambiarlo en **Ajustes › Acerca de**), y puede avisarte cuando encuentre una. Haz clic en
la insignia «Actualización» y después en «Instalar y reiniciar». Con Docker, el panel te
avisa: ejecuta `docker compose pull` y luego `docker compose up -d`. Sin Docker, el panel
te avisa igual: consulta [Sin Docker](#systemd) para reemplazar el archivo y reiniciar el
servicio.

**He olvidado la contraseña de la app.**\
No se puede recuperar: es lo que cifra tus sesiones de Twitch. Cierra la app, borra la
carpeta de datos y vuelve a añadir tus cuentas; tendrás que configurar de nuevo sus
ajustes. Con Docker, la contraseña del panel se cambia en `MINER_PASSWORD` y luego
`docker compose up -d`.

### Puntos y predicciones

**¿Cuántos puntos por hora?**\
Depende de Twitch y de cada canal. El minero gana lo que Twitch da a un espectador: los
puntos por tiempo de visualización, los cofres de bonificación (más o menos uno cada 15
minutos) y el bono de racha de visualización. Una suscripción multiplica los puntos, de
×1,2 a ×2 según el nivel, y con «Canales suscritos primero», activado por defecto, esos
canales van antes que el resto de tu lista. La página Inicio muestra los puntos ganados, y
el Diario, cada ganancia.

**¿Son arriesgadas las apuestas?**\
Sí, se pueden perder puntos. Las predicciones están desactivadas por defecto, streamer por
streamer. Tú eliges la estrategia, cuándo apostar, la apuesta (5 % del saldo por defecto,
50 000 puntos como máximo), un saldo mínimo y, si quieres, una condición, como una cuota
de al menos 1,5. Mantén una apuesta pequeña.
Una apuesta de menos de 10 puntos nunca se hace.

**No se está viendo un streamer.**\
Solo se ven los streamers activados y en directo, y solo en un canal libre: en el modo
**Drops y puntos**, uno de los dos sirve a la campaña actual; en el modo **Solo Drops**, no
se ve ningún streamer; una cuenta en pausa no ve nada. Los huecos se asignan en este
orden, que aparece como insignia en cada canal visto: «Racha en riesgo», «Bono de racha»,
«Suscrito» y, por último, «Orden de la lista». Sube el streamer en la lista, o desactiva
«Rachas de visualización primero» y «Canales suscritos primero».

## <a name="related-projects"></a>Proyectos relacionados

Otros proyectos del mismo autor:

- [**Twitch Drops API**](https://drops.sunkwi.com): una API pública y gratuita con todas
  las campañas de Drops y de recompensas de Twitch en curso, actualizada cada minuto. Esta
  app saca de ella su catálogo de campañas.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): un bot de Discord que anuncia en tu
  servidor los directos de Twitch y las campañas de Drops.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): un bot de Discord que anuncia en tu
  servidor los directos de Kick y las campañas de Drops.

## <a name="security"></a>Seguridad

- Los tokens se sellan con AES-256-GCM. La clave es o bien un archivo `master.key`
  aleatorio junto a la base de datos (por defecto, sin hacer nada), o bien una clave
  derivada de la contraseña de la app con Argon2id.
- La carpeta de datos y la de registros son privadas (`0700`): solo el usuario que ejecuta
  el minero puede abrirlas.
- El panel HTTP comprueba la contraseña en tiempo constante y usa cookies HttpOnly y
  SameSite=Strict (`Secure`, con el prefijo `__Host-`, detrás de HTTPS). El servidor
  solo guarda un SHA-256 de cada sesión. Termina una sesión a los 30 días o al cerrar
  sesión. Comprueba la sesión antes de leer cualquier cuerpo de petición y limita los
  cuerpos a 256 KiB.
- Tras 5 contraseñas incorrectas seguidas desde una misma dirección IP (formulario de
  inicio de sesión o bearer), esa dirección espera 30 segundos. El servidor responde
  `429` con `Retry-After` sin mirar la contraseña. Además, cada contraseña incorrecta
  tarda medio segundo en recibir respuesta.
  Pasadas 30 contraseñas incorrectas en un minuto desde una misma dirección de conexión,
  un proxy compartido incluido, esa dirección tiene un solo intento cada 2 segundos:
  adivinar se vuelve lento y nadie se queda bloqueado.
- Fuera de tu red, pon el panel detrás de un proxy inverso HTTPS con
  `MINER_PUBLIC_URL` (consulta [Docker](#docker)).
- El minero solo se comunica con `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `drops.sunkwi.com` y los destinos de notificación que configures. La app
  (escritorio y Docker) busca versiones nuevas en `github.com`, y la app de escritorio las
  descarga de las releases de GitHub del proyecto. Están firmadas: solo se instala la
  versión que se muestra.

## <a name="build"></a>Compilar desde el código fuente

**Requisitos previos:** Rust, Node.js y los
[requisitos previos de Tauri](https://v2.tauri.app/start/prerequisites/) para tu sistema.

```bash
npm ci
npm run build          # panel
cargo build --release -p miner-server   # servidor: target/release/twitch-miner
npm run bundle:linux   # paquetes de escritorio (requiere las dependencias de Tauri para Linux)
```

Comprobaciones del código:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (con tipos), svelte-check y la comprobación de i18n de inlang
```

Los tipos TypeScript de la interfaz se generan a partir de los tipos de Rust con
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) en `ui/src/lib/bindings`. Tras cambiar un
tipo que ve la interfaz, ejecuta `npm run bindings`.

## <a name="license"></a>Licencia

[MIT](../../LICENSE). Inspirado en
[TwitchDropsMiner de DevilXD](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
y sus forks.
