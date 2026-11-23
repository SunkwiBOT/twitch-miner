<p align="center">
  <img src="../../ui/public/app-mark.svg" width="112" height="112" alt="Twitch Miner logosu">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Twitch Drops, rozet ve kanal puanlarını istediğiniz kadar hesapta, tarayıcı ve açık bir Twitch sekmesi olmadan kazar.</strong>
</p>

<p align="center">
  Linux, Windows ve macOS için bir masaüstü uygulaması ya da ev sunucusunda veya Raspberry Pi üzerinde çalıştırmak için aynı web paneline sahip bir Docker imajı. Her hesap yaklaşık 3 MB bellek kullanır ve işlemciyi neredeyse hiç yormaz.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Son sürüm"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="İndirmeler"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Platformlar: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="amd64 ve arm64 için Docker imajı"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT lisansı"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Görüntülenme">
</p>

<p align="center">
  <a href="#download"><strong>İndir</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Hata bildir</a>
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
  <strong>Türkçe</strong> ·
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
  <img src="../media/tr/presentation.avif" alt="Twitch Miner'ın video tanıtımı">
</p>

## <a name="features"></a>Özellikler

### Hesaplar ve yayıncılar

- **Tek uygulamada istediğiniz kadar hesap.** Her hesap kendi başına, kendi modunda kazar: **Drops ve puan** (bir kanal en iyi kampanya için, diğeri yayıncılarınız için), **Yalnızca Drops** veya **Yalnızca puan**.

  <img src="../media/tr/accounts-mining.avif" alt="Ana sayfada aynı anda madencilik yapan birkaç hesap">

- **Her yeni hesap için bir sihirbaz.** Giriş yaptıktan sonra varsayılan ayarlardan ya da başka bir hesabın ayarlarından başlayın. Modu, kampanyaları ve yayıncıları seçin, ardından “Kazmaya başla” veya “Kaydet ve sonra başlat” düğmesine basın.

  <img src="../media/tr/account-setup.avif" alt="Yeni bir hesabın kurulum sihirbazı, adım adım">

- **Tek tıkla başka hesaplara da uygulanan bir değişiklik.** Bir hesapta bir şeyi değiştirdiğinizde onay mesajı “Diğer hesaplara uygula” seçeneğini sunar. Değişiklik, işaretlediğiniz hesaplarda da yapılır; bu hesapların diğer ayarları olduğu gibi kalır.

  <img src="../media/tr/apply-to-accounts.avif" alt="“Diğer hesaplara uygula” ile başka hesaplarda da yapılan bir değişiklik">

- **Yayıncı ekleme.** Yayıncı varsayılan seçeneklerle hemen başlar: uygulamanın seçenekleriyle ya da bu hesap için seçtiklerinizle. Gerekirse onları hemen orada ayarlayın: bonus sandıkları, baskınlar, anlar, izleme serisi, sohbet, tahminler.

  <img src="../media/tr/streamer-add.avif" alt="Twitch aramasıyla bir yayıncı ekleme, ardından seçenekleri">

- **Yayıncı düzenleme.** “Düzenle” yayıncının seçeneklerini açar: bonus sandıkları, baskınlar, anlar, izleme serisi, sohbet, tahminler… Bir anahtar, yayıncıyı listeden çıkarmadan açar veya kapatır.

  <img src="../media/tr/streamer-edit.avif" alt="Bir yayıncının kendi satırından açılan seçenekleri">

- **Birkaç yayıncıyı aynı anda düzenleme.** Yayıncıları işaretleyin, ardından “Seçili N yayıncıyı düzenle” düğmesine basın. Yalnızca değiştirdiğiniz seçenekler uygulanır. Aralarında farklı olan bir seçenek “Karışık” olarak görünür ve anahtarı “Evet” ve “Hayır” adlı iki düğmeye dönüşür. Uygulama ardından bunları yeni yayıncılar için varsayılan seçenekler yapmayı önerir.

  <img src="../media/tr/streamers-group-edit.avif" alt="“Karışık” seçeneklerle birkaç yayıncıyı düzenleme">

- **Yayıncıları içe aktarma.** Bir liste yapıştırın (en fazla 500 ad) ya da başka bir hesabın listesini veya hesabın takip ettiği kanalları kullanın. Liste denetlenir, böylece yalnızca gerçekten var olan kanallar eklenir. Canlı güncellemeler hesap başına 46 yayıncıyı kapsar: listedeki sonraki yayıncılar baskın, an ve tahmin almaz.

  <img src="../media/tr/streamers-import.avif" alt="Yayıncıları içe aktarma: yapıştırılan liste, başka bir hesap, takip edilen kanallar">

- **Spam Points.** Bir kanalın özel ödülünü art arda birkaç kez kullanın. Puan listenizden bir kanal ya da Twitch aramasıyla bulunan herhangi bir kanal seçin: ödülleri, canlı olarak yeniden okunan hesap bakiyesiyle birlikte görünür. Adedi ayarlayın (bakiyenin yettiği kadar, en fazla 25), ardından “Kullan”. Sonuç seri bittiğinde görünür ve her serinin Günlük'te kendi satırı olur.

  <img src="../media/tr/spam-points.avif" alt="Spam Points: seçilen bir kanal, 10 × kullanılan mesajlı bir ödül, kullanımlar kanalın Twitch sohbetine düşüyor, ardından Günlük satırı">

### Drops

- **Süren tüm Drops kampanyaları.** Her kampanyanın kendi ayrıntıları vardır: ödüller, yayıncı hesabı bağlama, “Önce kaz”. Liste [Twitch Drops API](https://drops.sunkwi.com) projesinden gelir ve 5 dakikada bir yenilenir.

  <img src="../media/tr/drops-catalogue.avif" alt="Takvim görünümündeki Katalog, arama ve bir kampanyanın ayrıntıları">

- **Kendi kazma sıranız.** Kampanyaları istediğiniz sıraya sürükleyin. Yeni kampanyalar yerlerini kendileri bulur: önce öncelikli kategorileriniz, ardından en erken bitenler ya da en az dakikası kalanlar. Kategorileri, kelimeleri, bir kampanyayı veya tek bir ödülü hariç tutun ve zamanında bitemeyecek kampanyaları atlayın. Bir kampanyanın ayrıntılarındaki “Önce kaz”, onu en başa taşır.

  <img src="../media/tr/drops-order.avif" alt="Kazma sırası, öncelikli kategoriler ve hariç tutmalar">

- **Envanter ve “Tümünü al”.** Envanter sayfası ödülleri “Alınacaklar”, “Devam edenler” ve “Alınanlar” olarak ayırır. Ödülleri tek tek ya da hepsini birden alın veya “Drops'u otomatik al” seçeneğini açın. “Yayıncı hesabını bağla”dan sonra uygulamaya döner dönmez hesap yeniden okunur, böylece bağlantı hemen görünür.

  <img src="../media/tr/drops-inventory.avif" alt="Envanter ve “Tümünü al” düğmesi">

- **“Drops ve rozetler için önce benim yayıncılarım”.** Geçerli kampanyada, listenizdeki yayıncılardan yayını o kampanyaya sayılan biri diğer kanallardan önce gelir. Aynı kanal hem drop'u ilerletir hem de size kanal puanlarını kazandırır. Varsayılan olarak açıktır.

  <img src="../media/tr/drops-streamers-first.avif" alt="Listeden bir yayıncı, hem Drops hem de puanları için izleniyor">

- **Ödül önceliği.** İki kampanya eşit sıradaysa madenci önce ödülleri tercih ettiğiniz türe (rozet, emote, oyun içi eşya) uyan kampanyayı seçer. Madencilik sıranızı asla geçersiz kılmaz.

- **Rozetler (beta).** Rozetler sayfası, Twitch'in tüm genel sohbet rozetlerini ve nasıl alınacaklarını listeler. Her hesabın kendi Rozetler sayfası vardır: ilerlemeleriyle birlikte devam eden rozetler, hâlâ alınabilecekler ve hesabın sahip olduğu rozetler. Madenci, Drops kampanyalarının ve Twitch görevlerinin rozetlerinin peşine düşer ve hesabın zaten sahip olduğu rozetleri atlar. Her gün izlenmesi gereken bir rozet, sonuncusuna kadar her gün yeniden kazılır (“Günde 20 dk., 3 gün”) ve kampanyanın kategorilerinden herhangi birindeki bir kanal sayılır. Bir sandık “N rozetten rastgele 1 tanesi” verir. Abonelik gerektiren bir rozet, kampanya sırasında kategorideki bir kanala abone olduktan ya da abonelik hediye ettikten ve Twitch bunu saydıktan sonra kazılır. Devam eden bir abonelik, Prime aboneliği ya da yenileme sayılmaz. Kampanyası başlamadan hiçbir şey kazılmaz. “Ne kazılacak, sırayla” bölümünde öncelikli kategorileri, rozetleri ve diğer Drops kampanyalarını istediğiniz sıraya sürükleyin ya da birini kapatın. Bir Twitch görevi yalnızca Twitch'in onu Drops panelinde sunduğu bir kanalda kazılır. Bir evrim (Pichu → Pikachu), hesap evrimleştiği rozete sahip olur olmaz kazılır: madenci o rozeti hemen önce kendisi takar ve eskisini geri takmaz; hesabın rozet ayarlarındaki “Rozet evrimlerini kaz” bunu kapatır. Bir rozete tıklamak ayrıntılarını açar: tüm gereksinim, bütün kategoriler ve kanallar, koleksiyon, evrim ve her hesabın ilerlemesi. Bu özellik beta aşamasındadır.

  <img src="../media/tr/badges.avif" alt="Rozetler sayfası, bir hesabın rozetleri, ardından Rozetler'in en başa sürüklendiği “Ne kazılacak, sırayla”">

### Takip

- **Her şeyin günlüğü ve her istek.** Günlük her olayı tutar: drop'lar, kazanılan ve harcanan puanlar, sandıklar, bahisler, baskınlar, başlayan yayınlar… Onu hesaba, sekmeye (Drops, Rozetler, Puan, Diğer) ve türe göre filtreleyin. **Teknik kayıtlar › İstekler**, Twitch'e ve diğer hizmetlere gönderilen her isteği sonucu ve süresiyle birlikte listeler. Kayıtlar ve istekler hiçbir token veya gizli bilgi içermez: “Hata raporu için kopyala” onları kopyalar; [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) veya [Discord](https://discord.gg/support-bots-891685895781113886) üzerinde bir hata raporuna yapıştırmaya hazırdır.

  <img src="../media/tr/journal-requests.avif" alt="Filtrelenmiş Günlük, ardından Teknik kayıtlar › İstekler">

- **Bir İstatistikler sayfası.** 7, 30 veya 90 gün boyunca kazanılan puanlar, alınan drop'lar, izlenen süre ve kazanılan bahisler, her sayının arkasındaki en iyi yayıncı veya kategoriyle birlikte. Yayıncı başına bir puan eğrisi, bakiyeyi zaman içinde gösterir; seri, baskın, an ve bahisler için işaretleyicilerle ve her birinde bir araç ipucuyla birlikte.

  <img src="../media/tr/stats.avif" alt="30 günlük İstatistikler kutucukları, ardından bir işaretleyiciyle yayıncının puan eğrisi">

- **Duraklatma ve günlük duraklatma.** Tek bir duraklatma düğmesi, bir hesap veya tüm hesaplar için: siz devam ettirene kadar duraklatın, ya da 30 dakika, 1 saat, 4 saat, yarın 09:00'a kadar veya özel bir tarih ve saate kadar; duraklatılmışken aynı düğme “Devam et” yazar. Bir rozet süreli duraklatmanın ne zaman biteceğini gösterir ve kenar çubuğundaki her hesap durumunu gösterir: çalışıyor, duraklatıldı veya bakılması gereken bir sorun. Günlük duraklatma, her gün iki saat arasında, seçtiğiniz saat diliminde tüm hesapların madenciliğini durdurur ve süre bitince üst çubuktaki kendi rozetiyle birlikte devam ettirir.
- **CSV dışa aktarma ve kategoriye göre sayılar.** Günlük, alınan drop'ları bir CSV dosyası olarak dışa aktarır (tarih, hesap, kategori, kampanya, drop, dakika); hesaba göre filtrelenir ve bir iletişim kutusu her kategoride kaç drop alındığını listeler. Masaüstü uygulaması dosyayı bir kaydetme iletişim kutusuyla kaydeder; web paneli indirir.

### Masaüstünde veya bir sunucuda

- **Her yerde aynı arayüz.** Masaüstü uygulaması (Linux, Windows, macOS) ve Docker imajı aynı paneli gösterir. Docker bir VPS, NAS veya Raspberry Pi üzerinde, o makinede ekran ya da tarayıcı olmadan çalışır: panel herhangi bir tarayıcıda açılır.
  Telefonda panel ekrana uyum sağlar; HTTPS üzerinden bir uygulama gibi yüklenebilir.
- **Hafif.** Hesap başına yaklaşık 3 MB bellek, neredeyse hiç işlemci kullanımı ve yaklaşık 12 MB'lık bir Docker imajı.

### Güvenlik ve güncellemeler

- **Oturumlarınız korunur.** Twitch şifresi yok: tarayıcınızdan `twitch.tv/activate` adresinde giriş yaparsınız. Oturumlar diskte şifrelenir ve onları [Twitch ayarlarınızdan](https://www.twitch.tv/settings/connections) iptal edebilirsiniz. Masaüstünde isteğe bağlı bir uygulama şifresi uygulamayı kilitler: şifre yazılana kadar hiçbir şey kazılmaz. Docker'da panelin kendi şifresi vardır; panel birkaç dakika işlem yapılmayınca kendini kilitler ve bir proxy arkasında HTTPS üzerinden çalışır. Art arda beş yanlış şifre, IP adresini 30 saniyeliğine engeller.

  <img src="../media/tr/security-lock.avif" alt="Asma kilit: kilitli uygulama, ardından şifresiyle açılması">

- **İmzalı güncellemeler, asla otomatik değil.** Üst çubukta sürüm numarasıyla birlikte bir “Güncelleme” rozeti belirir, ayrıca bir masaüstü bildirimi de aynısını haber verir; bu varsayılan olarak açıktır (**Ayarlar › Bildirimler** içindeki bir anahtar onu kapatır; tarayıcıda sorduğunda “İzin ver”e tıklayın). “Yükle ve yeniden başlat” önce onay ister, ardından uygulama sürümü yükler ve yeniden başlar. Uygulama şifresi varsa uygulama kilitli olarak yeniden başlar: kazmaya devam etmek için şifreyi yeniden yazın. Yalnızca proje tarafından imzalanmış bir sürüm yüklenir. Docker paneli ve systemd ikili dosyası da çalıştırılacak komutlarla birlikte haber verir.

  <img src="../media/tr/updates.avif" alt="“Güncelleme” rozeti, onay, ardından yükleme">

### Ve ayrıca

- Diğer madencilerde olduğu gibi: bonus sandıkları, baskınlar, anlar, topluluk hedefleri, tahminler (stratejiler, bahis zamanı, koşullar), önce izleme serileri ve abone olunan kanallar, sohbette bulunma (kapatılabilir), yeni yayıncılar için varsayılan seçenekler, resimli ve her biri için bir “Test et” düğmesine sahip Discord, Telegram ve webhook bildirimleri (sunucuda dilleri **Ayarlar › Bildirimler** bölümünden seçilir), genel veya hesap başına proxy.
- 29 dil, koyu ve açık tema, masaüstü bildirimleri, sistem tepsisi simgesi, sistemle birlikte başlatma.

  <img src="../media/tr/basics.avif" alt="Diller, açık ve koyu tema, bir masaüstü bildirimi ve sistem tepsisi simgesi">

- **Token listesi yapıştırarak hesap ekleme.** Başka bir yerde zaten oturum açtınız mı? Erişim tokenları listesini, satır başına bir tane olacak şekilde (en fazla 50) yapıştırın; her biri canlı olarak kontrol edilir, “Kontrol edildi: n/N” sayacı ve başarısız olan her satır için yeniden deneme vardır, ardından tek bir ayar adımı tüm gruba uygulanır. İçe aktarma kendisi arka planda, hesap başına canlı ilerlemeyle çalışır, yarıda iptal edilebilir ve daha sonraki yeniden deneme yalnızca başarısız olan hesapları yeniden gönderir. Bu şekilde içe aktarılan bir hesap kaldırılırken tokenı Twitch üzerinde de iptal etme seçeneği sunulur.

> [!WARNING]
> Bu proje Twitch ile bağlantılı değildir. Bir hesabı otomatikleştirmek Twitch'in koşullarını veya bir kampanyanın kurallarını ihlal edebilir.

## İçindekiler

- [Karşılaştırma](#comparison)
- [İndirme](#download)
- [Docker](#docker)
- [Nasıl çalışır](#how-it-works)
- [Sık sorulan sorular](#faq)
- [İlgili projeler](#related-projects)
- [Güvenlik](#security)
- [Kaynaktan derleme](#build)
- [Lisans](#license)

## <a name="comparison"></a>Karşılaştırma

[Twitch Miner](https://github.com/SunkwiBOT/twitch-miner) ile [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2) (kanal puanları) ve [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops) arasındaki karşılaştırma. Kasım 2026'da bu projelerin kodu ve README dosyaları üzerinden doğrulandı.

| | Bu proje | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops kampanyaları: bulur, kazar ve alır | ✅ | ❌ ¹ | ✅ |
| Twitch rozetleri: tüm rozetlerin listesi, her hesabın rozetleri, rozet Drops'ları ve Twitch görevleri sizin sıranızla kazılır (beta) | ✅ | ❌ | kısmen ² |
| Kanal puanları: sandıklar, baskınlar, anlar, tahminler | ✅ | ✅ | ❌ |
| Spam Points: bir kanalın özel ödülü art arda birkaç kez kullanılır | ✅ | ❌ | ❌ |
| Tek hesapta aynı anda Drops ve kanal puanları | ✅ | kısmen ¹ | ❌ |
| Tek uygulamada istediğiniz kadar hesap | ✅ | ❌ ³ | ❌ |
| Birden fazla hesabın ayarlarını aynı anda değiştirme | ✅ | ❌ | ❌ |
| Grafik arayüz | ✅ masaüstü ve web ⁴ | ❌ ⁵ | ✅ masaüstü |
| Ev sunucusu için web paneli (Docker), şifreli ve HTTPS üzerinden erişilebilir | ✅ | ❌ ⁵ | ❌ |
| Diskte şifrelenmiş oturumlar, isteğe bağlı uygulama şifresi | ✅ | ❌ | ❌ |
| 29 dilde arayüz | ✅ | ❌ | kısmen ⁶ |
| Uygulama içinden imzalı güncellemeler (masaüstü) | ✅ | ❌ | ❌ |
| Her yeni hesap için kurulum sihirbazı | ✅ | ❌ ⁵ | ❌ |
| Arayüzden tek tek veya toplu olarak eklenen ve düzenlenen yayıncılar | ✅ | ❌ ⁵ | — |
| Aramayla, bir listeden, başka bir hesaptan veya takip edilen kanallardan içe aktarılan yayıncılar | ✅ | kısmen ⁷ | — |
| Resimli Discord, Telegram ve webhook bildirimleri | ✅ | kısmen ⁸ | ❌ |
| Tüm Drops kampanyalarının aranabilir takvimi | ✅ | ❌ | kısmen ⁹ |
| Kampanya bazında kazma sırası, hariç tutulan kelimeler, yok sayılan kampanya veya ödül | ✅ | ❌ | ❌ ¹⁰ |
| Bir kampanyanın Drops'ları için önce listenizdeki yayıncılar izlenir | ✅ | kısmen ¹¹ | ❌ |
| Arayüzde filtrelenebilir olay günlüğü ve her isteğin listesi | ✅ | ❌ ⁵ | ❌ |
| Hesap başına yaklaşık 3 MB bellek (Docker sunucusu) | ✅ | — ¹² | — ¹² |

¹ Yalnızca listesindeki yayıncıları izler: kampanya seçmeden, onların yayınlarındaki Drops'ları ilerletir ve alır.\
² Ödülleri rozet veya ifade olan Drops kampanyalarını kazmak ya da dışarıda bırakmak için bir ayar; Twitch görevleri ve rozet listesi yok.\
³ Hesap başına bir süreç.\
⁴ Bilgisayarınızda masaüstü uygulaması olarak ve/veya Docker ile bir sunucuda (VPS, NAS, Raspberry Pi); o makinede grafik ortam ve tarayıcı gerekmez: panel herhangi bir tarayıcıda açılır.\
⁵ Bir Python dosyasında yapılandırılır; web sayfası puan grafiklerini ve kayıt dosyasını gösterir.\
⁶ 21 dil.\
⁷ Hesabın takip ettiği kanalları takip edebilir.\
⁸ Yalnızca metin, resim yok. Ayrıca Matrix, Pushover ve Gotify desteği var; bunlar bu projede yok.\
⁹ Filtreli bir kampanya listesi; takvim veya arama yok.\
¹⁰ Kategori adına göre seçilen öncelikli kategorileri ve hariç tutulan kategorileri vardır.\
¹¹ Varsayılan olarak önce listesindeki yayıncılardan yayınında Drops olanları izler (`DROPS` önceliği), ama hiçbir zaman başka bir kanalı izlemez.\
¹² Ölçülmedi.

## <a name="download"></a>İndirme

Sisteminize uygun dosyayı [son sürümden](https://github.com/SunkwiBOT/twitch-miner/releases/latest) indirin.

| Sistem | Paket |
|---|---|
| **Windows** · x64 | Yükleyici (`x64-setup.exe`) veya [taşınabilir çalıştırılabilir dosya](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_x64_portable.exe) |
| **Windows** · ARM64 | Yükleyici (`arm64-setup.exe`) veya [taşınabilir çalıştırılabilir dosya](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/twitch-miner_windows_arm64_portable.exe) |
| **macOS** 13 veya üstü · evrensel: Intel ve Apple Silicon | Disk görüntüsü (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) veya `.rpm` (Fedora, openSUSE) |
| **Linux** · ARM64 | `.AppImage`, `.deb` (Debian, Ubuntu, Raspberry Pi OS) veya `.rpm` (Fedora, openSUSE) |
| **Linux sunucusu** · x86-64 veya ARM64 | Yalnızca sunucu, Docker'sız (`twitch-miner-server_…_linux_x64` veya `_linux_arm64`): bkz. [Docker olmadan](#systemd) |

macOS derlemesi notarize edilmemiştir. İlk seferde uygulamayı açın: macOS onu engeller. Ardından **Sistem Ayarları › Gizlilik ve Güvenlik** bölümüne gidip **Yine de Aç** düğmesine tıklayın.

Güncellemeler imzalıdır. Uygulama açılırken, ardından saatte bir yeni bir sürüm olup olmadığını denetler (sıklığı **Ayarlar › Hakkında** bölümünden değiştirebilirsiniz). Hiçbir sürümü kendiliğinden yüklemez: güncellemeyi, beliren mesajdan ya da **Ayarlar › Hakkında** bölümünden tek tıkla siz yüklersiniz.

Veriler kullanıcı veri klasöründe bulunur:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

Sunucu aynı paneli bir tarayıcıda gösterir. İmajın adı `ghcr.io/sunkwibot/twitch-miner`; `amd64` ve `arm64` için sunulur. Yeni bir sürüm çıktığında panel bunu söyler: `docker compose pull`, ardından `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # her başlangıçta en yeni imajı çeker
    restart: unless-stopped
    ports: ["8080:8080"]   # HTTPS ters proxy arkasında: kaldırın ya da "127.0.0.1:8080:8080" yapın
    environment:
      MINER_PASSWORD: "change-me"   # zorunlu, 12 karakter veya daha uzun
      # MINER_PUBLIC_URL: "https://miner.example.com"   # yalnızca HTTPS ters proxy arkasında
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

<http://localhost:8080> adresini (veya makinenin ağınızdaki adresini) açın ve şifreyi girin.

### Değişkenler

| Değişken | Varsayılan | Amaç |
|---|---|---|
| `MINER_PASSWORD` | yok | Panel şifresi, zorunludur: sunucu onsuz başlamayı reddeder. 12 karakter veya daha uzun olmalıdır (daha kısa bir şifre başlangıçta bir uyarı gösterir). Örnek değer `change-me` reddedilir. |
| `MINER_PASSWORD_FILE` | yok | `MINER_PASSWORD` değerini bir dosyadan, örneğin bir Docker secret'tan okur. İkisinden yalnızca biri, ikisi birden değil. |
| `MINER_PUBLIC_URL` | yok | Yalnızca bir HTTPS ters proxy (Caddy, nginx, Traefik, Cloudflare Tunnel…) arkasında: panelin `https://` adresi, yol olmadan; örneğin `https://miner.example.com`. Proxy `X-Forwarded-Proto`, `X-Forwarded-For` başlıklarını ve websocket'leri iletmelidir (yaygın proxy'ler bunu varsayılan olarak yapar) ve container'ın portuna doğrudan ulaşılamamalıdır. Panel bu durumda proxy üzerinden HTTPS ile gelmeyen her şeyi reddeder ve 5 yanlış şifreden sonra bir adresi 30 saniyeliğine kilitler. Proxy aynı makinede veya onun yerel ağında çalışmalıdır: genel bir adresten gelen bağlantı reddedilir. |
| `MINER_TRUSTED_HOPS` | `1` | Yalnızca `MINER_PUBLIC_URL` ile: `X-Forwarded-For` başlığına girdi ekleyen proxy sayısı, 1 ile 10 arasında. Art arda iki proxy varsa, örneğin Traefik'in önünde Cloudflare, `2`. |
| `MINER_PROXY_NETWORKS` | yok | Yalnızca `MINER_PUBLIC_URL` ile: proxy'nin bağlandığı ağlar, virgülle ayrılmış: tek bir adres ya da `192.168.1.0/24` veya `2001:db8:1::/64` gibi bir ağ. Ayarlandığında proxy yalnızca loopback adresinden ve bu ağlardan kabul edilir, artık tüm özel yerel ağdan değil. |
| `MINER_VAULT_PASSPHRASE` | yok | Yalnızca uygulama şifresi olan masaüstü uygulamasından kopyalanmış bir veri klasörü için: başlangıçta klasörün kilidini açar. Sıfırdan kurulan bir sunucunun buna hiç ihtiyacı olmaz. |
| `MINER_VAULT_PASSPHRASE_FILE` | yok | `MINER_VAULT_PASSPHRASE` değerini bir dosyadan okur. İkisinden yalnızca biri, ikisi birden değil. |
| `DATA_DIR` | imajda `/data`, aksi hâlde çalışma dizinindeki `data` | Veritabanı, anahtar dosyası ve kayıtlar. |
| `LOG_DIR` | `$DATA_DIR/logs` | Kayıt dosyaları: günde bir tane, son 14 tanesi saklanır. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Dinleme adresi: bir IP adresi ve 1 ile 65535 arasında bir port. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Kayıt düzeyi. |

Sunucu değişkenleri başlangıçta bir kez okur ve denetler. Geçersiz bir değer, sessizce varsayılana dönmek yerine sunucuyu bir mesajla durdurur.

Bir ters proxy arkasında `MINER_PUBLIC_URL` değişkenini ayarlayın: o olmadan, bir proxy üzerinden gelen ilk istek kayıtlara bir uyarı yazar, çünkü giriş denemeleri o zaman proxy'nin adresine sayılır ve tek bir ziyaretçinin yanlış şifreleri herkesi dışarıda bırakır. Canlı bağlantı her 30 saniyede bir ping gönderir; böylece hiçbir şey olmadığında proxy onu kapatmaz. Twitch veya ağ hata verdiğinde sunucu `424` ile yanıt verir, asla `502` veya `504` ile değil: Cloudflare arkasında Cloudflare'in sayfası yerine gerçek hatayı görürsünüz.

### <a name="systemd"></a>Docker olmadan (systemd)

Docker'sız bir Linux sunucusunda, yayınlanan sürüm sunucuyu tek bir dosya olarak da
içerir, panel içinde gömülü: `twitch-miner-server_<sürüm>_linux_x64` veya
`twitch-miner-server_<sürüm>_linux_arm64`. Docker imajının çalıştırdığı programın
kendisidir, statik olarak derlenmiştir: HTTPS için okuduğu sistem CA sertifikaları hariç
başka hiçbir şey kurmanız gerekmez (Docker imajı da aynı nedenle bunları kopyalar). Aynı
değişkenleri kabul eder.

```bash
# ARM64: twitch-miner-server_<sürüm>_linux_arm64
sudo install -m 755 twitch-miner-server_<sürüm>_linux_x64 /usr/local/bin/twitch-miner
sudo install -m 600 /dev/null /etc/twitch-miner.env
```

Kendi şifrenizi, 12 karakter veya daha uzun, yalnızca root'un okuyabildiği
`/etc/twitch-miner.env` dosyasına yazın:

```ini
MINER_PASSWORD=change-me
```

Ardından `/etc/systemd/system/twitch-miner.service` dosyasında servisi oluşturun:

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

systemd, veriler için `/var/lib/twitch-miner` dizinini oluşturur ve sunucuyu kendi
kullanıcısı altında çalıştırır, asla root olarak değil. `http://<sunucunun adresi>:8080`
adresini açın; kayıtlar `journalctl -u twitch-miner` komutunda.

Bu dosya kendini güncellemez. Panel yeni bir sürüm gösterdiğinde, yeni dosyayı yayından
indirin, ardından eskisinin yerine koyup yeniden başlatın:

```bash
# ARM64: twitch-miner-server_<sürüm>_linux_arm64
sudo install -m 755 twitch-miner-server_<sürüm>_linux_x64 /usr/local/bin/twitch-miner
sudo systemctl restart twitch-miner
```

## <a name="how-it-works"></a>Nasıl çalışır

Her hesap 20 saniyede bir iki kanalını seçer: kuyruğunun başındaki kampanya için en iyi canlı kanal, ardından puan listesindeki yayıncılar. Canlı kanalı olmayan kampanya, canlı kanalı olan bir sonrakine yer vermek için atlanır (kuyruğunda “Canlı kanal yok” yazar); canlı kanallar 90 saniyede bir yeniden kontrol edilir ve hesap, yayıncılarından biri canlı yayına girer girmez o kampanyaya geri döner.

Twitch, hesap başına aynı anda **bir Drops kampanyası** ve **puanlar için iki kanal** sayar. Ekim 2026'dan beri bir drop yalnızca yayıncı hesabı bağlıysa alınabilir. Bağlı olmayan kampanyalar yine de ilerler ve panel bağlantıyı gösterir.

Twitch, 30 dakika gerçek izleme sonrasında en üstteki kampanyanın kendi ilerlemesini hiç hareket ettirmediyse, o kampanyayı saymıyor demektir: hesap onu bir kenara bırakır, sıradaki kampanya onun yerini alır ve 6 saat sonra otomatik olarak yeniden denenir — ilerleme daha önce geri dönerse daha erken, ancak yeniden denenen kampanya yine de o sırada kazılan kampanyanın arkasında bekleyebilir. Herhangi bir ilerleme bu bekleme süresini sıfırlar ve bu, her kampanya türü için geçerlidir (Drops, rozetler, görevler). Twitch bunun yerine aynı kategorideki başka bir kampanyayı sayarsa ve o kampanya tamamlanır veya alınırsa, bir kenara bırakma tam 6 saat beklemek yerine hemen kaldırılır. Uygulamayı veya sunucuyu yeniden başlatmak her bir kenara bırakmayı unutturur: takılı kalan bir kampanya basitçe yeni bir 30 dakika alır.

Kampanya listesi [Twitch Drops API](https://drops.sunkwi.com) projesinden gelir (5 dakikada bir yenilenir), çünkü Twitch onu TV istemcisinden gizler.

## <a name="faq"></a>Sık sorulan sorular

### Riskler ve Twitch

**Hesabım yasaklanabilir mi?**\
Kimse yasaklanmayacağını garanti edemez. Bu proje Twitch ile bağlantılı değildir ve bir hesabı otomatikleştirmek Twitch'in koşullarını veya bir kampanyanın kurallarını ihlal edebilir. Madenci hesap başına en fazla iki kanal izler, bu da Twitch'in sınırıdır, ve hiçbir video indirmez. Karar sizin.

**Video gerçekten izleniyor mu?**\
Hayır, hiçbir video indirilmez. Drops için madenci 15 saniyede bir yayının oynatma listesini en düşük kalitede okur ve yalnızca her yeni segmentin başlığını ister. Puanlar için her dakika Twitch'in “izlenen dakika” sinyalini gönderir. İzlenen her kanal kaç segment istendiğini gösterir.

**Neden en fazla iki kanal?**\
Twitch'in sınırı bu: hesap başına yalnızca aynı anda izlenen iki kanalı ve tek seferde yalnızca bir Drops kampanyasını sayar. **Drops ve puan** modunda bir kanal kampanyaya, diğeri yayıncılarınıza hizmet eder. **Hesap ayarları** içindeki “Tek kanal” ayarı daha az dikkat çeker.

### Drops

**Bir kampanya kazılmıyor veya bir drop ilerlemiyor.**\
Hesabın **Drops** sayfasını açın. “Kazma sırası” bölümünde bekleyen her kampanya nedenini söyler: “Canlı kanal yok” veya “Bekliyor (tek seferde bir kampanya)”. “Twitch bunu saymıyor — … tekrar denenecek” ifadesi, 30 dakikalık gerçek izlemenin Twitch tarafında hiçbir ilerleme getirmediği anlamına gelir: kampanya bir kenara bırakılır ve gösterilen sürede, ya da ilerleme daha önce geri dönerse daha erken, otomatik olarak yeniden denenir. Bu listede görünmeyen bir kampanya dışarıda bırakılmıştır: “Ne kazılacak, sırayla”, “Hangi kampanyalar” ayarı ya da **Hariç tutmalar** sekmesindeki bir kural yüzünden, zamanında bitemeyeceği için ya da kalan tüm ödülleri abonelik gerektirdiği için. Abonelik gerektiren bir ödül, kampanya sırasında kategorideki bir kanala abone olana ya da abonelik hediye edene kadar bekler (devam eden bir abonelik, Prime aboneliği ya da yenileme sayılmaz); her gün izlenmesi gereken bir rozet ertesi günü bekler; kampanyası başlamadan hiçbir şey kazılmaz ve başka bir drop'un ardından gelen bir drop, ilki bitene kadar bekler. **Yalnızca puan** modunda madenci hiçbir kampanya seçmez. Bir kampanyanın ayrıntılarındaki “Önce kaz” onu en başa taşır; yok sayılan bir kampanya, **Hariç tutmalar** sekmesindeki “Yeniden kaz” ile geri gelebilir. **Kampanyalar** sekmesindeki **Engelli** grubu aynı nedenleri gösterir.

**Yayıncı hesabını bağlamam gerekiyor mu?**\
Ödülü almak için evet: Ekim 2026'dan beri Twitch, bir drop yalnızca yayıncı hesabı bağlıysa alınmasına izin veriyor. Varsayılan olarak bağlı olmayan bir kampanya yine de ilerler ve alma işlemi bağlantıyı bekler. “Bağlı değil” rozeti bunu belirtir; kampanyanın ayrıntılarındaki “Yayıncı hesabını bağla” da bağlama sayfasını açar. Yalnızca bağlı kategorileri kazmak için “Yalnızca yayıncı hesabı bağlı kategoriler” seçeneğini seçin.

### Kurulum ve veriler

**Masaüstü uygulaması mı, Docker mı?**\
Masaüstü uygulaması bilgisayarınız açıkken kazar; sistem tepsisi simgesi, isteğe bağlı şifresi ve tek tıkla güncellemeleri vardır. Docker, sürekli açık duran bir makinede (VPS, NAS, Raspberry Pi) gece gündüz kazar; onu bir tarayıcıda, `MINER_PASSWORD` ile korunarak açarsınız. Arayüz aynıdır.

**Verilerim nerede?**\
Veri klasöründe: Linux'ta `~/.local/share/twitch-miner`, Windows'ta `%APPDATA%\twitch-miner`, macOS'ta `~/Library/Application Support/twitch-miner`, Docker'da ise `/data` birimi. Masaüstünde **Ayarlar › Masaüstü** onu tek tıkla açar. İçinde `miner.db` (hesaplar, ayarlar, günlük), `master.key` (uygulama şifresi ayarlanmamışsa oturum anahtarı) ve kayıtlar bulunur.

**Nasıl güncellerim?**\
Masaüstü uygulaması açılışta, ardından saatte bir yeni sürüm olup olmadığını denetler (bunu **Ayarlar › Hakkında** bölümünden değiştirebilirsiniz ve bulduğunda size haber verebilir). “Güncelleme” rozetine, ardından “Yükle ve yeniden başlat” düğmesine tıklayın. Docker'da panel size haber verir: `docker compose pull`, ardından `docker compose up -d` komutunu çalıştırın. Docker olmadan da panel aynı şekilde haber verir: dosyayı değiştirip hizmeti yeniden başlatmak için [Docker olmadan](#systemd) bölümüne bakın.

**Uygulama şifresini unuttum.**\
Kurtarılamaz: Twitch oturumlarınızı şifreleyen odur. Uygulamadan çıkın, veri klasörünü silin, ardından hesaplarınızı yeniden ekleyin; ayarlarını da baştan yapmanız gerekir. Docker'da panel şifresi `MINER_PASSWORD` içinde değiştirilir, ardından `docker compose up -d` çalıştırılır.

### Puanlar ve tahminler

**Saatte kaç puan?**\
Twitch'e ve her kanala bağlıdır. Madenci, Twitch'in bir izleyiciye verdiğini kazanır: izleme süresi puanları, bonus sandıkları (yaklaşık 15 dakikada bir) ve izleme serisi bonusu. Abonelik, seviyesine göre puanları ×1,2 ile ×2 arasında katlar; varsayılan olarak açık olan “Önce abone olunan kanallar” ile bu kanallar listenizin geri kalanından önce gelir. Ana sayfa kazanılan puanları, Günlük ise her kazancı gösterir.

**Bahisler riskli mi?**\
Evet, puan kaybedebilirsiniz. Tahminler varsayılan olarak her yayıncı için ayrı ayrı kapalıdır. Stratejiyi, bahis zamanını, bahis miktarını (varsayılan olarak bakiyenin %5'i, en fazla 50.000 puan), bir asgari bakiyeyi ve isterseniz en az 1,5 oran gibi bir koşulu siz belirlersiniz. Bahis miktarını düşük tutun. 10 puanın altındaki bir bahis asla yapılmaz.

**Bir yayıncı izlenmiyor.**\
Yalnızca açık ve canlı yayında olan yayıncılar, o da yalnızca boş bir kanalda izlenir: **Drops ve puan** modunda iki kanaldan biri geçerli kampanyaya hizmet eder; **Yalnızca Drops** modunda hiçbir yayıncı izlenmez; duraklatılmış bir hesap hiçbir şey izlemez. Yerler şu sırayla dağıtılır ve izlenen her kanalda bir rozet olarak gösterilir: “Seri tehlikede”, “Seri bonusu”, “Abone”, ardından “Liste sırası”. Yayıncıyı listede yukarı taşıyın ya da “Önce izleme serileri” ve “Önce abone olunan kanallar” seçeneklerini kapatın.

## <a name="related-projects"></a>İlgili projeler

Aynı geliştiricinin diğer projeleri:

- [**Twitch Drops API**](https://drops.sunkwi.com): süren tüm Twitch Drops ve ödül kampanyalarını sunan, dakikada bir güncellenen ücretsiz ve herkese açık bir API. Bu uygulama kampanya kataloğunu buradan okur.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): Twitch canlı yayınlarını ve Drops kampanyalarını sunucunuzda duyuran bir Discord botu.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): Kick canlı yayınlarını ve Drops kampanyalarını sunucunuzda duyuran bir Discord botu.

## <a name="security"></a>Güvenlik

- Token'lar AES-256-GCM ile mühürlenir. Anahtar ya veritabanının yanındaki rastgele bir `master.key` dosyasıdır (varsayılan, yapmanız gereken bir şey yok) ya da uygulama şifresinden Argon2id ile türetilir.
- Veri klasörü ve kayıt klasörü özeldir (`0700`): yalnızca madenciyi çalıştıran kullanıcı onları açabilir.
- HTTP paneli şifreyi sabit sürede denetler ve HttpOnly, SameSite=Strict çerezleri kullanır (HTTPS arkasında `__Host-` önekiyle birlikte `Secure`). Sunucu her oturumun yalnızca SHA-256 özetini tutar. Bir oturumu 30 gün sonra veya çıkışta sonlandırır. Herhangi bir istek gövdesini okumadan önce oturumu denetler ve gövdeleri 256 KiB ile sınırlar.
- Bir IP adresinden art arda 5 yanlış şifre girilirse (giriş formu veya bearer), o adres 30 saniye bekler. Sunucu şifreye bakmadan `Retry-After` ile `429` yanıtı verir. Her yanlış şifrenin yanıtı ayrıca yarım saniye sürer. Aynı bağlantı adresinden, paylaşılan bir proxy dahil, bir dakikada 30'dan fazla yanlış şifre gelirse o adres her 2 saniyede yalnızca bir deneme hakkı alır: tahmin yavaşlar ve kimse dışarıda kalmaz.
- Ağınızın dışında paneli `MINER_PUBLIC_URL` ile bir HTTPS ters proxy'nin arkasına koyun ([Docker](#docker) bölümüne bakın).
- Madenci yalnızca `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`, `drops.sunkwi.com` ve yapılandırdığınız bildirim hedefleriyle konuşur. Uygulama (masaüstü ve Docker) yeni sürümleri `github.com` üzerinde denetler; masaüstü uygulaması bunları projenin GitHub sürümlerinden indirir. Sürümler imzalıdır: yalnızca gösterilen sürüm yüklenir.

## <a name="build"></a>Kaynaktan derleme

**Ön koşullar:** Rust, Node.js ve sisteminiz için [Tauri ön koşulları](https://v2.tauri.app/start/prerequisites/).

```bash
npm ci
npm run build          # panel
cargo build --release -p miner-server   # sunucu: target/release/twitch-miner
npm run bundle:linux   # masaüstü paketleri (Tauri'nin Linux bağımlılıkları gerekir)
```

Kod denetimleri:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (tür bilgisiyle), svelte-check ve inlang i18n kontrolünü
```

Arayüzün TypeScript türleri, Rust türlerinden [ts-rs](https://github.com/Aleph-Alpha/ts-rs) ile `ui/src/lib/bindings` içine üretilir. Arayüzün gördüğü bir türü değiştirdikten sonra `npm run bindings` komutunu çalıştırın.

## <a name="license"></a>Lisans

[MIT](../../LICENSE). [DevilXD'nin TwitchDropsMiner'ından](https://github.com/DevilXD/TwitchDropsMiner), [Twitch-Channel-Points-Miner-v2'den](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2) ve bunların fork'larından ilham alınmıştır.
