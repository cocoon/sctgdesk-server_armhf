<p align="center">
  <a href="#how-to-build-manually">Manually</a> •
  <a href="#docker-images">Docker</a> •
  <a href="#how-to-create-a-keypair">Keypair</a> •
  <a href="#packages">Binaries</a> •
  <a href="#env-variables">Variables</a><br>
  [<a href="README-FR.md">French</a>] | [<a href="README-DE.md">Deutsch</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-TW.md">繁體中文</a>] | [<a href="README-ZH.md">简体中文</a>] | [<a href="README-RU.md">Русский</a>]<br>
</p>

# SctgDesk 服务器程序

[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**二进制下载**](https://github.com/sctg-development/sctgdesk-server/releases)

[**API 文档**](https://sctg-development.github.io/sctgdesk-api-server/)

这是 RustDesk Server 的修改版本，它是免费和开源的。

*   第一个区别是这个版本包括新的*TCP 协议*模式包含在 RustDesk Server Pro 版本中。
*   第二个区别是此版本包括 Rustdesk Server Pro API 服务器的初步实现。
    *   支持个人地址簿
    *   支持组级别的共享地址簿
        *   只读、读写、管理员
    *   支持用户级别的共享地址簿
        *   只读、读写、管理员
*   第三个区别是，此版本包括简单 Web 控制台的初步实现。

Web 控制台可通过以下地址访问`http://<server-ip>:21114/`登录名 “admin” 和密码 “Hello，world！” 。\
您可以在 builtins API 服务器中的地址`http://<server-ip>:21114/api/doc/`.

非交互式 API 文档可在[sctgdesk-api-server 存储库](https://sctg-development.github.io/sctgdesk-api-server/).

## TL;博士

您可以使用以下内容`docker-compose.yml`文件启动服务器：

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

并使用以下命令启动服务器：

```bash
mkdir -p data
docker-compose up 
```

### 默认管理员用户

默认 admin 用户是使用用户名创建的`admin`和密码`Hello,world!`.您可以在 Web 控制台上首次登录后更改密码。

## API 独立版本

api 独立版本是包括 API 服务器和 Web 控制台，但不包括 rendez-vous 服务器的服务器版本。\
独立版本在其自己的存储库中提供[sctgdesk-api 服务器](https://github.com/sctg-development/sctgdesk-api-server).\
所有 api 或 webconsole 相关问题，请参考[sctgdesk-api 服务器](https://github.com/sctg-development/sctgdesk-api-server)存储 库。

## 屏幕截图

### Web 控制台

<img width="1085" alt="login" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/fe72a374-8a98-4606-8632-3d919f9317c9">

<img width="1285" alt="dashboard" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/0bb148d6-8723-491f-88c5-b98331d64f61">

<img width="1085" alt="devices" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/6ae55861-f65c-4950-a068-f22eef3ad81a">

<img width="1084" alt="users" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/8d225841-43f5-44f4-8d41-5b6ca3324096">

<img width="1087" alt="groups" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/d84ce3d3-1d19-4765-883f-001f313a4a1e">

<img width="1089" alt="address books" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/db13010b-077a-4e14-943b-9d8de3266f82">

<img width="730" alt="rues" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/3a990deb-d8bb-4725-a47d-435ec3667fee">

<img width="621" alt="add rules" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/355f3903-2b54-4b08-abd0-e33c84a260ed">

### API 文档

<img width="1502" alt="apidoc" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/88fe7910-fe62-43e5-a16c-70dc1201e040">

### 在 Rustdesk 客户端中使用

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## 生成自动更新链接

我们修改了客户端，从 api 服务器而不是 Github 版本检索自动更新链接。\
要使自动更新链接正常工作，您需要修改客户端以从 api 服务器检索自动更新链接。这[你怎么做](https://github.com/sctg-development/sctgdesk/blob/481d3516fef1daa145d8044594187cb11959f8be/src/common.rs#L953L972):

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

嵌入式 API 服务器不受保护，也不受 DDOS 攻击保护。一个好的做法是在 API 服务器前面使用反向代理。NGINX 是实现此目的的不错选择。HAProxy 也是一个不错的选择。\
我们在生产环境中的 API 服务器前面使用 HAProxy。
这是我们的 HAProxy 配置文件，仅作为示例提供。您应该根据自己的需要进行调整。

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

hbbs 服务器使用

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

# RustDesk 服务器程序

[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**下载**](https://github.com/sctgdesk/sctgdesk-server/releases)

[**手动**](https://rustdesk.com/docs/en/self-host/)

[**常见问题**](https://github.com/rustdesk/rustdesk/wiki/FAQ)

自托管您自己的 RustDesk 服务器，它是免费和开源的。

## 如何手动构建

首先，您需要有一个有效的 Rust 开发工具链和一个 Node ≥ 20 工作安装。

*   Unices（Linux、MacOS 等）：

```bash
DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3 cargo build --release
```

*   带有 cmd.exe shell 的 Windows：

```cmd
set "DATABASE_URL=sqlite://%CD%/db_v2.sqlite3" && cargo build --release
```

将在 target/release 中生成三个可执行文件。

*   hbbs - 带有 API 服务器的 RustDesk ID/Rendezvous 服务器
*   hbbr - RustDesk 中继服务器
*   rustdesk-utils - RustDesk CLI 实用程序

您可以在[释放](https://github.com/sctg-development/sctgdesk-server/releases)页。

版本 v1.1.99-40 之后发布的所有二进制文件都使用 Github Actions 进行证明。您可以通过使用`https://search.sigstore.dev/?hash=<sha256>`例如。

如果您想要额外的功能[RustDesk 服务器专业版](https://rustdesk.com/pricing.html)可能更适合您。

如果你想开发自己的服务器，[rustdesk-server-demo](https://github.com/rustdesk/rustdesk-server-demo)对您来说，可能比此 repo 更好、更简单的开始。

## Docker 镜像

Docker 镜像是自动生成的，并在每个 github 版本上发布。

这些映像是针对`ubuntu-22.04`唯一添加的主二进制文件 （`hbbr`和`hbbs`).它们可在[Docker 中心](https://hub.docker.com/r/sctg/sctgdesk-server/)替换为这些标签：

|建筑 |图片：标签 |
|--- |--- |
|AMD64 系列 |`sctg/sctgdesk-server:latest`|
|ARM64V8 |`sctg/sctgdesk-server:latest`|

您可以直接使用`docker run`使用以下命令：

```bash
docker run --name hbbs --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbr 
```

或没有`--net=host`，但 P2P 直连无法使用。

对于使用 SELinux 的系统，将`/root`由`/root:z`是容器正常运行所必需的。或者，可以完全禁用 SELinux 容器分离，并添加选项`--security-opt label=disable`.

```bash
docker run --name hbbs -p 21114:21114 -p 21115:21115 -p 21116:21116 -p 21116:21116/udp -p 21118:21118 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr -p 21117:21117 -p 21119:21119 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-serverlatest hbbr 
```

这`relay-server-ip`parameter 是运行这些容器的服务器的 IP 地址（或 DNS 名称）。这**自选**`port`参数，如果使用的端口不同于**21117**为`hbbr`.

您还可以使用 docker-compose，使用此配置作为模板：

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

编辑第 16 行以指向您的中继服务器（侦听端口 21117 的服务器）。如果需要，您还可以编辑体积行（第 18 行和第 33 行）。

（docker-compose 功劳归于 @lukebarone 和 @QuiGonLeong）

> 注意，这里中国的 sctg/sctgdesk-server-server：latest 可能会替换为 dockerhub 上的最新版本号，例如 sctg/sctgdesk-server-server：1.1.99-37。否则，可能会因镜像加速而拉取旧版本。

## 如何创建密钥对

加密需要密钥对;如前所述，您可以提供它，但您需要一种方法来创建一个。

您可以使用此命令生成密钥对：

```bash
/usr/bin/rustdesk-utils genkeypair
```

如果您没有（或不想要）`rustdesk-utils`软件包，您可以使用 Docker 调用相同的命令：

```bash
docker run --rm --entrypoint /usr/bin/rustdesk-utils  sctg/sctgdesk-server-server:latest genkeypair
```

输出将如下所示：

```text
Public Key:  8BLLhtzUBU/XKAH4mep3p+IX4DSApe7qbAwNH9nv4yA=
Secret Key:  egAVd44u33ZEUIDTtksGcHeVeAwywarEdHmf99KM5ajwEsuG3NQFT9coAfiZ6nen4hfgNICl7upsDA0f2e/jIA==
```

## 包

每个二进制文件都有单独的 .deb 包，您可以在[释放](https://github.com/sctg-development/sctgdesk-server/releases).
这些软件包适用于以下发行版：

*   Ubuntu 22.04 LTS
*   MacOS Intel 或 Apple Silicon
*   Windows x86\_64 或 i686

## ENV 变量

hbbs 和 hbbr 可以使用这些 ENV 变量进行配置。
您可以像往常一样指定变量，也可以使用`.env`文件。

|变量 |二进制 |描述 |
|--- |--- |--- |
|ALWAYS_USE_RELAY |HBBS |如果设置为**“Y”**不允许直接对等连接 |
|DOWNGRADE_START_CHECK |HBBBR |降级检查前的延迟（以秒为单位） |
|DOWNGRADE_THRESHOLD |HBBBR |降级检查阈值 （bit/ms） |
|密钥 |HBBS/HBBR |如果设置为 ，则强制使用特定密钥，如果设置为**"\_"**强制使用任意键 |
|LIMIT_SPEED |HBBBR |速度限制（Mb/s） |
|OAUTH2\_CONFIG_FILE |HBBS |OAuth2 配置文件的路径 |
|OAUTH2\_CREATE_USER |HBBS |如果设置为**"1"**创建不存在的用户 |
|端口 |HBBS/HBBR |侦听端口（HBB 为 21116 - HBBR 为 21117）|
|继电器 |HBBS |运行 hbbr 的计算机的 IP 地址/DNS 名称（以逗号分隔） |
|RUST_LOG |全部 |设置调试级别 （error|warn|info|debug|trace） |
|S3CONFIG_FILE |HBBS |S3 配置文件的路径 |
|SINGLE_BANDWIDTH |HBBBR |单个连接的最大带宽（以 Mb/s 为单位） |
|TOTAL_BANDWIDTH |HBBBR |最大总带宽（以 Mb/s 为单位） |
