<p align="center">
  <img src="../../ui/public/app-mark.svg" width="112" height="112" alt="Twitch Miner 標誌">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>挖取 Twitch Drops、徽章和頻道點數，想用多少帳號都可以，不需要瀏覽器，也不必開著 Twitch 分頁。</strong>
</p>

<p align="center">
  支援 Linux、Windows 和 macOS 的桌面應用程式；也有內含相同網頁儀表板的 Docker 映像檔，可在家用伺服器或 Raspberry Pi 上執行。每個帳號約使用 3 MB 記憶體，幾乎不佔用 CPU。
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="最新版本"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="下載次數"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="平台：Linux、Windows、macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="支援 amd64 和 arm64 的 Docker 映像檔"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT 授權"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="瀏覽次數">
</p>

<p align="center">
  <a href="#download"><strong>下載</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://dropsalerts.com">Drops Alerts</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new">回報錯誤</a>
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
  <a href="README.fi.md">Suomi</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <strong>繁體中文</strong> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.vi.md">Tiếng&nbsp;Việt</a> ·
  <a href="README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="../media/zh-TW/presentation.avif" alt="Twitch Miner 影片介紹">
</p>

## <a name="download"></a>下載

請從[最新版本](https://github.com/SunkwiBOT/twitch-miner/releases/latest)下載適合你系統的檔案。

| 系統 | 套件 |
|---|---|
| **Windows** · x64 | [安裝程式](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_x64-setup.exe)或[免安裝執行檔](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_x64_portable.exe) |
| **Windows** · ARM64 | [安裝程式](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_arm64-setup.exe)或[免安裝執行檔](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_arm64_portable.exe) |
| **macOS** 13 或以上版本 · 通用版：Intel 和 Apple Silicon | [磁碟映像檔](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_macos_universal.dmg)（`.dmg`） |
| **Linux** · x86-64 | [`.AppImage`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.AppImage)、[`.deb`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.deb)（Debian、Ubuntu）或 [`.rpm`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.rpm)（Fedora、openSUSE） |
| **Linux** · ARM64 | [`.AppImage`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_arm64.AppImage)、[`.deb`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_arm64.deb)（Debian、Ubuntu、Raspberry Pi OS）或 [`.rpm`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_arm64.rpm)（Fedora、openSUSE） |
| **Linux 伺服器** · x86-64 或 ARM64 | 只有伺服器本體，不需要 Docker（[x86-64](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner-server_linux_x64) 或 [ARM64](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner-server_linux_arm64)）：參見[不使用 Docker](#systemd) |

在 Linux 上，也可以從終端機安裝。ARM64 請把指令中的 `x64` 換成 `arm64`。

Debian、Ubuntu、Raspberry Pi OS：

```bash
curl -LO https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.deb
sudo apt install ./twitch-miner_linux_x64.deb
```

Fedora、openSUSE：

```bash
curl -LO https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.rpm
sudo dnf install ./twitch-miner_linux_x64.rpm
# openSUSE: sudo zypper install ./twitch-miner_linux_x64.rpm
```

AppImage，適用於任何發行版：

```bash
curl -LO https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.AppImage
chmod +x twitch-miner_linux_x64.AppImage
./twitch-miner_linux_x64.AppImage
```

檔名不會改變，所以同樣的指令也能更新已安裝的版本。

macOS 版本未經 Apple 公證。第一次執行時，先開啟應用程式：macOS 會阻擋它。接著前往**系統設定 › 隱私權與安全性**，按一下**強制打開**。

更新都經過簽章。應用程式啟動時會檢查是否有新版本，之後每小時檢查一次（可在**設定 › 關於**中變更檢查頻率）。它絕不會自行安裝更新：你可以從跳出的訊息或從**設定 › 關於**一鍵安裝。

資料存放在使用者資料夾中：

- Linux：`~/.local/share/twitch-miner`
- Windows：`%APPDATA%\twitch-miner`
- macOS：`~/Library/Application Support/twitch-miner`

## <a name="features"></a>功能

### 帳號與實況主

- **一個應用程式，想用多少帳號都可以。** 每個帳號依照自己的模式各自挖取：**Drops 和點數**（一個頻道用於最佳活動，另一個用於你的實況主）、**僅 Drops** 或**僅點數**。

  <img src="../media/zh-TW/accounts-mining.avif" alt="首頁上多個帳號同時挖取">

- **每個新帳號都有設定精靈。** 登入後，可以從預設設定開始，也可以沿用另一個帳號的設定。選好模式、活動和實況主，再按「開始挖取」或「儲存並稍後開始」。

  <img src="../media/zh-TW/account-setup.avif" alt="新帳號的設定精靈，逐步示範">

- **一次變更，一鍵套用到其他帳號。** 在某個帳號上做了變更後，確認訊息會提供「套用到其他帳號」。這項變更會在你勾選的帳號上再做一次，這些帳號的其他設定則維持不變。

  <img src="../media/zh-TW/apply-to-accounts.avif" alt="透過「套用到其他帳號」把一項變更同步到其他帳號">

- **新增實況主。** 實況主會立即以預設選項開始挖取：應用程式的預設選項，或你為這個帳號選定的選項。需要的話可以當場調整：獎勵寶箱、Raid、時刻、連續觀看紀錄、聊天室、預測。

  <img src="../media/zh-TW/streamer-add.avif" alt="透過 Twitch 搜尋新增實況主，接著查看它的選項">

- **編輯實況主。** 「編輯」會開啟它的選項：獎勵寶箱、Raid、時刻、連續觀看紀錄、聊天室、預測……用一個開關就能啟用或停用它，不必把它從清單中移除。

  <img src="../media/zh-TW/streamer-edit.avif" alt="從實況主所在列開啟它的選項">

- **一次編輯多位實況主。** 勾選他們，再按「編輯所選的 N 位實況主」。只有你變更的選項才會套用。各實況主之間設定不同的選項會顯示為「混合」，它的開關會變成「是」和「否」兩個按鈕。接著，應用程式會詢問是否要把這些選項設為新實況主的預設選項。

  <img src="../media/zh-TW/streamers-group-edit.avif" alt="同時編輯多位實況主，其中有顯示為「混合」的選項">

- **匯入實況主。** 貼上一份清單（最多 500 個名稱），或沿用另一個帳號的清單，或使用該帳號追隨的頻道。清單會先經過檢查，只加入確實存在的頻道。即時更新每個帳號最多涵蓋 46 位實況主：清單中之後的實況主收不到 Raid、時刻和預測。

  <img src="../media/zh-TW/streamers-import.avif" alt="匯入實況主：貼上的清單、另一個帳號、追隨的頻道">

- **Spam Points。** 將某個頻道的自訂獎勵連續兌換多次。從你的點數清單中選擇頻道，或透過 Twitch 搜尋找到任何頻道：它的獎勵會和帳號的餘額一起顯示，餘額即時讀取。設定數量（最多 25 次，且不超過餘額），再按「兌換」。連發結束時會顯示結果，每次連發都會在「紀錄」中留下一筆。

  <img src="../media/zh-TW/spam-points.avif" alt="Spam Points：選擇一個頻道，將一項附留言的獎勵兌換 10 次，兌換出現在頻道的 Twitch 聊天室中，接著是紀錄中的那一筆">

### Drops

- **所有進行中的 Drops 活動。** 每個活動都有自己的詳細資料：獎勵、發行商帳號連結、「優先挖取」。清單來自 [Drops Alerts](https://dropsalerts.com) 專案，每 5 分鐘更新一次。

  <img src="../media/zh-TW/drops-catalogue.avif" alt="以月曆呈現的目錄、搜尋和某個活動的詳細資料">

- **你的挖取順序。** 把活動拖曳成你想要的順序。新活動會自動排到合適的位置：先是你的優先分類，再來是最早結束或剩餘分鐘數最少的活動。可以排除分類、關鍵字、某個活動或單一獎勵，並略過來不及完成的活動。在活動的詳細資料中按「優先挖取」，就能把它排到最前面。

  <img src="../media/zh-TW/drops-order.avif" alt="挖取順序、優先分類和排除項目">

- **收藏庫與「全部領取」。** 「收藏庫」頁面把獎勵分成「待領取」「進行中」和「已領取」三類。可以逐一領取、一次全部領取，或開啟「自動領取掉寶」。按下「連結發行商帳號」後，回到應用程式時會立即重新讀取帳號，連結狀態馬上就會顯示。

  <img src="../media/zh-TW/drops-inventory.avif" alt="收藏庫和「全部領取」按鈕">

- **「Drops 與徽章優先我的實況主」。** 對於目前的活動，如果你清單上某位實況主的直播可計入該活動，就會比其他頻道優先觀看。同一個頻道既能推進掉寶進度，又能幫你賺取它的頻道點數。預設為開啟。

  <img src="../media/zh-TW/drops-streamers-first.avif" alt="清單上的一位實況主，同時為了 Drops 和它的點數而觀看">

- **獎勵優先順序。** 當兩個活動排序相同時，挖掘器會先選擇獎勵符合你偏好類型（徽章、表情、遊戲內物品）的那個。它絕不會改變你設定的挖取順序。

- **徽章（測試版）。** 徽章頁面列出 Twitch 所有全域聊天徽章及取得方式。每個帳號都有自己的徽章頁面：進行中的徽章及其進度、仍可取得的徽章，以及已擁有的徽章。挖掘器會挖取 Drops 活動和 Twitch 任務中的徽章，並略過帳號已擁有的徽章。需要每天觀看的徽章會每天重新挖取，直到最後一天（「每天 20 分鐘，共 3 天」），活動任一分類中的頻道都算數。寶箱會給出「從 N 個中隨機 1 個徽章」。需要訂閱的徽章，要等你在活動期間於該分類的頻道自行訂閱或贈送訂閱且 Twitch 計入後才會挖取；已有的訂閱、Prime 訂閱或續訂都不算。活動開始之前不會挖取任何內容。在「挖取什麼，依順序」中，把優先分類、徽章和其他 Drops 活動拖曳成你想要的順序，或關閉其中一項。Twitch 任務只會在 Twitch 於其 Drops 面板中提供該任務的頻道上挖取。進化（Pichu → Pikachu）會在帳號擁有其進化前的徽章後挖取：挖掘器會在開始前自行配戴該徽章，且不會換回原本的徽章；在帳號的徽章設定中關閉「挖取徽章進化」即可停用。點擊徽章會開啟其詳細資訊：完整的條件、所有分類和頻道、系列、進化，以及每個帳號的進度。此功能為測試版。

  <img src="../media/zh-TW/badges.avif" alt="徽章頁面、一個帳號的徽章，接著是把徽章拖曳到最前面的「挖取什麼，依順序」">

### 追蹤

- **完整紀錄，連每個請求都看得到。** 「紀錄」頁面會保存每一個事件：掉寶、賺取和花費的點數、寶箱、下注、Raid、開台……可以依帳號、依分頁（Drops、徽章、點數、其他）和依類型篩選。**技術日誌 › 請求**會列出送往 Twitch 和其他服務的每個請求，以及結果和花費的時間。日誌和請求都不含任何權杖或密鑰：按「複製以回報錯誤」就能複製，直接貼到 [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) 或 [Discord](https://discord.gg/support-bots-891685895781113886) 上的錯誤回報中。

  <img src="../media/zh-TW/journal-requests.avif" alt="篩選後的紀錄，接著是技術日誌 › 請求">

- **統計頁面。** 顯示 7 天、30 天或 90 天內賺取的點數、已領取的掉寶、觀看時長和贏得的下注，每個數字背後都標出排名最高的實況主或分類。依實況主劃分的點數曲線會呈現餘額隨時間的變化，並在連續觀看、Raid、精彩時刻和下注處標出標記，每個標記都附帶提示說明。

  <img src="../media/zh-TW/stats.avif" alt="統計頁面的 30 天磚塊，接著是帶標記的實況主點數曲線">

- **暫停和每日暫停。** 只有一個暫停按鈕，可用於單一帳號或所有帳號：暫停到你繼續為止，或暫停 30 分鐘、1 小時、4 小時、至明天 09:00，或自訂日期和時間；暫停期間，同一個按鈕會顯示「繼續」。徽章會顯示定時暫停結束的時間，側邊欄中的每個帳號會顯示自己的狀態：執行中、已暫停，或需要查看的問題。每日暫停會在你選擇的時區裡，每天在兩個時間點之間暫停所有帳號的挖取，結束後自動恢復，並在頂端列顯示自己的徽章。
- **CSV 匯出與依分類統計。** 「紀錄」可以把已領取的掉寶匯出為 CSV 檔案（日期、帳號、分類、活動、掉寶、分鐘），並依帳號篩選，對話框會列出每個分類下已領取的數量。桌面應用程式透過儲存對話框儲存檔案，網頁儀表板則直接下載。

### 桌面或伺服器

- **到哪裡都是同一個介面。** 桌面應用程式（Linux、Windows、macOS）和 Docker 映像檔顯示的是同一個儀表板。Docker 可以在 VPS、NAS 或 Raspberry Pi 上執行，那台機器不需要螢幕或瀏覽器：儀表板可以在任何瀏覽器中開啟。在手機上，儀表板會配合螢幕顯示；透過 HTTPS 開啟時，還能像 App 一樣安裝。
- **輕巧。** 每個帳號約使用 3 MB 記憶體，幾乎不佔用 CPU，Docker 映像檔約 12 MB。

### 安全性與更新

- **工作階段受到保護。** 不需要 Twitch 密碼：你在自己的瀏覽器中透過 `twitch.tv/activate` 登入。工作階段會加密儲存在磁碟上，你可以在 [Twitch 設定](https://www.twitch.tv/settings/connections)中撤銷。在桌面版中，可選用的應用程式密碼會鎖定應用程式：輸入密碼之前什麼都不會挖取。在 Docker 版本中，儀表板有自己的密碼，閒置幾分鐘後會自動鎖定，並在代理伺服器後方透過 HTTPS 執行。連續輸錯 5 次密碼，該 IP 位址會被封鎖 30 秒。

  <img src="../media/zh-TW/security-lock.avif" alt="鎖頭：應用程式已鎖定，再用它的密碼解鎖">

- **簽章更新，絕不自動安裝。** 頂端列會出現附有版本號碼的「更新」徽章，桌面通知也會提醒你，預設開啟（**設定 › 通知**中的開關可以關閉它；在瀏覽器中，出現詢問時按「允許」即可）。按「安裝並重新啟動」會先請你確認，接著應用程式會安裝該版本並重新啟動。若設定了應用程式密碼，重新啟動後會處於鎖定狀態：再次輸入密碼即可繼續挖取。只有經過本專案簽章的版本才會安裝。Docker 儀表板和 systemd 二進位檔同樣會通知你，並附上需要執行的指令。

  <img src="../media/zh-TW/updates.avif" alt="「更新」徽章、確認，接著安裝">

### 此外

- 與其他挖掘器一樣：獎勵寶箱、Raid、時刻、社群目標、預測（策略、下注時機、條件）、連續觀看紀錄和已訂閱頻道優先、聊天室在線（可關閉）、新實況主的預設選項、附圖片並為每個目標配有「測試」按鈕的 Discord、Telegram 和 Webhook 通知（在伺服器上，通知語言在**設定 › 通知**中選擇）、全域或個別帳號的代理伺服器。
- 29 種語言、深色和淺色主題、桌面通知、系統匣圖示、隨系統啟動。

  <img src="../media/zh-TW/basics.avif" alt="多種語言、淺色和深色主題、一則桌面通知和系統匣圖示">

- **透過貼上權杖清單新增帳戶。** 已經在別處登入了嗎？貼上存取權杖清單，每行一個（最多 50 個）；每個權杖會即時檢查，並顯示「已檢查：n/N」計數，失敗的行可以重試，接著一個設定步驟會套用到整批帳戶。匯入在背景執行，即時顯示每個帳戶的進度，可中途取消；之後重試只會重新送出失敗的帳戶。移除以此方式匯入的帳戶時，也可以選擇同時在 Twitch 上撤銷其權杖。

> [!WARNING]
> 本專案與 Twitch 無關聯。自動操作帳號可能違反 Twitch 的條款或活動規則。

## 目錄

- [比較](#comparison)
- [Docker](#docker)
- [運作原理](#how-it-works)
- [常見問題](#faq)
- [相關專案](#related-projects)
- [安全性](#security)
- [從原始碼建置](#build)
- [授權](#license)

## <a name="comparison"></a>比較

[Twitch Miner](https://github.com/SunkwiBOT/twitch-miner) 與 [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)（頻道點數）和 [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner)（Drops）的比較如下。已於 2026 年 11 月對照它們的程式碼和 README 確認。

| | 本專案 | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops 活動：尋找、挖取並領取 | ✅ | ❌ ¹ | ✅ |
| Twitch 徽章：所有徽章的清單、每個帳號的徽章、依你的順序挖取徽章 Drops 與 Twitch 任務（測試版） | ✅ | ❌ | 部分 ² |
| 頻道點數：寶箱、Raid、時刻、預測 | ✅ | ✅ | ❌ |
| Spam Points：將某個頻道的自訂獎勵連續兌換多次 | ✅ | ❌ | ❌ |
| 同一個帳號同時挖取 Drops 和頻道點數 | ✅ | 部分 ¹ | ❌ |
| 一個應用程式，想用多少帳號都可以 | ✅ | ❌ ³ | ❌ |
| 一次變更多個帳號的設定 | ✅ | ❌ | ❌ |
| 圖形介面 | ✅ 桌面版和網頁版 ⁴ | ❌ ⁵ | ✅ 桌面版 |
| 家用伺服器的網頁儀表板（Docker），有密碼保護，可透過 HTTPS 連線 | ✅ | ❌ ⁵ | ❌ |
| 工作階段加密儲存在磁碟上，可選用應用程式密碼 | ✅ | ❌ | ❌ |
| 介面支援 29 種語言 | ✅ | ❌ | 部分 ⁶ |
| 在應用程式內完成簽章更新（桌面版） | ✅ | ❌ | ❌ |
| 每個新帳號都有設定精靈 | ✅ | ❌ ⁵ | ❌ |
| 在介面中新增和編輯實況主，可逐一或批次處理 | ✅ | ❌ ⁵ | — |
| 透過搜尋、清單、另一個帳號或追隨的頻道匯入實況主 | ✅ | 部分 ⁷ | — |
| 附圖片的 Discord、Telegram 和 Webhook 通知 | ✅ | 部分 ⁸ | ❌ |
| 所有 Drops 活動的月曆，支援搜尋 | ✅ | ❌ | 部分 ⁹ |
| 依活動設定挖取順序、排除的詞、忽略活動或獎勵 | ✅ | ❌ | ❌ ¹⁰ |
| 挖取活動的 Drops 時，優先觀看你清單上的實況主 | ✅ | 部分 ¹¹ | ❌ |
| 介面中可篩選的事件紀錄，以及每個請求的清單 | ✅ | ❌ ⁵ | ❌ |
| 每個帳號約使用 3 MB 記憶體（Docker 伺服器） | ✅ | — ¹² | — ¹² |

¹ 它只觀看自己清單上的實況主：會推進並領取這些直播的 Drops，但不會挑選活動。\
² 有一個設定可挖取或排除獎勵為徽章或表情符號的 Drops 活動；沒有 Twitch 任務，也沒有徽章清單。\
³ 每個帳號一個程序。\
⁴ 可以當作桌面應用程式在你的 PC 上執行，也可以用 Docker 在伺服器（VPS、NAS、Raspberry Pi）上執行，或兩者並用；那台機器不需要圖形環境，也不需要瀏覽器：儀表板可以在任何瀏覽器中開啟。\
⁵ 以 Python 檔案設定；它的網頁會顯示點數圖表和日誌檔案。\
⁶ 21 種語言。\
⁷ 它可以使用該帳號追隨的頻道。\
⁸ 只有文字，沒有圖片。它另外支援 Matrix、Pushover 和 Gotify，本專案則沒有。\
⁹ 有篩選條件的活動清單，沒有月曆，也沒有搜尋。\
¹⁰ 它有優先分類和排除的分類，依分類名稱選擇。\
¹¹ 預設會先觀看清單上直播附有 Drops 的實況主（它的 `DROPS` 優先順序），但絕不會觀看其他頻道。\
¹² 未測量。

## <a name="docker"></a>Docker

伺服器會在瀏覽器中提供同一個儀表板。映像檔為 `ghcr.io/sunkwibot/twitch-miner`，支援 `amd64` 和 `arm64`。有新版本推出時，儀表板會告訴你：先執行 `docker compose pull`，再執行 `docker compose up -d`。

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # 每次啟動時都取得最新的映像檔
    restart: unless-stopped
    ports: ["8080:8080"]   # 位於 HTTPS 反向代理伺服器後方時：刪除這一行，或改成 "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # 必填，至少 12 個字元
      # MINER_PUBLIC_URL: "https://miner.example.com"   # 僅在 HTTPS 反向代理伺服器後方使用
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

開啟 <http://localhost:8080>（或這台機器在你網路中的位址），然後輸入密碼。

### 變數

| 變數 | 預設值 | 用途 |
|---|---|---|
| `MINER_PASSWORD` | 無 | 儀表板密碼，必填：沒有設定的話，伺服器會拒絕啟動。至少 12 個字元（較短的密碼會在啟動時顯示警告）。範例值 `change-me` 會被拒絕。 |
| `MINER_PASSWORD_FILE` | 無 | 從檔案讀取 `MINER_PASSWORD`，例如 Docker secret。兩者只能擇一設定。 |
| `MINER_PUBLIC_URL` | 無 | 僅在 HTTPS 反向代理伺服器（Caddy、nginx、Traefik、Cloudflare Tunnel……）後方使用：儀表板的 `https://` 位址，不含路徑，例如 `https://miner.example.com`。代理伺服器必須轉送 `X-Forwarded-Proto`、`X-Forwarded-For` 和 websocket（常見的代理伺服器預設都會這麼做），而且容器的連接埠不能被直接連到。如此一來，儀表板會拒絕一切不是經由代理伺服器透過 HTTPS 送來的請求，並在某個位址輸錯 5 次密碼後將其封鎖 30 秒。代理伺服器必須在同一台機器或其區域網路中執行：來自公開位址的連線會被拒絕。 |
| `MINER_TRUSTED_HOPS` | `1` | 僅搭配 `MINER_PUBLIC_URL` 使用：會在 `X-Forwarded-For` 加上一筆項目的代理伺服器數量，介於 1 到 10。兩台代理伺服器前後串接時（例如 Cloudflare 位於 Traefik 前方）設為 `2`。 |
| `MINER_PROXY_NETWORKS` | 無 | 僅與 `MINER_PUBLIC_URL` 一起使用：代理伺服器連線所來自的網路，以逗號、空格或換行分隔：單一位址，或 `192.168.1.0/24`、`2001:db8:1::/64` 這樣的網路。設定後，只接受來自回送位址和這些網路的代理伺服器，不再接受整個私有區域網路。空值、`/0` 或設定了主機位元的網路，只有在同時設定 `MINER_AUTH_HEADER` 時才會阻止啟動；沒有它時，它們會被接受並顯示警告。 |
| `MINER_AUTH_HEADER` | 無 | Forward auth / SSO：代理伺服器設定為已登入使用者的標頭名稱：Authelia 為 `Remote-User`，Authentik 為 `X-authentik-username`，oauth2-proxy（搭配 `--set-xauthrequest`）為 `X-Auth-Request-User`。經代理伺服器放行的請求——來自 `MINER_PROXY_NETWORKS` 中的位址，且 `Host` 與 `MINER_PUBLIC_URL` 相符——若該標頭只有一行非空值，就能在不使用 `MINER_PASSWORD` 的情況下登入。需要 `MINER_PUBLIC_URL` 以及明確設定的 `MINER_PROXY_NETWORKS`；僅靠回送位址永遠不夠。拒絕代理伺服器、瀏覽器或用戶端已經寫入的標頭（`X-Forwarded-*`、`Host`、`Cookie`、`Authorization`、`X-Real-IP`、`Forwarded`、`Via`、`User-Agent`、`Accept*`、`Sec-Fetch-*`、`Cache-Control`、`CF-Connecting-IP`、`Priority`、`Connection`、`Origin`、`Referer`、`X-Request-Id`、`Content-Type`……）。 |
| `MINER_AUTH_USERS` | 無 | 僅搭配 `MINER_AUTH_HEADER` 使用：允許登入的標頭值，像 `MINER_PROXY_NETWORKS` 一樣以逗號、空格或換行分隔。比對區分大小寫。已設定但為空的值會被拒絕，就像設定了標頭卻留空的 `MINER_PROXY_NETWORKS` 一樣。未設定時，代理伺服器設定的任何值都能登入——每個已登入的 SSO 使用者都會成為管理員——伺服器會在啟動時提出此警告；設定後，伺服器會在啟動時記錄辨識到的值的數量。 |
| `MINER_VAULT_PASSPHRASE` | 無 | 僅適用於從已設定應用程式密碼的桌面版複製過來的資料目錄：在啟動時將其解鎖。全新架設的伺服器完全不需要它。 |
| `MINER_VAULT_PASSPHRASE_FILE` | 無 | 從檔案讀取 `MINER_VAULT_PASSPHRASE`。兩者只能擇一設定。 |
| `DATA_DIR` | 映像檔中為 `/data`，否則為工作目錄下的 `data` | 資料庫、金鑰檔案和日誌。 |
| `LOG_DIR` | `$DATA_DIR/logs` | 日誌檔案：每天一個，保留最近 14 個。 |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | 監聽位址：一個 IP 位址和一個介於 1 到 65535 的連接埠。 |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | 日誌等級。 |

伺服器只會在啟動時讀取並檢查這些變數一次。值無效時，伺服器會停止並顯示訊息，而不是默默改用預設值。

在反向代理伺服器後方，請設定 `MINER_PUBLIC_URL`：沒有它時，第一個經由代理伺服器抵達的請求會在日誌中寫下一則警告，因為登入嘗試此時會依代理伺服器的位址計算，一位訪客輸錯密碼就會把所有人都鎖在外面。即時連線每 30 秒送出一次 ping，讓代理伺服器不會在沒有動靜時把它關閉。當 Twitch 或網路出錯時，伺服器會回應 `424`，絕不回應 `502` 或 `504`：在 Cloudflare 後方，你看到的是真正的錯誤，而不是 Cloudflare 的頁面。

#### 透過受信任的代理伺服器登入（forward auth / SSO）

_自 v1.16.0 起可用。_

若多個自架應用程式（Authelia、Authentik、oauth2-proxy……）前方只有一個統一登入，
`MINER_AUTH_HEADER` 可省略儀表板自己的密碼：只要代理伺服器上的登入就足夠了。同時也
請將 `MINER_AUTH_USERS` 設定為允許的值，否則任何 SSO 使用者都會以管理員身分登入。
`MINER_PASSWORD` 仍然對直接以 `POST /api/auth` 登入的指令碼有效，但絕不透過瀏覽器：
下方的路由器會把其他所有請求都送經 `forwardAuth`，所以造訪儀表板的瀏覽器一律需要
先有可用的 SSO。第二個路由器只處理 `/api/auth` 上的 `POST` 和 `DELETE`——即密碼登入
與登出——並自行清空該標頭，而不是移除 `forwardAuth`：若把這條規則放寬到那一條路徑
之外，用戶端偽造的標頭也會從那裡通過。

代理伺服器必須在前往 miner 的途中保持請求原始的 `Host` 不變，否則 `MINER_PUBLIC_URL`
永遠無法與之相符，該標頭會被靜默忽略，首次發生時會在日誌中寫下警告：nginx 需要
`proxy_set_header Host $host;`，Apache 需要 `ProxyPreserveHost On`。Traefik 和 Caddy
預設就會保持它不變，兩者都不需要額外加上任何內容。

這些產品都不會移除用戶端自行傳送的標頭：從代理伺服器到 miner 的每一條路由都需要
自己的規則來覆寫它（Traefik 的 `forwardAuth` 只會移除 `authResponseHeaders` 中列出
的標頭，其他用戶端標頭會原樣轉送）。讓代理伺服器和 miner 單獨執行在屬於它們自己的
網路上，為代理伺服器設定一個靜態位址，並且絕不要公開 miner 的連接埠：任何能連線到
它的其他東西——另一個容器、主機上的行程——都可能自行設定該標頭。

Authelia 位於 Traefik 之後，兩者單獨處於 `miner-net` 網路中，將已登入使用者轉送到
`Remote-User`：

```yaml
networks:
  edge: {}          # Traefik 和 Authelia 在這裡通訊
  miner-net:        # 只有 Traefik 和 miner 在這個網路上
    ipam:
      config:
        - subnet: 10.10.0.0/29

services:
  traefik:
    # 你平常的 Traefik 設定：進入點、其他網路、其他應用程式
    networks:
      edge: {}
      miner-net:
        ipv4_address: 10.10.0.2

  authelia:
    # 你平常的 Authelia 設定
    networks:
      edge: {}
    labels:
      traefik.http.middlewares.authelia.forwardAuth.address: "http://authelia:9091/api/authz/forward-auth"
      traefik.http.middlewares.authelia.forwardAuth.trustForwardHeader: "true"
      traefik.http.middlewares.authelia.forwardAuth.authResponseHeaders: "Remote-User"
      # 供下方指令碼用的路由器使用：清空 Remote-User，而非跳過驗證，
      # 如此用戶端偽造的標頭在這條路由上也同樣無效
      traefik.http.middlewares.blank-remote-user.headers.customrequestheaders.Remote-User: ""

  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    networks:
      miner-net:
        ipv4_address: 10.10.0.3   # 同樣是靜態的：隨機挑選的位址可能會衝突
    # 沒有 "ports"：這個網路只能從 10.10.0.2 連到 Traefik
    environment:
      MINER_PASSWORD: "change-me"
      MINER_PUBLIC_URL: "https://miner.example.com"
      MINER_PROXY_NETWORKS: "10.10.0.2"
      MINER_AUTH_HEADER: "Remote-User"
      MINER_AUTH_USERS: "alice,bob"
    volumes:
      - miner-data:/data
    labels:
      traefik.http.routers.miner.rule: "Host(`miner.example.com`)"
      traefik.http.routers.miner.middlewares: "authelia@docker" # 在每個指向 miner 的路由器上重複此項
      # 供指令碼使用，搭配 MINER_PASSWORD：繞過 Authelia，但不會跳過標頭的清空
      traefik.http.routers.miner-api.rule: "Host(`miner.example.com`) && Path(`/api/auth`) && (Method(`POST`) || Method(`DELETE`))"
      traefik.http.routers.miner-api.middlewares: "blank-remote-user@docker"

volumes:
  miner-data:
```

#### 強化其他代理伺服器

這些產品都不會自行移除用戶端傳送的標頭副本：在那條從 SSO 回應中設定該標頭的指令
之前，於通往 miner 的每一條路由（包括指令碼專用的那條）加入下面這條移除指令。

- **Caddy**：在 `forward_auth` 之前緊接著使用 `request_header -Remote-User`。它的展開
  形式（`request_header <名稱> {rp.header.<名稱>}`）只會加入閘道自己的標頭，絕不會
  移除用戶端的。
- **nginx**：在 `auth_request` 之前使用 `proxy_set_header Remote-User "";`，以及
  `proxy_set_header Host $host;`（`auth_request` 本身需要以
  `--with-http_auth_request_module` 編譯）。nginx 還會丟棄名稱中含有底線的任何標頭，
  除非設定了 `underscores_in_headers on;`；這正是 `MINER_AUTH_HEADER` 拒絕這類名稱的
  原因。
- **Apache**：在加入真正標頭的 `RequestHeader set` 之前使用
  `RequestHeader unset Remote-User`，以及因應上述相同的 `Host` 要求使用
  `ProxyPreserveHost On`。
- **HAProxy**：在依驗證回應設定該標頭的規則之前使用
  `http-request del-header Remote-User`。
- **Cloudflare Tunnel**：讓 `httpHostHeader` 保持未設定（預設情況下它已經會轉送用戶端
  請求的公開 `Host`）；它是通道，而非清理標頭的代理伺服器，因此應將它與上述其中一個
  代理伺服器搭配用於 `MINER_AUTH_HEADER`，而非只信任它自己。
- **Tailscale Serve**：`Tailscale-User-Login`。`tailscaled` 在設定自己的值之前，已經
  會移除用戶端傳送的任何副本，不必自行清理；重要的只是必須只有 `tailscale serve`，
  絕非直接連線，才能到達 miner——將它綁定到 `127.0.0.1`，只讓 Serve 來做中介。

無論選擇哪種代理伺服器，在信任它之前先測試一次：在登出 SSO 後，直接對 miner 自身的
位址發出 `curl` 請求（繞過代理伺服器，從執行它的機器或容器上），手動設定該標頭，例如
`curl -H "Remote-User: mallory" http://127.0.0.1:8080/api/snapshot`；應得到 `401`
回應，而絕非快照內容。

### <a name="systemd"></a>不使用 Docker（systemd）

在不使用 Docker 的 Linux 伺服器上，發行版本裡也包含單一檔案形式的伺服器，內建儀表板：
`twitch-miner-server_<版本>_linux_x64` 或 `twitch-miner-server_<版本>_linux_arm64`。
這就是 Docker 映像檔所執行的那個程式，靜態連結，但它會讀取系統的 CA 憑證用於 HTTPS（Docker 映像檔基於相同原因也複製了這些憑證）。它接受相同的變數。

```bash
# ARM64: twitch-miner-server_<版本>_linux_arm64
sudo install -m 755 twitch-miner-server_<版本>_linux_x64 /usr/local/bin/twitch-miner
sudo install -m 600 /dev/null /etc/twitch-miner.env
```

把你自己的密碼（至少 12 個字元）寫入 `/etc/twitch-miner.env`，這個檔案只有 root 能讀取：

```ini
MINER_PASSWORD=change-me
```

接著是服務檔案，`/etc/systemd/system/twitch-miner.service`：

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

systemd 會建立 `/var/lib/twitch-miner` 用來存放資料，並讓伺服器以自己專屬的使用者執行，絕不以
root 身分執行。開啟 `http://<伺服器位址>:8080`；日誌在 `journalctl -u twitch-miner` 裡。

這個檔案不會自動更新。當儀表板顯示有新版本時，從發行版本下載新檔案，取代舊檔案後重新啟動：

```bash
# ARM64: twitch-miner-server_<版本>_linux_arm64
sudo install -m 755 twitch-miner-server_<版本>_linux_x64 /usr/local/bin/twitch-miner
sudo systemctl restart twitch-miner
```

## <a name="how-it-works"></a>運作原理

每個帳號每 20 秒會選定一次自己的兩個頻道：先為佇列中最前面的活動挑出最合適的直播頻道，再安排點數清單上的實況主。沒有直播頻道的活動會被跳過，改選下一個有直播頻道的活動（在其佇列中顯示「沒有直播中的頻道」）；直播頻道每 90 秒重新檢查一次，帳號的實況主一開播就會立刻回到該活動。

Twitch 對每個帳號同一時間只會計入**一個 Drops 活動**和**兩個頻道的點數**。自 2026 年 10 月起，必須先連結發行商帳號才能領取掉寶。未連結的活動仍會累積進度，儀表板會顯示連結的入口。

如果 Twitch 本身對排名最前面的活動的進度，在 30 分鐘真實觀看後仍未變動，表示它沒有計入這個活動：帳號會把它先擱置一旁，排名居次的活動遞補上這個名額，並在 6 小時後自動重試——如果進度先恢復，則會更早重試，不過被重試的活動仍可能排在正在挖取的那個活動之後。任何進度都會重設這段等待時間，且適用於每一種活動類型（Drops、徽章、任務）。如果 Twitch 改為計入同一類別的另一個活動，並且該活動完成或被領取，擱置會立即解除，而不必等候完整的 6 小時。重新啟動應用程式或伺服器會忘記所有被擱置的紀錄：卡住的活動只會重新獲得一段全新的 30 分鐘。

活動清單來自 [Drops Alerts](https://dropsalerts.com) 專案（每 5 分鐘更新一次），因為 Twitch 不會讓 TV 用戶端看到它。

## <a name="faq"></a>常見問題

### 風險與 Twitch

**我的帳號會被停權嗎？**\
沒有人能保證不會。本專案與 Twitch 無關聯，自動操作帳號可能違反 Twitch 的條款或活動規則。挖掘器每個帳號最多觀看兩個頻道，這是 Twitch 的上限，而且不會下載任何影片。要不要使用，由你自行判斷。

**真的會觀看影片嗎？**\
不會，不會下載任何影片。為了 Drops，挖掘器每 15 秒以最低畫質讀取一次直播的播放清單，而且只請求每個新片段的標頭。為了點數，它每分鐘會送出一次 Twitch 的「已觀看一分鐘」訊號。每個正在觀看的頻道都會顯示已請求的片段數。

**為什麼最多兩個頻道？**\
這是 Twitch 的限制：每個帳號同一時間只會計入兩個正在觀看的頻道，而且一次只計入一個 Drops 活動。在 **Drops 和點數**模式下，一個頻道用於活動，另一個用於你的實況主。**帳號設定**中的「一個頻道」選項比較低調。

### Drops

**某個活動沒有被挖取，或某個掉寶沒有進度。**\
開啟該帳號的 **Drops** 頁面。在「挖取順序」中，每個等待中的活動都會說明原因：「沒有直播中的頻道」或「等待中（一次一個活動）」。「Twitch 不計入——…重試」表示 30 分鐘的真實觀看並未在 Twitch 那端帶來任何進度：該活動會被擱置，並在顯示的時間後自動重試，如果進度提早恢復則會更早重試。沒有出現在這份清單中的活動已被排除，原因可能是「挖取什麼，依順序」、「挖取哪些活動」設定、**排除**分頁中的某條規則、它來不及完成，或它剩下的獎勵都需要訂閱。需要訂閱的獎勵會等你在活動期間於該分類的頻道自行訂閱或贈送訂閱（已有的訂閱、Prime 訂閱或續訂都不算），需要每天觀看的徽章會等到隔天，活動開始之前不會挖取任何內容，而接在另一個掉寶之後的掉寶，會等前一個完成後才開始。在**僅點數**模式下，挖掘器不會選擇任何活動。在活動的詳細資料中按「優先挖取」可以把它排到最前面；被忽略的活動可以在**排除**分頁中透過「恢復挖取」重新加入。**活動**分頁的**已阻擋**群組會顯示相同的原因。

**需要連結發行商帳號嗎？**\
要領取的話，需要：自 2026 年 10 月起，Twitch 只允許在發行商帳號已連結時領取掉寶。預設情況下，未連結的分類仍會累積進度，領取則會等到連結之後。「未連結」徽章會標示出這種情況，活動詳細資料中的「連結發行商帳號」會開啟連結頁面。如果只想挖取已連結的分類，請選擇「僅已連結發行商帳號的分類」。

### 安裝與資料

**桌面應用程式還是 Docker？**\
桌面應用程式會在你的 PC 開機期間挖取，提供系統匣圖示、可選用的密碼和一鍵更新。Docker 則在一台隨時開著的機器（VPS、NAS、Raspberry Pi）上日夜挖取；你在瀏覽器中開啟它，並由 `MINER_PASSWORD` 保護。兩者的介面相同。

**我的資料在哪裡？**\
在資料目錄中：Linux 上是 `~/.local/share/twitch-miner`，Windows 上是 `%APPDATA%\twitch-miner`，macOS 上是 `~/Library/Application Support/twitch-miner`，Docker 則是 `/data` 磁碟區。在桌面版中，**設定 › 桌面**可以一鍵開啟它。裡面有 `miner.db`（帳號、設定、紀錄）、`master.key`（未設定應用程式密碼時的工作階段金鑰）和日誌。

**要怎麼更新？**\
桌面應用程式啟動時會檢查是否有新版本，之後每小時檢查一次（可在**設定 › 關於**中變更，找到新版本時也會通知你）。按「更新」徽章，再按「安裝並重新啟動」。使用 Docker 時，儀表板會通知你：執行 `docker compose pull`，再執行 `docker compose up -d`。不使用 Docker 時，儀表板也會以相同方式通知你：替換檔案並重新啟動服務的方法請見[不使用 Docker](#systemd)。應用程式會依從新到舊列出你目前版本之後每個版本的更新內容；每個版本的說明也在其 GitHub 發布頁上。

**我忘了應用程式密碼。**\
密碼無法找回：你的 Twitch 工作階段正是用它加密的。請結束應用程式、刪除資料目錄，然後重新新增帳號；這些帳號的設定必須重新設定。使用 Docker 時，儀表板密碼要在 `MINER_PASSWORD` 中變更，再執行 `docker compose up -d`。

### 點數與預測

**每小時能賺多少點數？**\
這取決於 Twitch 和各個頻道。挖掘器能賺到的，就是 Twitch 給觀眾的那些：觀看時數點數、獎勵寶箱（大約每 15 分鐘一個）和連續觀看獎勵。訂閱會依等級讓點數變成 ×1.2 到 ×2；「已訂閱頻道優先」預設開啟，這些頻道會排在清單其餘頻道之前。首頁會顯示賺到的點數，「紀錄」則列出每一筆收入。

**下注有風險嗎？**\
有，點數可能會輸掉。預測預設為關閉，需要逐一為實況主開啟。你可以設定策略、下注時機、下注額（預設為餘額的 5%，最多 50,000 點）、最低餘額，也可以加上一個條件，例如賠率至少 1.5。請控制下注額。低於 10 點的下注不會送出。

**某位實況主沒有被觀看。**\
只有已啟用且正在直播的實況主才會被觀看，而且只能使用空著的頻道：在 **Drops 和點數**模式下，其中一個頻道用於目前的活動；在**僅 Drops** 模式下，不會觀看任何實況主；已暫停的帳號什麼都不會觀看。頻道名額依以下順序分配，並以徽章顯示在每個正在觀看的頻道上：「連續紀錄有風險」「連續觀看獎勵」「已訂閱」，然後是「清單順序」。把這位實況主在清單中往上移，或關閉「連續觀看紀錄優先」和「已訂閱頻道優先」。

## <a name="related-projects"></a>相關專案

同一位作者的其他專案：

- [**Drops Alerts**](https://dropsalerts.com)：免費的公開 API，收錄所有進行中的 Twitch Drops 和獎勵活動，每分鐘更新一次。本應用程式的活動目錄就是從這裡讀取。
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com)：一個 Discord 機器人，會在你的伺服器中通知 Twitch 直播和 Drops 活動。
- [**Kick Alerts 🔔**](https://kick.sunkwi.com)：一個 Discord 機器人，會在你的伺服器中通知 Kick 直播和 Drops 活動。

## <a name="security"></a>安全性

- 權杖以 AES-256-GCM 加密封存。金鑰可以是資料庫旁隨機產生的 `master.key`（預設，不必做任何事），也可以用 Argon2id 從應用程式密碼衍生。
- 資料目錄和日誌目錄是私人的（`0700`）：只有執行挖掘器的使用者才能開啟它們。
- HTTP 儀表板使用固定時間的密碼比對，以及 HttpOnly、SameSite=Strict 的 Cookie（透過 HTTPS 時另加 `Secure` 和 `__Host-` 前綴）。伺服器只保存每個工作階段的 SHA-256。工作階段會在 30 天後或登出時結束。伺服器會先檢查工作階段，才讀取任何請求內容，並將請求內容限制在 256 KiB 以內。
- 同一個 IP 位址連續輸錯 5 次密碼後（登入表單或 bearer），該位址必須等待 30 秒。伺服器會直接回傳 `429` 和 `Retry-After`，不會檢查密碼。每次輸錯密碼，回應也會延遲半秒。同一個連線位址（包括共用的代理伺服器）在一分鐘內輸錯密碼超過 30 次後，該位址每 2 秒只能嘗試一次：猜測會變慢，且不會有人被鎖在外面。
- 在你的網路以外使用時，請把儀表板放在 HTTPS 反向代理伺服器後方，並設定 `MINER_PUBLIC_URL`（請參閱 [Docker](#docker)）。
- 挖掘器只會與 `twitch.tv`、`ttvnw.net`、`jtvnw.net`、`twitchcdn.net`、`dropsalerts.com` 以及你設定的通知目標通訊。應用程式（桌面版和 Docker 版）會在 `github.com` 和 `api.github.com` 檢查新版本及其說明，桌面版會從本專案的 GitHub releases 下載新版本。新版本都經過簽章：只會安裝畫面上顯示的那個版本。

## <a name="build"></a>從原始碼建置

**必要條件：** Rust、Node.js，以及適用於你系統的 [Tauri 必要條件](https://v2.tauri.app/start/prerequisites/)。

```bash
npm ci
npm run build          # 儀表板
cargo build --release -p miner-server   # 伺服器：target/release/twitch-miner
npm run bundle:linux   # 桌面版安裝套件（需要 Tauri 的 Linux 相依套件）
```

程式碼檢查：

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier、oxlint（型別感知）、svelte-check 和 inlang 的 i18n 檢查
```

介面的 TypeScript 型別是用 [ts-rs](https://github.com/Aleph-Alpha/ts-rs) 從 Rust 型別產生到 `ui/src/lib/bindings` 的。變更介面會用到的型別後，請執行 `npm run bindings`。

## <a name="license"></a>授權

[MIT](../../LICENSE)。靈感來自 [DevilXD 的 TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner)、[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2) 及其衍生專案。

<img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-miner" width="1" height="1" alt="">
