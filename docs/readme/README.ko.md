<p align="center">
  <img src="../../ui/public/app-mark.svg" width="112" height="112" alt="Twitch Miner 로고">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Twitch Drops, 배지, 채널 포인트를 원하는 만큼의 계정으로 브라우저도, 열어 둔 Twitch 탭도 없이 마이닝해요.</strong>
</p>

<p align="center">
  Linux, Windows, macOS용 데스크톱 앱이에요. 같은 웹 대시보드를 담은 Docker 이미지도 있어서 홈 서버나 Raspberry Pi에서 돌릴 수 있어요. 계정마다 메모리를 약 3 MB 쓰고 CPU는 거의 쓰지 않아요.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="최신 릴리스"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="다운로드 수"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="플랫폼: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="amd64 및 arm64용 Docker 이미지"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT 라이선스"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="조회수">
</p>

<p align="center">
  <a href="#download"><strong>다운로드</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://dropsalerts.com">Drops Alerts</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new">버그 신고</a>
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
  <a href="README.zh-TW.md">繁體中文</a> ·
  <a href="README.ja.md">日本語</a> ·
  <strong>한국어</strong> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.vi.md">Tiếng&nbsp;Việt</a> ·
  <a href="README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="../media/ko/presentation.avif" alt="Twitch Miner 소개 영상">
</p>

## <a name="download"></a>다운로드

[최신 릴리스](https://github.com/SunkwiBOT/twitch-miner/releases/latest)에서 내 시스템에 맞는 파일을 내려받으세요.

| 시스템 | 패키지 |
|---|---|
| **Windows** · x64 | [설치 프로그램](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_x64-setup.exe) 또는 [포터블 실행 파일](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_x64_portable.exe) |
| **Windows** · ARM64 | [설치 프로그램](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_arm64-setup.exe) 또는 [포터블 실행 파일](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_arm64_portable.exe) |
| **macOS** 13 이상 · universal: Intel 및 Apple Silicon | [디스크 이미지](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_macos_universal.dmg)(`.dmg`) |
| **Linux** · x86-64 | [`.AppImage`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.AppImage), [`.deb`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.deb)(Debian, Ubuntu) 또는 [`.rpm`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.rpm)(Fedora, openSUSE) |
| **Linux** · ARM64 | [`.AppImage`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_arm64.AppImage), [`.deb`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_arm64.deb)(Debian, Ubuntu, Raspberry Pi OS) 또는 [`.rpm`](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_arm64.rpm)(Fedora, openSUSE) |
| **Linux 서버** · x86-64 또는 ARM64 | 서버만, Docker 없이([x86-64](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner-server_linux_x64) 또는 [ARM64](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner-server_linux_arm64)): [Docker 없이](#systemd) 참고 |

Linux에서는 터미널에서도 설치할 수 있어요. ARM64에서는 명령어의 `x64`를 `arm64`로 바꾸세요.

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

AppImage, 모든 배포판:

```bash
curl -LO https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_linux_x64.AppImage
chmod +x twitch-miner_linux_x64.AppImage
./twitch-miner_linux_x64.AppImage
```

파일 이름이 바뀌지 않으므로 같은 명령어로 기존 설치도 업데이트할 수 있어요.

macOS 빌드는 공증을 받지 않았어요. 처음에는 앱을 먼저 여세요. macOS가 막으면 **시스템 설정 › 개인정보 보호 및 보안**으로 가서 **그래도 열기**를 클릭하세요.

업데이트는 서명되어 있어요. 앱은 시작할 때 새 버전이 있는지 확인하고, 그 뒤로는 한 시간마다 확인해요(확인 주기는 **설정 › 정보**에서 바꿀 수 있어요). 업데이트를 스스로 설치하지는 않아요. 화면에 뜨는 메시지나 **설정 › 정보**에서 클릭 한 번으로 설치하면 돼요.

데이터는 사용자 데이터 폴더에 저장돼요:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="features"></a>기능

### 계정과 스트리머

- **앱 하나에 원하는 만큼의 계정.** 계정마다 각자의 모드로 따로 마이닝해요. **Drops와 포인트**(한 채널은 가장 좋은 캠페인, 다른 채널은 내 스트리머), **Drops만**, **포인트만** 중에서 고를 수 있어요.

  <img src="../media/ko/accounts-mining.avif" alt="홈에서 여러 계정이 동시에 마이닝하는 모습">

- **새 계정마다 설정 마법사.** 로그인한 뒤 기본 설정에서 시작하거나 다른 계정의 설정을 가져와요. 모드, 캠페인, 스트리머를 고른 다음 ‘마이닝 시작’이나 ‘저장하고 나중에 시작’을 누르세요.

  <img src="../media/ko/account-setup.avif" alt="새 계정의 설정 마법사를 단계별로 진행하는 모습">

- **변경 하나를 클릭 한 번으로 다른 계정에도.** 한 계정에서 무언가를 바꾸면 확인 메시지에 ‘다른 계정에 적용’이 나타나요. 체크한 계정에도 같은 변경이 그대로 적용되고, 나머지 설정은 그대로 유지돼요.

  <img src="../media/ko/apply-to-accounts.avif" alt="‘다른 계정에 적용’으로 같은 변경을 다른 계정에도 적용하는 모습">

- **스트리머 추가.** 추가한 스트리머는 기본 옵션으로 바로 마이닝을 시작해요. 기본 옵션은 앱의 기본값이거나, 이 계정에 맞게 골라 둔 옵션이에요. 필요하면 그 자리에서 조정할 수 있어요: 보너스 상자, 레이드, 모먼트, 시청 스트릭, 채팅, 예측.

  <img src="../media/ko/streamer-add.avif" alt="Twitch 검색으로 스트리머를 추가하고 옵션을 여는 모습">

- **스트리머 편집.** ‘편집’을 누르면 그 스트리머의 옵션이 열려요: 보너스 상자, 레이드, 모먼트, 시청 스트릭, 채팅, 예측… 스위치 하나로 목록에서 빼지 않고도 켜고 끌 수 있어요.

  <img src="../media/ko/streamer-edit.avif" alt="스트리머 행에서 연 옵션">

- **여러 스트리머를 한 번에 편집.** 체크한 다음 ‘선택한 스트리머 N명 편집’을 누르세요. 바꾼 옵션만 적용돼요. 스트리머마다 값이 다른 옵션은 ‘혼합’으로 표시되고, 스위치가 ‘예’와 ‘아니요’ 두 버튼으로 바뀌어요. 그다음 앱이 이 값을 새 스트리머의 기본 옵션으로 삼을지 물어봐요.

  <img src="../media/ko/streamers-group-edit.avif" alt="‘혼합’ 옵션이 있는 여러 스트리머 편집">

- **스트리머 가져오기.** 목록을 붙여 넣거나(최대 500명), 다른 계정의 목록이나 이 계정이 팔로우하는 채널을 그대로 가져와요. 목록을 확인해서 실제로 있는 채널만 추가해요. 실시간 업데이트는 계정당 스트리머 46명까지 받아요. 목록에서 그다음 스트리머는 레이드, 모먼트, 예측을 받지 못해요.

  <img src="../media/ko/streamers-import.avif" alt="스트리머 가져오기: 붙여 넣은 목록, 다른 계정, 팔로우한 채널">

- **Spam Points.** 채널의 사용자 지정 보상을 여러 번 연속으로 교환해요. 포인트 목록에서 채널을 고르거나 Twitch 검색으로 찾은 아무 채널이나 고르면, 그 채널의 보상이 계정 잔액과 함께 표시돼요. 잔액은 실시간으로 다시 읽어요. 수량을 정하고(최대 25개, 잔액 범위 안에서) ‘교환’을 누르세요. 연속 교환이 끝나면 결과가 표시되고, 연속 교환마다 ‘일지’에 한 줄씩 남아요.

  <img src="../media/ko/spam-points.avif" alt="Spam Points: 채널을 고르고 메시지를 넣는 보상을 10번 교환하자 교환 내역이 채널의 Twitch 채팅에 올라오고, 일지에 남은 줄을 보는 모습">

### Drops

- **진행 중인 모든 Drops 캠페인.** 캠페인마다 상세 정보가 있어요: 보상, 퍼블리셔 계정 연동, ‘먼저 마이닝’. 목록은 [Drops Alerts](https://dropsalerts.com) 프로젝트에서 가져오고, 5분마다 새로 고쳐져요.

  <img src="../media/ko/drops-catalogue.avif" alt="달력 형태의 카탈로그, 검색, 캠페인 상세 정보">

- **내가 정하는 마이닝 순서.** 캠페인을 원하는 순서로 끌어다 놓으세요. 새 캠페인은 알아서 자리를 잡아요. 우선 카테고리가 먼저, 그다음은 가장 빨리 끝나거나 남은 시간(분)이 가장 적은 캠페인 순이에요. 카테고리, 단어, 캠페인, 개별 보상을 제외할 수 있고, 기한 안에 끝낼 수 없는 캠페인은 건너뛸 수 있어요. 캠페인 상세 정보에서 ‘먼저 마이닝’을 누르면 맨 위로 올라가요.

  <img src="../media/ko/drops-order.avif" alt="마이닝 순서, 우선 카테고리, 제외 항목">

- **인벤토리와 ‘모두 받기’.** ‘인벤토리’ 페이지는 보상을 ‘받을 수 있음’, ‘진행 중’, ‘수령함’으로 나눠 보여 줘요. 하나씩 받거나, 한 번에 모두 받거나, ‘드롭 자동 수령’을 켤 수 있어요. ‘퍼블리셔 계정 연동’ 후에 앱으로 돌아오면 계정을 바로 다시 읽어서, 연동이 곧바로 보여요.

  <img src="../media/ko/drops-inventory.avif" alt="인벤토리와 ‘모두 받기’ 버튼">

- **‘Drops와 배지에서 내 스트리머 우선’.** 현재 캠페인에서, 내 목록에 있는 스트리머의 방송이 그 캠페인에 포함되면 다른 채널보다 먼저 시청해요. 같은 채널에서 드롭도 진행되고 그 채널의 채널 포인트도 쌓여요. 기본으로 켜져 있어요.

  <img src="../media/ko/drops-streamers-first.avif" alt="목록의 스트리머를 Drops와 그 채널의 포인트를 위해 시청하는 모습">

- **보상 우선순위.** 두 캠페인의 순위가 같으면 마이닝 도구는 선호하는 종류(배지, 이모트, 게임 내 아이템)의 보상이 있는 캠페인을 먼저 골라요. 정해 둔 마이닝 순서를 덮어쓰는 일은 없어요.

- **배지(베타).** 배지 페이지에는 Twitch의 모든 글로벌 채팅 배지와 얻는 방법이 나와요. 계정마다 배지 페이지가 있어서 진행 중인 배지와 진행률, 아직 얻을 수 있는 배지, 이미 가진 배지를 보여 줘요. 마이닝 도구는 Drops 캠페인과 Twitch 퀘스트의 배지를 노리고, 계정이 이미 가진 배지는 건너뛰어요. 매일 시청해야 하는 배지는 마지막 날까지 날마다 다시 마이닝하고(‘하루 20분, 3일’), 캠페인의 어느 카테고리 채널이든 인정돼요. 상자는 ‘N개 중 무작위 배지 1개’를 줘요. 구독이 필요한 배지는 캠페인 기간에 카테고리 채널에서 직접 구독하거나 구독을 선물하고 Twitch가 그 구독을 인정한 뒤에 마이닝해요. 이미 진행 중인 구독, Prime 구독, 갱신은 인정되지 않아요. 캠페인이 시작되기 전에는 아무것도 마이닝하지 않아요. ‘마이닝할 대상과 순서’에서 우선 카테고리, 배지, 기타 Drops 캠페인을 원하는 순서로 끌어 놓거나 하나를 끌 수 있어요. Twitch 퀘스트는 Twitch가 Drops 패널에서 그 퀘스트를 제공하는 채널에서만 마이닝해요. 진화(Pichu → Pikachu)는 계정이 진화 전 배지를 갖게 되면 마이닝해요. 마이닝 도구가 바로 전에 그 배지를 직접 장착하고, 이전 배지로 되돌리지 않아요. 계정의 배지 설정에 있는 ‘배지 진화 마이닝’으로 끌 수 있어요. 배지를 클릭하면 세부 정보가 열려요: 전체 조건, 모든 카테고리와 채널, 컬렉션, 진화, 계정별 진행률. 이 기능은 베타예요.

  <img src="../media/ko/badges.avif" alt="배지 페이지, 한 계정의 배지, 그리고 배지를 맨 앞으로 끌어 놓은 ‘마이닝할 대상과 순서’">

### 기록

- **모든 일과 모든 요청을 기록.** ‘일지’에는 모든 이벤트가 남아요: 드롭, 얻고 쓴 포인트, 상자, 베팅, 레이드, 방송 시작… 계정, 탭(Drops, 배지, 포인트, 기타), 유형별로 필터링할 수 있어요. **기술 로그 › 요청**에는 Twitch와 다른 서비스로 보낸 모든 요청이 결과와 걸린 시간과 함께 표시돼요. 로그와 요청에는 토큰이나 비밀 정보가 전혀 없어요. ‘버그 리포트용으로 복사’로 복사해서 [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues)나 [Discord](https://discord.gg/support-bots-891685895781113886)의 버그 리포트에 바로 붙여 넣으면 돼요.

  <img src="../media/ko/journal-requests.avif" alt="필터링한 일지와 기술 로그 › 요청">

- **통계 페이지.** 얻은 포인트, 받은 드롭, 시청 시간, 적중한 예측을 7일, 30일, 90일 단위로 보여주고, 각 숫자 뒤에 있는 1위 스트리머나 카테고리도 함께 표시해요. 스트리머별 포인트 곡선은 시간에 따른 잔액을 보여주며, 스트릭, 레이드, 모먼트, 예측에 표시가 붙고 각각에 툴팁이 떠요.

  <img src="../media/ko/stats.avif" alt="30일 동안의 통계 타일, 그다음 표시가 있는 스트리머의 포인트 곡선">

- **일시 중지와 매일 일시 중지.** 일시 중지 버튼 하나로 계정 하나 또는 모든 계정을 일시 중지할 수 있어요. 재개할 때까지, 또는 30분, 1시간, 4시간, 내일 09:00까지, 또는 원하는 날짜와 시간으로 지정할 수 있고, 일시 중지 중에는 같은 버튼이 ‘재개’로 바뀌어요. 배지에는 시간이 정해진 일시 중지가 끝나는 시각이 표시되고, 사이드바의 각 계정에는 실행 중, 일시 중지됨, 확인이 필요한 문제 같은 상태가 표시돼요. 매일 일시 중지는 선택한 시간대에서 매일 두 시각 사이에 모든 계정의 마이닝을 멈추고, 끝나면 자동으로 다시 시작하며 상단 바에 전용 배지가 표시돼요.
- **CSV 내보내기와 카테고리별 개수.** ‘일지’는 받은 드롭을 CSV 파일(날짜, 계정, 카테고리, 캠페인, 드롭, 분)로 계정별로 필터링해 내보낼 수 있고, 대화 상자에는 카테고리별로 받은 드롭 개수가 나와요. 데스크톱 앱은 저장 대화 상자로 파일을 저장하고, 웹 대시보드는 파일을 다운로드해요.

### 데스크톱이든 서버든

- **어디서나 같은 인터페이스.** 데스크톱 앱(Linux, Windows, macOS)과 Docker 이미지는 같은 대시보드를 보여 줘요. Docker는 VPS, NAS, Raspberry Pi에서 돌아가고, 그 기기에는 화면도 브라우저도 필요 없어요. 대시보드는 어떤 브라우저에서든 열려요.
  휴대폰에서는 대시보드가 화면에 맞춰지고, HTTPS로 열면 앱처럼 설치할 수 있어요.
- **가벼움.** 계정당 메모리 약 3 MB, CPU는 거의 쓰지 않고, Docker 이미지는 약 12 MB예요.

### 보안과 업데이트

- **세션을 안전하게.** Twitch 비밀번호는 필요 없어요. 내 브라우저에서 `twitch.tv/activate`로 로그인해요. 세션은 디스크에 암호화되어 저장되고, [Twitch 설정](https://www.twitch.tv/settings/connections)에서 해지할 수 있어요. 데스크톱에서는 선택 사항인 앱 비밀번호로 앱을 잠글 수 있고, 비밀번호를 입력하기 전에는 아무것도 마이닝하지 않아요. Docker에서는 대시보드에 별도의 비밀번호가 있고, 몇 분 동안 활동이 없으면 스스로 잠기며, 프록시 뒤에서 HTTPS로 동작해요. 비밀번호를 연속 5번 틀리면 그 IP 주소는 30초 동안 차단돼요.

  <img src="../media/ko/security-lock.avif" alt="자물쇠: 잠긴 앱을 비밀번호로 잠금 해제하는 모습">

- **서명된 업데이트, 자동 설치는 없어요.** 상단 바에 버전 번호가 붙은 ‘업데이트’ 배지가 나타나고, 데스크톱 알림으로도 알려줘요. 기본적으로 켜져 있고, **설정 › 알림**의 스위치로 끌 수 있으며, 브라우저에서는 물어볼 때 ‘허용’을 클릭하면 돼요. ‘설치 후 다시 시작’을 누르면 먼저 확인을 요청하고, 그다음 앱이 그 버전을 설치하고 다시 시작해요. 앱 비밀번호를 설정했다면 잠긴 상태로 다시 시작하니, 비밀번호를 다시 입력하면 마이닝이 이어져요. 프로젝트가 서명한 버전만 설치돼요. Docker 대시보드와 systemd 바이너리도 실행할 명령어와 함께 알려 줘요.

  <img src="../media/ko/updates.avif" alt="‘업데이트’ 배지, 확인, 그리고 설치">

### 그 밖에

- 다른 마이닝 도구처럼 지원하는 기능: 보너스 상자, 레이드, 모먼트, 커뮤니티 목표, 예측(전략, 베팅 시점, 조건), 시청 스트릭과 구독 채널 우선, 채팅 접속(끌 수 있음), 새 스트리머의 기본 옵션, 이미지와 각각에 ‘테스트’ 버튼이 있는 Discord, Telegram, 웹훅 알림(서버에서는 알림 언어를 **설정 › 알림**에서 골라요), 전체 또는 계정별 프록시.
- 29개 언어, 다크·라이트 테마, 데스크톱 알림, 트레이 아이콘, 시스템 시작 시 실행.

  <img src="../media/ko/basics.avif" alt="언어, 라이트·다크 테마, 데스크톱 알림, 트레이 아이콘">

- **토큰 목록을 붙여넣어 계정 추가하기.** 이미 다른 곳에서 로그인했나요? 기기 코드 대신 액세스 토큰 목록을 한 줄에 하나씩(최대 50개) 붙여넣으세요. 각 토큰은 즉시 확인되며 ‘확인함: n/N’ 카운터가 표시되고, 실패한 줄은 다시 시도할 수 있어요. 그런 다음 하나의 설정 단계가 전체 목록에 적용돼요. 가져오기는 백그라운드에서 계정별 진행 상황을 보여주며 도중에 취소할 수 있고, 다시 시도하면 실패한 계정만 다시 보내요. 이 방식으로 가져온 계정을 삭제할 때는 Twitch에서 토큰도 취소할지 물어봐요.

> [!WARNING]
> 이 프로젝트는 Twitch와 제휴 관계가 없어요. 계정을 자동화하면 Twitch 약관이나 캠페인 규칙을 위반할 수 있어요.

## 목차

- [비교](#comparison)
- [Docker](#docker)
- [작동 방식](#how-it-works)
- [자주 묻는 질문](#faq)
- [관련 프로젝트](#related-projects)
- [보안](#security)
- [소스에서 빌드](#build)
- [라이선스](#license)

## <a name="comparison"></a>비교

[Twitch Miner](https://github.com/SunkwiBOT/twitch-miner)를 [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)(채널 포인트), [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner)(Drops)와 비교하면 다음과 같아요. 2026년 11월에 각 프로젝트의 코드와 README로 확인했어요.

| | 이 프로젝트 | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops 캠페인: 찾기, 마이닝, 수령 | ✅ | ❌ ¹ | ✅ |
| Twitch 배지: 모든 배지 목록, 계정별 배지, 배지 Drops와 Twitch 퀘스트를 원하는 순서로 마이닝(베타) | ✅ | ❌ | 일부 ² |
| 채널 포인트: 상자, 레이드, 모먼트, 예측 | ✅ | ✅ | ❌ |
| Spam Points: 채널의 사용자 지정 보상을 여러 번 연속으로 교환 | ✅ | ❌ | ❌ |
| 한 계정에서 Drops와 채널 포인트를 동시에 | ✅ | 일부 ¹ | ❌ |
| 앱 하나에 원하는 만큼의 계정 | ✅ | ❌ ³ | ❌ |
| 여러 계정의 설정을 한 번에 변경 | ✅ | ❌ | ❌ |
| 그래픽 인터페이스 | ✅ 데스크톱과 웹 ⁴ | ❌ ⁵ | ✅ 데스크톱 |
| 홈 서버용 웹 대시보드(Docker), 비밀번호 보호, HTTPS로 접속 가능 | ✅ | ❌ ⁵ | ❌ |
| 디스크에 암호화된 세션, 선택형 앱 비밀번호 | ✅ | ❌ | ❌ |
| 29개 언어 인터페이스 | ✅ | ❌ | 일부 ⁶ |
| 앱 안에서 받는 서명된 업데이트(데스크톱) | ✅ | ❌ | ❌ |
| 새 계정마다 설정 마법사 | ✅ | ❌ ⁵ | ❌ |
| 인터페이스에서 스트리머 추가·편집(하나씩 또는 여러 명 한 번에) | ✅ | ❌ ⁵ | — |
| 검색, 목록, 다른 계정, 팔로우한 채널에서 스트리머 가져오기 | ✅ | 일부 ⁷ | — |
| 이미지가 포함된 Discord, Telegram, 웹훅 알림 | ✅ | 일부 ⁸ | ❌ |
| 모든 Drops 캠페인 달력과 검색 | ✅ | ❌ | 일부 ⁹ |
| 캠페인별 마이닝 순서, 제외 단어, 무시한 캠페인이나 보상 | ✅ | ❌ | ❌ ¹⁰ |
| 캠페인 Drops를 위해 내 목록의 스트리머를 먼저 시청 | ✅ | 일부 ¹¹ | ❌ |
| 인터페이스 안에서 필터링할 수 있는 이벤트 일지와 모든 요청 목록 | ✅ | ❌ ⁵ | ❌ |
| 메모리 약 3 MB(계정당, Docker 서버) | ✅ | — ¹² | — ¹² |

¹ 자기 목록의 스트리머만 시청해요. 그 방송의 Drops를 진행하고 수령하지만, 캠페인을 고르지는 않아요.\
² 보상이 배지나 이모티콘인 Drops 캠페인을 마이닝할지 뺄지 정하는 설정이 있어요. Twitch 퀘스트와 배지 목록은 없어요.\
³ 계정마다 프로세스 하나.\
⁴ PC에서 데스크톱 앱으로, 그리고/또는 Docker로 서버(VPS, NAS, Raspberry Pi)에서 실행할 수 있어요. 그 기기에는 그래픽 환경도 브라우저도 필요 없고, 대시보드는 어떤 브라우저에서든 열려요.\
⁵ Python 파일로 설정해요. 웹 페이지는 포인트 차트와 로그 파일을 보여 줘요.\
⁶ 21개 언어.\
⁷ 계정이 팔로우하는 채널을 대상으로 삼을 수 있어요.\
⁸ 이미지 없이 텍스트만 보내요. Matrix, Pushover, Gotify도 지원하는데, 이 프로젝트에는 없어요.\
⁹ 필터가 있는 캠페인 목록으로, 달력이나 검색은 없어요.\
¹⁰ 우선 카테고리와 제외 카테고리가 있고, 카테고리 이름으로 골라요.\
¹¹ 기본적으로 목록의 스트리머 중 방송에 Drops가 걸린 스트리머를 먼저 시청하지만(`DROPS` 우선순위), 다른 채널은 절대 시청하지 않아요.\
¹² 측정하지 않았어요.

## <a name="docker"></a>Docker

서버는 같은 대시보드를 브라우저에서 보여 줘요. 이미지는 `ghcr.io/sunkwibot/twitch-miner`이고, `amd64`와 `arm64`를 지원해요. 새 버전이 나오면 대시보드가 알려 줘요. `docker compose pull`을 실행한 다음 `docker compose up -d`를 실행하세요.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # 시작할 때마다 최신 이미지 받기
    restart: unless-stopped
    ports: ["8080:8080"]   # HTTPS 리버스 프록시 뒤에 둘 때: 이 줄을 지우거나 "127.0.0.1:8080:8080"으로 변경
    environment:
      MINER_PASSWORD: "change-me"   # 필수, 12자 이상
      # MINER_PUBLIC_URL: "https://miner.example.com"   # HTTPS 리버스 프록시 뒤에 둘 때만
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

<http://localhost:8080>(또는 내 네트워크에서 이 기기의 주소)를 열고 비밀번호를 입력하세요.

### 변수

| 변수 | 기본값 | 용도 |
|---|---|---|
| `MINER_PASSWORD` | 없음 | 대시보드 비밀번호. 필수예요. 없으면 서버가 시작을 거부해요. 12자 이상이어야 해요(더 짧으면 시작할 때 경고가 표시돼요). 예시 값 `change-me`는 거부돼요. |
| `MINER_PASSWORD_FILE` | 없음 | `MINER_PASSWORD`를 파일에서 읽어요(예: Docker secret). 둘 중 하나만 설정하세요. |
| `MINER_PUBLIC_URL` | 없음 | HTTPS 리버스 프록시(Caddy, nginx, Traefik, Cloudflare Tunnel 등) 뒤에 둘 때만 써요. 대시보드의 `https://` 주소로, 경로 없이 `https://miner.example.com`처럼 써요. 프록시는 `X-Forwarded-Proto`, `X-Forwarded-For`, 웹소켓을 전달해야 하고(흔히 쓰는 프록시는 기본으로 그렇게 해요), 컨테이너의 포트에 직접 접속할 수 없어야 해요. 그러면 대시보드는 프록시를 거쳐 HTTPS로 들어오지 않은 접속을 모두 거부하고, 비밀번호를 5번 틀린 주소를 30초 동안 차단해요. 프록시는 같은 기기나 그 로컬 네트워크에서 실행해야 해요. 공인 주소에서 온 연결은 거부돼요. |
| `MINER_TRUSTED_HOPS` | `1` | `MINER_PUBLIC_URL`을 쓸 때만: `X-Forwarded-For`에 항목을 추가하는 프록시 수로, 1~10이에요. 프록시 두 개가 앞뒤로 놓여 있으면(예: Traefik 앞에 Cloudflare) `2`예요. |
| `MINER_PROXY_NETWORKS` | 없음 | `MINER_PUBLIC_URL`과 함께일 때만: 프록시가 접속해 오는 네트워크를 쉼표, 공백, 줄바꿈으로 구분해 지정해요. 주소 하나 또는 `192.168.1.0/24`, `2001:db8:1::/64` 같은 네트워크예요. 설정하면 프록시는 루프백 주소와 이 네트워크에서만 받아들이고, 사설 로컬 네트워크 전체에서는 더 이상 받지 않아요. 빈 값, `/0`, 호스트 비트가 설정된 네트워크는 `MINER_AUTH_HEADER`도 설정된 경우에만 시작을 막아요. 설정하지 않으면 경고와 함께 받아들여져요. |
| `MINER_AUTH_HEADER` | 없음 | Forward auth / SSO: 프록시가 로그인한 사용자를 설정하는 헤더 이름이에요: Authelia는 `Remote-User`, Authentik은 `X-authentik-username`, oauth2-proxy는 `X-Auth-Request-User`(`--set-xauthrequest`와 함께)예요. 프록시를 통해 허용된 요청 — `MINER_PROXY_NETWORKS`에 있는 주소에서 오고, `Host`가 `MINER_PUBLIC_URL`과 일치하는 요청 — 에 이 헤더에 비어 있지 않은 줄이 하나 있으면 `MINER_PASSWORD` 없이 로그인돼요. `MINER_PUBLIC_URL`과 명시적으로 설정한 `MINER_PROXY_NETWORKS`가 필요하고, 루프백만으로는 절대 충분하지 않아요. 프록시, 브라우저, 클라이언트 중 하나가 이미 쓰는 헤더(`X-Forwarded-*`, `Host`, `Cookie`, `Authorization`, `X-Real-IP`, `Forwarded`, `Via`, `User-Agent`, `Accept*`, `Sec-Fetch-*`, `Cache-Control`, `CF-Connecting-IP`, `Priority`, `Connection`, `Origin`, `Referer`, `X-Request-Id`, `Content-Type`…)는 거부해요. |
| `MINER_AUTH_USERS` | 없음 | `MINER_AUTH_HEADER`와 함께일 때만: 로그인을 허용할 헤더 값이에요. `MINER_PROXY_NETWORKS`처럼 쉼표, 공백, 줄바꿈으로 구분해요. 대소문자를 구분해요. 설정은 됐지만 값이 비어 있으면 거부돼요. 헤더를 설정한 채로 `MINER_PROXY_NETWORKS`를 비워둔 것과 같아요. 설정하지 않으면 프록시가 설정한 어떤 값이든 로그인되고 — 로그인한 모든 SSO 사용자가 관리자가 되며 — 서버가 시작할 때 이에 대해 경고해요. 설정하면 서버는 시작할 때 인식한 값의 개수를 기록해요. |
| `MINER_VAULT_PASSPHRASE` | 없음 | 앱 비밀번호를 설정한 데스크톱 앱에서 복사한 데이터 폴더에만 필요해요. 시작할 때 잠금을 해제해요. 처음부터 새로 만든 서버에는 필요 없어요. |
| `MINER_VAULT_PASSPHRASE_FILE` | 없음 | `MINER_VAULT_PASSPHRASE`를 파일에서 읽어요. 둘 중 하나만 설정하세요. |
| `DATA_DIR` | 이미지에서는 `/data`, 그 외에는 작업 디렉터리의 `data` | 데이터베이스, 키 파일, 로그. |
| `LOG_DIR` | `$DATA_DIR/logs` | 로그 파일: 하루에 하나씩, 최근 14개 보관. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | 수신 주소: IP 주소와 1~65535 사이의 포트. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | 로그 레벨. |

서버는 시작할 때 변수를 한 번 읽고 검사해요. 값이 잘못되면 조용히 기본값으로 넘어가지 않고, 메시지를 남기며 서버가 멈춰요.

리버스 프록시 뒤에서는 `MINER_PUBLIC_URL`을 설정하세요. 설정하지 않으면 프록시를 거쳐 들어온 첫 요청이 로그에 경고를 남겨요. 이때는 로그인 시도가 프록시 주소 기준으로 세어져서, 방문자 한 명의 잘못된 비밀번호 때문에 모두가 잠기기 때문이에요. 실시간 연결은 30초마다 ping을 보내서, 아무 일도 없을 때 프록시가 연결을 닫지 않게 해요. Twitch나 네트워크에 문제가 생기면 서버는 `424`로 응답하고 `502`나 `504`는 절대 쓰지 않아요. 그래서 Cloudflare 뒤에서도 Cloudflare 페이지 대신 실제 오류를 볼 수 있어요.

#### 신뢰하는 프록시로 로그인하기 (forward auth / SSO)

_v1.16.0부터 사용할 수 있어요._

여러 셀프호스팅 앱(Authelia, Authentik, oauth2-proxy…) 앞에 로그인을 하나만 두는
구성이라면, `MINER_AUTH_HEADER`로 대시보드 자체 비밀번호를 건너뛸 수 있어요. 프록시의
로그인만으로 충분해요. 허용할 값을 `MINER_AUTH_USERS`에도 설정하세요. 그러지 않으면 어떤
SSO 사용자든 관리자로 로그인하게 돼요. `MINER_PASSWORD`는 `POST /api/auth`로 직접
로그인하는 스크립트에는 계속 동작하지만, 브라우저로는 절대 쓸 수 없어요: 아래 라우터는
다른 모든 요청을 `forwardAuth`를 거쳐 보내므로, 대시보드를 여는 브라우저는 항상 먼저
동작하는 SSO가 필요해요. 두 번째 라우터는 `/api/auth`에 대한 `POST`와 `DELETE` —
비밀번호 로그인과 로그아웃 — 에만 반응하고, `forwardAuth`를 빼는 대신 스스로 헤더를
비워요: 이 규칙을 그 한 경로 너머로 넓히면 클라이언트가 위조한 헤더도 거기서
통과해요.

프록시는 요청의 원래 `Host`를 마이너로 가는 길에서 바꾸지 않고 유지해야 해요. 그러지
않으면 `MINER_PUBLIC_URL`이 절대 맞지 않고, 헤더는 조용히 무시되며, 처음 그 일이
일어났을 때 로그에 경고가 남아요: nginx는 `proxy_set_header Host $host;`가, Apache는
`ProxyPreserveHost On`이 필요해요. Traefik과 Caddy는 기본으로 이를 바꾸지 않으므로,
둘 다 추가할 게 없어요.

이 제품들 중 어느 것도 클라이언트가 직접 보낸 헤더를 제거하지 않아요: 프록시에서
마이너로 가는 각 경로마다 이를 덮어쓰는 자체 규칙이 필요해요(Traefik의 `forwardAuth`는
`authResponseHeaders`에 나열된 헤더만 삭제하고, 다른 클라이언트 헤더는 그대로
전달해요). 프록시와 마이너만 전용 네트워크에서 단둘이 실행하고, 프록시에는 고정 주소를
주고, 마이너의 포트는 절대 공개하지 마세요: 다른 무언가 — 다른 컨테이너, 호스트의
프로세스 — 가 거기에 닿을 수 있다면 스스로 헤더를 설정할 수도 있어요.

Traefik 뒤의 Authelia가 `miner-net`에 둘만 있고, 로그인한 사용자를 `Remote-User`로
전달하는 예:

```yaml
networks:
  edge: {}          # 여기서 Traefik과 Authelia가 통신해요
  miner-net:        # 여기엔 Traefik과 마이너만 있어요
    ipam:
      config:
        - subnet: 10.10.0.0/29

services:
  traefik:
    # 평소의 Traefik 설정: 엔트리포인트, 다른 네트워크, 다른 앱
    networks:
      edge: {}
      miner-net:
        ipv4_address: 10.10.0.2

  authelia:
    # 평소의 Authelia 설정
    networks:
      edge: {}
    labels:
      traefik.http.middlewares.authelia.forwardAuth.address: "http://authelia:9091/api/authz/forward-auth"
      traefik.http.middlewares.authelia.forwardAuth.trustForwardHeader: "true"
      traefik.http.middlewares.authelia.forwardAuth.authResponseHeaders: "Remote-User"
      # 아래 스크립트용 라우터를 위해: 인증을 건너뛰는 대신 Remote-User를
      # 비워서, 클라이언트가 위조한 헤더가 이 경로에서도 통하지 않게 해요
      traefik.http.middlewares.blank-remote-user.headers.customrequestheaders.Remote-User: ""

  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    networks:
      miner-net:
        ipv4_address: 10.10.0.3   # 이것도 고정: 무작위로 고른 주소는 충돌할 수 있어요
    # "ports" 없음: 이 네트워크는 10.10.0.2에서 오는 Traefik만 접근할 수 있어요
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
      traefik.http.routers.miner.middlewares: "authelia@docker" # 마이너로 가는 모든 라우터에서 반복하세요
      # 스크립트용, MINER_PASSWORD와 함께: Authelia는 우회하지만 헤더 비우기는 우회하지 않아요
      traefik.http.routers.miner-api.rule: "Host(`miner.example.com`) && Path(`/api/auth`) && (Method(`POST`) || Method(`DELETE`))"
      traefik.http.routers.miner-api.middlewares: "blank-remote-user@docker"

volumes:
  miner-data:
```

#### 다른 프록시 강화하기

이 제품들 중 어느 것도 클라이언트가 보낸 헤더 사본을 스스로 제거하지 않아요: SSO의
응답에서 헤더를 설정하는 지시어 앞에, 스크립트용 경로를 포함해 마이너로 가는 모든
경로에 아래 제거 줄을 추가하세요.

- **Caddy**: `forward_auth` 바로 앞에 `request_header -Remote-User`. 확장형
  (`request_header <이름> {rp.header.<이름>}`)은 게이트웨이 자체 헤더만 추가할 뿐,
  클라이언트의 것을 지우지는 않아요.
- **nginx**: `auth_request` 앞에 `proxy_set_header Remote-User "";`, 그리고
  `proxy_set_header Host $host;`(`auth_request` 자체는 `--with-http_auth_request_module`
  로 빌드해야 해요). nginx는 `underscores_in_headers on;`을 설정하지 않으면 이름에
  밑줄이 있는 헤더도 버려요. `MINER_AUTH_HEADER`가 그런 이름을 거부하는 이유예요.
- **Apache**: 실제 값을 추가하는 `RequestHeader set` 앞에 `RequestHeader unset
  Remote-User`, 그리고 위와 같은 `Host` 요건을 위해 `ProxyPreserveHost On`.
- **HAProxy**: 인증 응답에서 설정하는 규칙 앞에 `http-request del-header Remote-User`.
- **Cloudflare Tunnel**: `httpHostHeader`를 설정하지 않은 채로 두세요(기본적으로 이미
  클라이언트가 요청한 공인 `Host`를 전달해요). 이건 터널이지, 헤더를 정리하는 프록시가
  아니에요. 그러니 `MINER_AUTH_HEADER`를 위해서는 혼자 믿지 말고 위 프록시 중 하나와
  짝지으세요.
- **Tailscale Serve**: `Tailscale-User-Login`. `tailscaled`는 자기 값을 설정하기 전에
  클라이언트가 보낸 사본을 이미 지워요. 직접 정리할 게 없어요. 중요한 건 직접 연결이
  아니라 `tailscale serve`만 마이너에 닿을 수 있어야 한다는 거예요 — `127.0.0.1`에
  묶고 Serve만 중개하게 하세요.

어떤 프록시를 고르든, 신뢰하기 전에 한 번 테스트하세요: SSO에서 로그아웃한 상태로,
마이너 자체 주소에 직접(프록시를 우회해서, 그것이 실행 중인 기기나 컨테이너에서)
헤더를 손으로 설정해 `curl`을 실행하세요. 예를 들어
`curl -H "Remote-User: mallory" http://127.0.0.1:8080/api/snapshot`처럼요. `401`로
응답해야지, 스냅샷을 주면 안 돼요.

5가지 흔한 구성에 대해 테스트를 마친, 그대로 복사해 쓸 수 있는 Compose 예시예요:

| 프록시 | 예시 | 보여주는 것 |
|---|---|---|
| Traefik + Authelia | [`docs/reverse-proxy/traefik-authelia/`](docs/reverse-proxy/traefik-authelia/) | 완전한 싱글 사인온 스택: 실제 Authelia, 파일 백엔드, Traefik을 통한 forward-auth. |
| Caddy | [`docs/reverse-proxy/caddy/`](docs/reverse-proxy/caddy/) | `forward_auth` + `copy_headers`, 다섯 중 가장 짧은 설정. |
| nginx | [`docs/reverse-proxy/nginx/`](docs/reverse-proxy/nginx/) | `auth_request`와, 실수로 빠뜨리기 쉬운 덮어쓰기 줄 — 그리고 그걸 빠뜨리면 무슨 일이 생기는지. |
| Apache httpd | [`docs/reverse-proxy/apache/`](docs/reverse-proxy/apache/) | 내장 forward-auth가 없어서, 대신 Apache와 마이너 사이에 작은 SSO 인식 프록시를 둬요. |
| HAProxy | [`docs/reverse-proxy/haproxy/`](docs/reverse-proxy/haproxy/) | Apache와 같은 패턴이고, `http-request del-header`로 클라이언트 자신의 사본을 지워요. |

### <a name="systemd"></a>Docker 없이(systemd)

Docker 없는 Linux 서버에서는, 릴리스에 대시보드가 들어 있는 단일 파일 서버도 포함돼 있어요: `twitch-miner-server_<버전>_linux_x64` 또는 `twitch-miner-server_<버전>_linux_arm64`예요. Docker 이미지가 실행하는 것과 같은 프로그램이고 정적으로 링크되어 있지만, HTTPS를 위해 읽는 시스템의 CA 인증서는 예외예요. Docker 이미지도 같은 이유로 이를 복사해 둬요. 같은 변수를 사용해요.

```bash
# ARM64: twitch-miner-server_<버전>_linux_arm64
sudo install -m 755 twitch-miner-server_<버전>_linux_x64 /usr/local/bin/twitch-miner
sudo install -m 600 /dev/null /etc/twitch-miner.env
```

root만 읽을 수 있는 `/etc/twitch-miner.env`에 12자 이상의 비밀번호를 직접 적으세요:

```ini
MINER_PASSWORD=change-me
```

그다음 `/etc/systemd/system/twitch-miner.service`에 서비스를 만드세요:

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

systemd가 데이터용으로 `/var/lib/twitch-miner`를 만들고, root가 아니라 자기만의 사용자로 서버를 실행해요. `http://<서버 주소>:8080`을 여세요. 로그는 `journalctl -u twitch-miner`에 있어요.

이 파일은 스스로 업데이트되지 않아요. 대시보드에 새 버전이 표시되면, 릴리스에서 새 파일을 내려받아 예전 파일을 바꾸고 다시 시작하세요:

```bash
# ARM64: twitch-miner-server_<버전>_linux_arm64
sudo install -m 755 twitch-miner-server_<버전>_linux_x64 /usr/local/bin/twitch-miner
sudo systemctl restart twitch-miner
```

## <a name="how-it-works"></a>작동 방식

각 계정은 20초마다 두 채널을 골라요. 먼저 대기열 맨 위 캠페인에 가장 알맞은 라이브 채널을, 그다음 포인트 목록의 스트리머를 골라요. 라이브 채널이 없는 캠페인은 라이브 채널이 있는 다음 캠페인으로 넘어가요(대기열에는 ‘라이브 채널 없음’으로 표시돼요). 라이브 채널은 90초마다 다시 확인하고, 계정은 스트리머 중 한 명이 라이브를 시작하는 즉시 그 캠페인으로 돌아가요.

Twitch는 계정마다 동시에 **Drops 캠페인 하나**와 **포인트용 채널 두 개**만 인정해요. 2026년 10월부터는 퍼블리셔 계정이 연동되어 있어야 드롭을 수령할 수 있어요. 연동되지 않은 캠페인도 진행도는 쌓이고, 대시보드에 연동 링크가 표시돼요.

Twitch 자체 진행 상황이 대기열 맨 위 캠페인에서 실제로 30분 시청해도 움직이지 않으면, 그 캠페인을 인정하지 않는 거예요: 계정은 그 캠페인을 보류하고, 순서상 다음 캠페인이 그 자리를 차지하며, 6시간 후 자동으로 다시 시도돼요 — 그 전에 진행 상황이 다시 나타나면 더 빨리 재시도되지만, 재시도되는 캠페인은 여전히 현재 마이닝 중인 캠페인 뒤에서 기다려야 할 수도 있어요. 어떤 진행이든 생기면 대기 시간이 초기화되고, 이는 모든 종류의 캠페인(Drops, 배지, 퀘스트)에 적용돼요. Twitch가 대신 같은 카테고리의 다른 캠페인을 인정하고 그 캠페인이 끝나거나 수령되면, 6시간을 다 기다리지 않고 보류가 바로 해제돼요. 앱이나 서버를 재시작하면 보류된 캠페인이 모두 초기화돼요: 막혀 있던 캠페인은 그냥 새로운 30분부터 다시 시작해요.

캠페인 목록은 [Drops Alerts](https://dropsalerts.com) 프로젝트에서 가져와요(5분마다 갱신). Twitch가 TV 클라이언트에는 이 목록을 숨기기 때문이에요.

## <a name="faq"></a>자주 묻는 질문

### 위험과 Twitch

**계정이 정지될 수 있나요?**\
그렇지 않다고 장담할 수 있는 사람은 없어요. 이 프로젝트는 Twitch와 제휴 관계가 없고, 계정을 자동화하면 Twitch 약관이나 캠페인 규칙을 위반할 수 있어요. 마이닝 도구는 계정당 최대 두 채널(Twitch의 한도)만 시청하고, 영상은 전혀 내려받지 않아요. 사용 여부는 스스로 판단하세요.

**영상을 정말로 시청하나요?**\
아니요, 영상은 내려받지 않아요. Drops의 경우, 마이닝 도구가 15초마다 방송의 재생 목록을 가장 낮은 화질로 읽고, 새 세그먼트마다 헤더만 요청해요. 포인트의 경우, Twitch의 ‘1분 시청’ 신호를 1분마다 보내요. 시청 중인 채널마다 요청한 세그먼트 수가 표시돼요.

**왜 최대 두 채널인가요?**\
Twitch의 한도라서 그래요. Twitch는 계정마다 동시에 시청하는 채널 두 개까지만, 그리고 Drops 캠페인은 한 번에 하나만 인정해요. **Drops와 포인트** 모드에서는 한 채널이 캠페인을, 다른 채널이 내 스트리머를 맡아요. **계정 설정**의 ‘채널 하나’ 설정을 쓰면 더 조용하게 동작해요.

### Drops

그 계정의 **Drops** 페이지를 여세요. ‘마이닝 순서’에서 대기 중인 캠페인마다 이유가 표시돼요: ‘라이브 채널 없음’ 또는 ‘대기 중 (한 번에 한 캠페인)’. ‘Twitch가 집계하지 않음 — … 재시도’는 실제로 30분 시청해도 Twitch 쪽에서 진행이 없었다는 뜻이에요: 캠페인은 보류되고 표시된 시간 안에 자동으로 다시 시도돼요. 그 전에 진행 상황이 다시 나타나면 더 빨리 재시도돼요. 이 목록에 없는 캠페인은 제외된 거예요. ‘마이닝할 대상과 순서’, ‘어떤 캠페인’ 설정, **제외** 탭의 규칙, 기한 안에 끝낼 수 없다는 이유, 또는 남은 보상이 모두 구독을 필요로 한다는 이유 때문이에요. 구독이 필요한 보상은 캠페인 기간에 카테고리 채널에서 직접 구독하거나 구독을 선물할 때까지 기다리고(이미 진행 중인 구독, Prime 구독, 갱신은 인정되지 않아요), 매일 시청해야 하는 배지는 다음 날을 기다리며, 캠페인이 시작되기 전에는 아무것도 마이닝하지 않아요. 다른 드롭에 이어지는 드롭은 앞의 드롭이 끝날 때까지 기다려요. **포인트만** 모드에서는 마이닝 도구가 캠페인을 고르지 않아요. 캠페인 상세 정보에서 ‘먼저 마이닝’을 누르면 맨 위로 올라가고, 무시한 캠페인은 **제외** 탭의 ‘다시 마이닝’으로 되돌릴 수 있어요. **캠페인** 탭의 **막힘** 그룹에서도 같은 이유를 볼 수 있어요.

**퍼블리셔 계정을 연동해야 하나요?**\
수령하려면 그래요. 2026년 10월부터 Twitch는 퍼블리셔 계정이 연동되어 있을 때만 드롭을 수령할 수 있게 해요. 기본적으로 연동되지 않은 캠페인도 진행도는 쌓이고, 수령은 연동할 때까지 기다려요. ‘연동 안 됨’ 배지로 알 수 있고, 캠페인 상세 정보의 ‘퍼블리셔 계정 연동’을 누르면 연동 페이지가 열려요. 연동된 카테고리만 마이닝하려면 ‘퍼블리셔 계정이 연동된 카테고리만’을 고르세요.

### 설치와 데이터

**데스크톱 앱과 Docker 중 무엇이 좋을까요?**\
데스크톱 앱은 PC가 켜져 있는 동안 마이닝하고, 트레이 아이콘, 선택형 비밀번호, 클릭 한 번 업데이트를 갖추고 있어요. Docker는 항상 켜져 있는 기기(VPS, NAS, Raspberry Pi)에서 밤낮없이 마이닝하고, 브라우저로 열며 `MINER_PASSWORD`로 보호해요. 인터페이스는 같아요.

**데이터는 어디에 있나요?**\
데이터 폴더에 있어요. Linux는 `~/.local/share/twitch-miner`, Windows는 `%APPDATA%\twitch-miner`, macOS는 `~/Library/Application Support/twitch-miner`, Docker는 `/data` 볼륨이에요. 데스크톱에서는 **설정 › 데스크톱**에서 클릭 한 번으로 열 수 있어요. 안에는 `miner.db`(계정, 설정, 일지), `master.key`(앱 비밀번호를 설정하지 않았을 때의 세션 키), 로그가 있어요.

**업데이트는 어떻게 하나요?**\
데스크톱 앱은 시작할 때 새 버전이 있는지 확인하고, 그 뒤로는 한 시간마다 확인해요(**설정 › 정보**에서 바꿀 수 있고, 찾으면 알려 줘요). ‘업데이트’ 배지를 누른 다음 ‘설치 후 다시 시작’을 누르세요. Docker에서는 대시보드가 알려 줘요. `docker compose pull`을 실행한 다음 `docker compose up -d`를 실행하세요. Docker 없이도 대시보드가 똑같이 알려 줘요. 파일 교체와 서비스 재시작은 [Docker 없이](#systemd)를 참고하세요. 앱은 지금 버전 이후의 모든 버전에서 바뀐 점을 최신순으로 보여 줘요. 각 버전의 노트는 GitHub 릴리스 페이지에서도 볼 수 있어요.

**앱 비밀번호를 잊어버렸어요.**\
복구할 수 없어요. Twitch 세션을 암호화하는 것이 바로 이 비밀번호이기 때문이에요. 앱을 종료하고 데이터 폴더를 삭제한 뒤 계정을 다시 추가하세요. 계정 설정도 다시 해야 해요. Docker에서는 대시보드 비밀번호를 `MINER_PASSWORD`에서 바꾼 다음 `docker compose up -d`를 실행하세요.

### 포인트와 예측

**한 시간에 포인트를 얼마나 얻나요?**\
Twitch와 채널마다 달라요. 마이닝 도구는 Twitch가 시청자에게 주는 만큼 얻어요: 시청 시간 포인트, 보너스 상자(약 15분마다 하나), 시청 스트릭 보너스. 구독하면 티어에 따라 포인트가 ×1.2에서 ×2까지 늘어나고, 기본으로 켜져 있는 ‘구독 채널 우선’ 덕분에 그 채널들이 목록의 나머지보다 먼저 시청돼요. 얻은 포인트는 홈에, 하나하나의 획득 내역은 일지에 표시돼요.

**베팅은 위험한가요?**\
네, 포인트를 잃을 수 있어요. 예측은 기본으로 꺼져 있고, 스트리머마다 따로 설정해요. 전략, 베팅 시점, 베팅액(기본값은 잔액의 5%, 최대 50,000포인트), 최소 잔액을 정하고, 원하면 배당이 1.5 이상일 때처럼 조건을 걸 수 있어요. 베팅액은 작게 유지하세요. 10포인트 미만의 베팅은 하지 않아요.

**스트리머가 시청되지 않아요.**\
켜져 있고 라이브 중인 스트리머만, 그것도 비어 있는 채널이 있을 때만 시청해요. **Drops와 포인트** 모드에서는 두 채널 중 하나가 현재 캠페인을 맡고, **Drops만** 모드에서는 스트리머를 시청하지 않으며, 일시정지된 계정은 아무것도 시청하지 않아요. 채널 자리는 다음 순서로 배정되고, 시청 중인 채널마다 배지로 표시돼요: ‘스트릭 위험’, ‘스트릭 보너스’, ‘구독 중’, 그다음 ‘목록 순서’. 그 스트리머를 목록에서 위로 올리거나, ‘시청 스트릭 우선’과 ‘구독 채널 우선’을 끄세요.

## <a name="related-projects"></a>관련 프로젝트

같은 제작자의 다른 프로젝트:

- [**Drops Alerts**](https://dropsalerts.com): 진행 중인 모든 Twitch Drops와 보상 캠페인을 담은 무료 공개 API로, 매분 갱신돼요. 이 앱은 캠페인 카탈로그를 여기서 읽어요.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): 내 서버에 Twitch 라이브와 Drops 캠페인을 알려 주는 Discord 봇이에요.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): 내 서버에 Kick 라이브와 Drops 캠페인을 알려 주는 Discord 봇이에요.

## <a name="security"></a>보안

- 토큰은 AES-256-GCM으로 봉인돼요. 키는 데이터베이스 옆에 있는 무작위 `master.key`(기본값, 따로 할 일 없음)이거나, 앱 비밀번호에서 Argon2id로 유도한 키예요.
- 데이터 폴더와 로그 폴더는 비공개(`0700`)라서, 마이닝 도구를 실행하는 사용자만 열 수 있어요.
- HTTP 대시보드는 상수 시간 비밀번호 비교와 HttpOnly, SameSite=Strict 쿠키(HTTPS 뒤에서는 `Secure`, `__Host-` 접두사 포함)를 사용해요. 서버는 각 세션의 SHA-256만 저장해요. 세션은 30일이 지나거나 로그아웃하면 끝나요. 서버는 요청 본문을 읽기 전에 세션을 확인하고, 본문 크기를 256 KiB로 제한해요.
- 한 IP 주소에서 비밀번호를 연속 5번 틀리면(로그인 폼 또는 bearer) 그 주소는 30초 동안 기다려야 해요. 서버는 비밀번호를 확인하지 않고 `Retry-After`와 함께 `429`를 응답해요. 비밀번호가 틀릴 때마다 응답에 0.5초가 걸리기도 해요. 같은 연결 주소(공유 프록시 포함)에서 1분에 30번 넘게 비밀번호가 틀리면, 그 주소는 2초에 한 번만 시도할 수 있어요. 추측은 느려지고, 아무도 잠기지 않아요.
- 내 네트워크 밖에서는 `MINER_PUBLIC_URL`을 설정하고 대시보드를 HTTPS 리버스 프록시 뒤에 두세요([Docker](#docker) 참고).
- 마이닝 도구는 `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`, `dropsalerts.com`, 그리고 직접 설정한 알림 대상하고만 통신해요. 앱(데스크톱과 Docker)은 `github.com`과 `api.github.com`에서 새 버전과 그 노트를 확인하고, 데스크톱 앱은 프로젝트의 GitHub releases에서 새 버전을 내려받아요. 새 버전은 서명되어 있어서, 화면에 표시된 버전만 설치돼요.

## <a name="build"></a>소스에서 빌드

**사전 요구 사항:** Rust, Node.js, 그리고 내 시스템에 맞는 [Tauri 사전 요구 사항](https://v2.tauri.app/start/prerequisites/).

```bash
npm ci
npm run build          # 대시보드
cargo build --release -p miner-server   # 서버: target/release/twitch-miner
npm run bundle:linux   # 데스크톱 번들(Tauri Linux 의존성 필요)
```

코드 검사:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint(타입 인식), svelte-check, inlang i18n 검사
```

인터페이스의 TypeScript 타입은 [ts-rs](https://github.com/Aleph-Alpha/ts-rs)로 Rust 타입에서 `ui/src/lib/bindings`에 생성돼요. 인터페이스가 쓰는 타입을 바꾼 뒤에는 `npm run bindings`를 실행하세요.

## <a name="license"></a>라이선스

[MIT](../../LICENSE). [DevilXD의 TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner), [Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)와 그 포크들에서 영감을 받았어요.

<img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-miner" width="1" height="1" alt="">
