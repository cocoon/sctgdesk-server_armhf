<p align="center">
  <a href="#how-to-build-manually">Manually</a> •
  <a href="#docker-images">Docker</a> •
  <a href="#how-to-create-a-keypair">Keypair</a> •
  <a href="#packages">Binaries</a> •
  <a href="#env-variables">Variables</a><br>
  [<a href="README-FR.md">French</a>] | [<a href="README-DE.md">Deutsch</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-TW.md">繁體中文</a>] | [<a href="README-ZH.md">简体中文</a>]<br>
</p>

# SctgDesk 伺服器程式

[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**二進位下載**](https://github.com/sctg-development/sctgdesk-server/releases)

[**API 文件**](https://sctg-development.github.io/sctgdesk-api-server/)

這是 RustDesk Server 的修改版本，它是免費和開源的。

*   第一個區別是這個版本包含新的*技術合作計劃*模式包含在 RustDesk Server Pro 版本中。
*   第二個區別是，這個版本包括了 Rustdesk Server Pro API 伺服器的初步實現。
    *   支援個人通訊錄
    *   支援組級別的共享通訊簿
        *   唯讀、讀寫、管理員
    *   支援用戶級別的共享通訊簿
        *   唯讀、讀寫、管理員
*   第三個區別是，此版本包含了簡單 Web 控制台的初步實現。

Web控制台可在以下位址訪問`http://<server-ip>:21114/`登錄名「admin」，密碼「Hello，world！\
您可以在位址所在的內置 API 伺服器中瀏覽 API 文件`http://<server-ip>:21114/api/doc/`.

非互動式 API 文件可在以下位置獲得[sctgdesk-api-server 倉庫](https://sctg-development.github.io/sctgdesk-api-server/).

## TL;博士

您可以使用以下命令`docker-compose.yml`啟動伺服器的檔案：

```yaml
version: '3'

networks:
  sctgdesk-net:
    external: false

services:
  hbbs:
    container_name: hbbs
    ports:
      - 21114:21114
      - 21115:21115
      - 21116:21116
      - 21116:21116/udp
      - 21118:21118
    image: sctg/sctgdesk-server:latest
    command: hbbs -r sctgdesk.example.com:21117
    volumes:
      - ./data:/usr/local/share/sctgdesk
    networks:
      - sctgdesk-net
    depends_on:
      - hbbr
    restart: unless-stopped

  hbbr:
    container_name: hbbr
    ports:
      - 21117:21117
      - 21119:21119
    image: sctg/sctgdesk-server:latest
    command: hbbr
    volumes:
      - ./data:/usr/local/share/sctgdesk
    networks:
      - sctgdesk-net
    restart: unless-stopped
```

並使用以下命令啟動伺服器：

```bash
mkdir -p data
docker-compose up 
```

### 預設管理員使用者

預設管理員使用者是使用使用者名創建的`admin`和密碼`Hello,world!`.您可以在 Web 控制臺上首次登錄後更改密碼。

## API 單機版

api 獨立版本是包含 API 伺服器和 webconsole 的伺服器版本，但不包括 rendez-vous 伺服器。\
獨立版本在其自己的存儲庫中可用[sctgdesk-api-伺服器](https://github.com/sctg-development/sctgdesk-api-server).\
對於所有與 api 或 webconsole 相關的問題，請參閱[sctgdesk-api-伺服器](https://github.com/sctg-development/sctgdesk-api-server)存儲庫。

## 截圖

### Web控制台

<img width="1085" alt="login" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/fe72a374-8a98-4606-8632-3d919f9317c9">

<img width="1285" alt="dashboard" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/0bb148d6-8723-491f-88c5-b98331d64f61">

<img width="1085" alt="devices" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/6ae55861-f65c-4950-a068-f22eef3ad81a">

<img width="1084" alt="users" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/8d225841-43f5-44f4-8d41-5b6ca3324096">

<img width="1087" alt="groups" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/d84ce3d3-1d19-4765-883f-001f313a4a1e">

<img width="1089" alt="address books" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/db13010b-077a-4e14-943b-9d8de3266f82">

<img width="730" alt="rues" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/3a990deb-d8bb-4725-a47d-435ec3667fee">

<img width="621" alt="add rules" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/355f3903-2b54-4b08-abd0-e33c84a260ed">

### Api 文件

<img width="1502" alt="apidoc" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/88fe7910-fe62-43e5-a16c-70dc1201e040">

### 在 Rustdesk 用戶端中使用

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## 生成自動更新連結

我們修改了用戶端，以從 api 伺服器而不是 Github 版本中檢索自動更新連結。\
為了使自動更新連結正常工作，您需要修改用戶端以從 api 伺服器檢索自動更新連結。這[你怎麼能做到](https://github.com/sctg-development/sctgdesk/blob/481d3516fef1daa145d8044594187cb11959f8be/src/common.rs#L953L972):

```rust
// src/common.rs
#[tokio::main(flavor = "current_thread")]
async fn check_software_update_() -> hbb_common::ResultType<()> {
    let url=format!("{}/api/software/releases/latest",get_api_server("".to_owned(), "".to_owned())).to_owned();
    log::info!("URL for checking software updates: {}", url);
    //let url = "https://github.com/rustdesk/rustdesk/releases/latest";
    let latest_release_response = create_http_client_async().get(url).send().await?;
    let latest_release_version = latest_release_response
        .url()
        .path()
        .rsplit('/')
        .next()
        .unwrap_or_default();

    let response_url = latest_release_response.url().to_string();

    if get_version_number(&latest_release_version) > get_version_number(crate::VERSION) {
        *SOFTWARE_UPDATE_URL.lock().unwrap() = response_url;
    }
    Ok(())
}
```

# 安全

嵌入式 API 伺服器不受保護，也無法抵禦 DDOS 攻擊。一個好的做法是在 API 伺服器之前使用反向代理。NGINX 是實現此目的的不錯選擇。HAProxy也是一個不錯的選擇。\
我們在生產環境中的 API 伺服器前面使用 HAProxy。
這是我們的HAProxy配置檔，僅作為示例提供。您應該根據自己的需要進行調整。

```haproxy
global
    log /dev/log    local0
    log /dev/log    local1 notice
    chroot /var/lib/haproxy
    stats socket /run/haproxy/admin.sock mode 660 level admin expose-fd listeners
    stats timeout 30s
    user haproxy
    group haproxy
    daemon

defaults
    log global
    retries 2
    timeout connect 3000ms
    timeout server 5000ms
    timeout client 5000ms

frontend hbbs_wss
    bind 0.0.0.0:21120 ssl crt /etc/haproxy/hbb.pem
    default_backend hbbs_wss_backend

frontend hbbs_api
    mode http
    option forwardfor
    bind 0.0.0.0:21114 ssl crt /etc/haproxy/api.pem
    http-request set-header X-Forwarded-Proto https
    default_backend hbbs_api_backend

frontend hbbs_api_443
    mode http
    option forwardfor
    bind 0.0.0.0:443 ssl crt /etc/haproxy/api.pem
    http-request set-header X-Forwarded-Proto https
    filter compression
    compression algo gzip
    compression type text/css text/html text/javascript application/javascript text/plain text/xml application/json
    compression offload
    default_backend hbbs_api_backend

frontend hbbr_wss
    bind 0.0.0.0:21121 ssl crt /etc/haproxy/hbb.pem
    default_backend hbbr_wss_backend

backend hbbs_api_backend
    mode http
    server srv_main 127.0.0.1:21113

backend hbbs_wss_backend
    server srv_main 127.0.0.1:21118

backend hbbr_wss_backend
    server srv_main 127.0.0.1:21119
```

hbbs 伺服器啟動時

```service
[Unit]
Description=Rustdesk Signal Server

[Service]
Type=simple
LimitNOFILE=1000000
ExecStart=/usr/bin/hbbs --api-port=21113 -k AucFCOYVWNHRkJnx13FFh7C0tmUZ3nei5wXKmlfK6WPYthz65fRavaA5HO/OIz2kq+bCSlAqBkZgvikwVGqw/Q== --mask=10.10.0.235/24 -r rendez-vous.example.org -R rendez-vous.example.org
#Environment="RUST_LOG=debug"
Environment="ALWAYS_USE_RELAY=Y"
Environment="OAUTH2_CREATE_USER=1"
Environment="S3CONFIG_FILE=s3config.toml"
Environment="OAUTH2_CONFIG_FILE=oauth2.toml"
WorkingDirectory=/var/lib/rustdesk-server/
User=
Group=
Restart=always
StandardOutput=append:/var/log/rustdesk-server/hbbs.log
StandardError=append:/var/log/rustdesk-server/hbbs.error
# Restart service after 10 seconds if node service crashes
RestartSec=10

[Install]
WantedBy=multi-user.target
```

# RustDesk 伺服器程式

[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**下載**](https://github.com/sctgdesk/sctgdesk-server/releases)

[**手動**](https://rustdesk.com/docs/en/self-host/)

[**常見問題**](https://github.com/rustdesk/rustdesk/wiki/FAQ)

自託管您自己的 RustDesk 伺服器，它是免費且開源的。

## 如何手動構建

首先，您需要有一個有效的 Rust 開發工具鏈和一個 Node ≥ 20 的工作安裝。

*   Unices（Linux、MacOS 等）：

```bash
DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3 cargo build --release
```

*   帶有 cmd.exe shell 的 Windows：

```cmd
set "DATABASE_URL=sqlite://%CD%/db_v2.sqlite3" && cargo build --release
```

將在 target/release 中生成三個可執行檔。

*   hbbs - 帶有 API 伺服器的 RustDesk ID/Rendezvous 伺服器
*   hbbr - RustDesk 中繼伺服器
*   rustdesk-utils - RustDesk CLI 實用程式

您可以在以下位置找到更新的二進位檔。[釋放](https://github.com/sctg-development/sctgdesk-server/releases)頁。

版本 v1.1.99-40 之後發佈的所有二進位檔都使用 Github Actions 進行認證。您可以通過檢查二進位檔的sha256sum來檢查證明`https://search.sigstore.dev/?hash=<sha256>`例如。

如果您想要額外的功能[RustDesk 伺服器專業版](https://rustdesk.com/pricing.html)可能更適合你。

如果你想開發自己的伺服器，[rustdesk-server-demo](https://github.com/rustdesk/rustdesk-server-demo)對你來說，可能比這個倉庫更好、更簡單的開始。

## Docker 鏡像

Docker 鏡像是自動生成的，並在每個 github 版本上發佈。

這些圖像是針對`ubuntu-22.04`僅添加主要二進位檔（`hbbr`和`hbbs`).它們可在以下位置獲得[Docker 中心](https://hub.docker.com/r/sctg/sctgdesk-server/)帶有以下標籤：

|建築 |圖片：標籤 |
|--- |--- |
|AMD64 （英语） |`sctg/sctgdesk-server:latest`|
|arm64v8 |`sctg/sctgdesk-server:latest`|

您可以直接使用以下命令開始這些圖像`docker run`使用以下命令：

```bash
docker run --name hbbs --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbr 
```

或不`--net=host`，但是P2P直連無法正常工作。

對於使用 SELinux 的系統，將`/root`由`/root:z`是容器正常運行所必需的。或者，可以完全禁用 SELinux 容器分離，並添加該選項`--security-opt label=disable`.

```bash
docker run --name hbbs -p 21114:21114 -p 21115:21115 -p 21116:21116 -p 21116:21116/udp -p 21118:21118 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr -p 21117:21117 -p 21119:21119 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-serverlatest hbbr 
```

這`relay-server-ip`參數是運行這些容器的伺服器的IP位址（或 DNS 名稱）。這**自選**`port`如果您使用的埠與以下埠不同，則必須使用參數**21117**為`hbbr`.

您還可以使用 docker-compose，將此設定用作範本：

```yaml
version: '3'

networks:
  sctgdesk-net:
    external: false

services:
  hbbs:
    container_name: hbbs
    ports:
      - 21114:21114
      - 21115:21115
      - 21115:21115
      - 21116:21116
      - 21116:21116/udp
      - 21118:21118
    image: sctg/sctgdesk-server:latest
    command: hbbs -r sctgdesk.example.com:21117
    volumes:
      - ./data:/usr/local/share/sctgdesk
    networks:
      - sctgdesk-net
    depends_on:
      - hbbr
    restart: unless-stopped

  hbbr:
    container_name: hbbr
    ports:
      - 21117:21117
      - 21119:21119
    image: sctg/sctgdesk-server-server:latest
    command: hbbr
    volumes:
      - ./data:/usr/local/share/sctgdesk
    networks:
      - sctgdesk-net
    restart: unless-stopped
```

編輯第16行以指向您的中繼伺服器（偵聽埠 21117 的伺服器）。如果需要，您還可以編輯音量行（第 18 行和第 33 行）。

（docker-compose 功勞歸 @lukebarone 和 @QuiGonLeong）

> 注意這裡，中國的sctg/sctgdesk-server-server：latest可能會替換為dockerhub上的最新版本號，如sctg/sctgdesk-server-server：1.1.99-37。否則，可能會因鏡像加速而拉取舊版本。

## 如何創建金鑰對

加密需要金鑰對;如前所述，您可以提供它，但您需要一種方法來創建一個。

您可以使用以下命令產生金鑰對：

```bash
/usr/bin/rustdesk-utils genkeypair
```

如果您沒有（或不想要）該`rustdesk-utils`包，您可以使用 Docker 呼叫相同的命令：

```bash
docker run --rm --entrypoint /usr/bin/rustdesk-utils  sctg/sctgdesk-server-server:latest genkeypair
```

輸出將如下所示：

```text
Public Key:  8BLLhtzUBU/XKAH4mep3p+IX4DSApe7qbAwNH9nv4yA=
Secret Key:  egAVd44u33ZEUIDTtksGcHeVeAwywarEdHmf99KM5ajwEsuG3NQFT9coAfiZ6nen4hfgNICl7upsDA0f2e/jIA==
```

## 包

每個二進位檔都有單獨的.deb包，您可以在[釋放](https://github.com/sctg-development/sctgdesk-server/releases).
這些包適用於以下發行版：

*   Ubuntu 22.04 LTS
*   MacOS Intel 或 Apple Silicon
*   Windows x86\_64 或 i686

## ENV 變數

可以使用這些 ENV 變數來配置 hbbs 和 hbbr。
您可以像往常一樣指定變數或使用`.env`檔。

|變數 |二進位 |描述 |
|--- |--- |--- |
|ALWAYS_USE_RELAY |哈佛商學院 |如果設置為**“Y”**禁止直接對等連接 |
|DOWNGRADE_START_CHECK |哈佛商業評論 |降級檢查前的延遲（以秒為單位） |
|DOWNGRADE_THRESHOLD |哈佛商業評論 |降級檢查閾值 （bit/ms） |
|鑰匙 |HBBS/HBBR |如果設置為強制使用特定鍵，如果設置為**"\_"**強制使用任何金鑰 |
|LIMIT_SPEED |哈佛商業評論 |速度限制（Mb/s） |
|OAUTH2\_CONFIG_FILE |哈佛商學院 |OAuth2 配置文件的路徑 |
|OAUTH2\_CREATE_USER |哈佛商學院 |如果設置為**"1"**如果使用者不存在，請創建該使用者 |
|港口 |HBBS/HBBR |偵聽埠（HBB 為 21116 - HBBR 為 21117） |
|繼電器 |哈佛商學院 |運行 hbbr 的電腦的 IP 位址/DNS 名稱（用逗號分隔） |
|RUST_LOG |全部 |設置調試級別（錯誤|警告|資訊|調試|跟蹤） |
|S3CONFIG_FILE |哈佛商學院 |S3 配置文件的路徑 |
|SINGLE_BANDWIDTH |哈佛商業評論 |單個連接的最大頻寬（以 Mb/s 為單位） |
|TOTAL_BANDWIDTH |哈佛商業評論 |最大總頻寬（單位：Mb/s） |
