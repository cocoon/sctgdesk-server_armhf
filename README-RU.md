<p align="center">
  <a href="#how-to-build-manually">Manually</a> •
  <a href="#docker-images">Docker</a> •
  <a href="#how-to-create-a-keypair">Keypair</a> •
  <a href="#packages">Binaries</a> •
  <a href="#env-variables">Variables</a><br>
  [<a href="README-FR.md">French</a>] | [<a href="README-DE.md">Deutsch</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-TW.md">繁體中文</a>] | [<a href="README-ZH.md">简体中文</a>] | [<a href="README-RU.md">русски</a>]<br>
</p>

# Серверная программа SctgDesk

[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**Двоичная загрузка**](https://github.com/sctg-development/sctgdesk-server/releases)

[**Документация по API**](https://sctg-development.github.io/sctgdesk-api-server/)

Это модифицированная версия RustDesk Server, которая является бесплатной и имеет открытый исходный код.

*   Первое отличие заключается в том, что эта версия включает в себя новую *протокол tcp* режим включен в версию RustDesk Server Pro.
*   Второе отличие заключается в том, что эта версия включает в себя предварительную реализацию сервера API Rustdesk Server Pro.
    *   Поддержка персональной адресной книги
    *   Поддержка общей адресной книги на уровне группы
        *   Только для чтения, Чтение-запись, Admin
    *   Поддержка общей адресной книги на уровне пользователя
        *   Только для чтения, Чтение-запись, Admin
*   Третье отличие заключается в том, что эта версия включает в себя предварительную реализацию простой веб-консоли.

Веб-консоль доступна по адресу `http://<server-ip>:21114/` с логином "admin" и паролем "Hello,world!" .\
Вы можете ознакомиться с документацией API на встроенном сервере API по адресу `http://<server-ip>:21114/api/doc/`.

Неинтерактивная документация по API доступна по адресу [sctgdesk-api-server repo](https://sctg-development.github.io/sctgdesk-api-server/).

## ТЛ; ДОКТОР

Вы можете использовать следующие `docker-compose.yml` Файл для запуска сервера:

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

и запустите сервер с:

```bash
mkdir -p data
docker-compose up 
```

### Пользователь admin по умолчанию

Пользователь admin по умолчанию создается с именем пользователя `admin` и пароль `Hello,world!`. Вы можете изменить пароль после первого входа в веб-консоль.

## API Автономная версия

Автономная версия API — это версия сервера, которая включает в себя сервер API и веб-консоль, но не сервер рандеву.\
Автономная версия доступна в собственном репозитории [sctgdesk-api-сервер](https://github.com/sctg-development/sctgdesk-api-server).\
По всем вопросам, связанным с API или веб-консолью, обращайтесь к [sctgdesk-api-сервер](https://github.com/sctg-development/sctgdesk-api-server) хранилище.

## Скриншоты

### Веб-консоль

<img width="1085" alt="login" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/fe72a374-8a98-4606-8632-3d919f9317c9">

<img width="1285" alt="dashboard" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/0bb148d6-8723-491f-88c5-b98331d64f61">

<img width="1085" alt="devices" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/6ae55861-f65c-4950-a068-f22eef3ad81a">

<img width="1084" alt="users" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/8d225841-43f5-44f4-8d41-5b6ca3324096">

<img width="1087" alt="groups" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/d84ce3d3-1d19-4765-883f-001f313a4a1e">

<img width="1089" alt="address books" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/db13010b-077a-4e14-943b-9d8de3266f82">

<img width="730" alt="rues" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/3a990deb-d8bb-4725-a47d-435ec3667fee">

<img width="621" alt="add rules" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/355f3903-2b54-4b08-abd0-e33c84a260ed">

### Документация по API

<img width="1502" alt="apidoc" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/88fe7910-fe62-43e5-a16c-70dc1201e040">

### Использование в клиенте Rustdesk

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## Генерация ссылок автообновления

Мы модифицировали наш клиент, чтобы получать ссылки на автообновление с сервера API, а не из релизов Github.\
Чтобы ссылки автообновления работали, вам необходимо изменить свой клиент, чтобы он получал ссылки автообновления с сервера API. Этот [Как это можно сделать](https://github.com/sctg-development/sctgdesk/blob/481d3516fef1daa145d8044594187cb11959f8be/src/common.rs#L953L972):

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

# Безопасность

Встроенный сервер API не защищен от DDOS-атак. Хорошей практикой является использование обратного прокси перед сервером API. NGINX — хороший выбор для этой цели. HAProxy также является хорошим выбором.\
Мы используем HAProxy перед сервером API в нашей производственной среде.
Это наш конфигурационный файл для HAProxy, он предоставлен только в качестве примера. Вы должны адаптировать его под свои собственные нужды:

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

Сервер hbbs запускается с помощью

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

# Серверная программа RustDesk

[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**Загружать**](https://github.com/sctgdesk/sctgdesk-server/releases)

[**Вручную**](https://rustdesk.com/docs/en/self-host/)

[**Вопросы и ответы**](https://github.com/rustdesk/rustdesk/wiki/FAQ)

Самостоятельно разместите свой собственный сервер RustDesk, он бесплатный и с открытым исходным кодом.

## Как собрать вручную

Для начала вам нужно иметь работающий набор инструментов для разработки Rust и рабочую установку Node ≥ 20.

*   Интерфейсы (Linux, MacOS и т.д.):

```bash
DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3 cargo build --release
```

*   Окна с cmd.exe оболочкой:

```cmd
set "DATABASE_URL=sqlite://%CD%/db_v2.sqlite3" && cargo build --release
```

Три исполняемых файла будут сгенерированы в target/release.

*   hbbs - сервер RustDesk ID/Rendezvous с сервером API
*   hbbr - релейный сервер RustDesk
*   rustdesk-utils - Утилиты RustDesk CLI

Вы можете найти обновленные двоичные файлы на странице [Релизы](https://github.com/sctg-development/sctgdesk-server/releases) страница.

Все выпущенные двоичные файлы после релиза v1.1.99-40 аттестованы с помощью Github Actions. Вы можете проверить аттестацию, проверив sha256sum двоичного файла с помощью `https://search.sigstore.dev/?hash=<sha256>` Например.

Если вам нужны дополнительные функции [RustDesk Server Pro](https://rustdesk.com/pricing.html) может вам больше подойти.

Если вы хотите разработать собственный сервер, [rustdesk-server-demo](https://github.com/rustdesk/rustdesk-server-demo) Возможно, это будет лучшим и более простым началом для вас, чем этот репозиторий.

## Образы Docker

Образы Docker автоматически создаются и публикуются в каждом выпуске github.

Эти образы созданы на фоне `ubuntu-22.04` с единственным добавлением основных двоичных файлов (`hbbr` и `hbbs`). Они доступны на [Docker hub](https://hub.docker.com/r/sctg/sctgdesk-server/) с такими тегами:

| архитектура | Изображение:Тег |
| --- | --- |
| АМД64 | `sctg/sctgdesk-server:latest` |
| АРМ64В8 | `sctg/sctgdesk-server:latest` |

Вы можете начать работу с этими изображениями непосредственно с `docker run` С помощью этих команд:

```bash
docker run --name hbbs --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbr 
```

или без `--net=host`, но прямое подключение P2P работать не может.

Для систем, использующих SELinux, замена `/root` около `/root:z` требуется для правильной работы контейнеров. В качестве альтернативы, разделение контейнеров SELinux можно полностью отключить, добавив опцию `--security-opt label=disable`.

```bash
docker run --name hbbs -p 21114:21114 -p 21115:21115 -p 21116:21116 -p 21116:21116/udp -p 21118:21118 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr -p 21117:21117 -p 21119:21119 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-serverlatest hbbr 
```

Тем `relay-server-ip` parameter — это IP-адрес (или DNS-имя) сервера, на котором работают эти контейнеры. Тем **необязательный** `port` необходимо использовать, если вы используете порт, отличный от **21117** для `hbbr`.

Вы также можете использовать docker-compose, используя эту конфигурацию в качестве шаблона:

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

Отредактируйте строку 16 так, чтобы она указывала на ваш ретрансляционный сервер (тот, который прослушивает порт 21117). Вы также можете отредактировать линии объема (строка 18 и строка 33), если вам это нужно.

(Авторство Docker-Compose принадлежит @lukebarone и @QuiGonLeong)

> Обратите внимание, что здесь sctg/sctgdesk-server-server:latest в Китае может быть заменен на номер последней версии на dockerhub, например, sctg/sctgdesk-server-server:1.1.99-37. В противном случае старая версия может быть подтянута из-за ускорения изображения.

## Как создать пару ключей

Пара ключей нужна для шифрования; Вы можете предоставить его, как объяснялось ранее, но вам нужен способ его создания.

Вы можете использовать эту команду для создания пары ключей:

```bash
/usr/bin/rustdesk-utils genkeypair
```

Если у вас нет (или вы не хотите) `rustdesk-utils` Пакет, установленный в вашей системе, вы можете вызвать ту же команду с Docker:

```bash
docker run --rm --entrypoint /usr/bin/rustdesk-utils  sctg/sctgdesk-server-server:latest genkeypair
```

На выходе получится примерно следующее:

```text
Public Key:  8BLLhtzUBU/XKAH4mep3p+IX4DSApe7qbAwNH9nv4yA=
Secret Key:  egAVd44u33ZEUIDTtksGcHeVeAwywarEdHmf99KM5ajwEsuG3NQFT9coAfiZ6nen4hfgNICl7upsDA0f2e/jIA==
```

## Пакеты

Для каждого двоичного файла доступны отдельные пакеты .deb, вы можете найти их в разделе [Релизы](https://github.com/sctg-development/sctgdesk-server/releases).
Эти пакеты предназначены для следующих дистрибутивов:

*   Ubuntu 22.04 LTS
*   MacOS, Intel или Apple Silicon
*   Windows x86\_64 или i686

## Переменные ENV

hbbs и hbbr могут быть сконфигурированы с использованием этих переменных ENV.
Вы можете указать переменные как обычно или использовать команду `.env` файл.

| переменная | Двоичный | Описание |
| --- | --- | --- |
| ALWAYS_USE_RELAY | ХББС | если установлено значение **«У»** Запрещает прямое одноранговое соединение |
| DOWNGRADE_START_CHECK | ХББР | задержка (в секундах) перед проверкой на понижение версии |
| DOWNGRADE_THRESHOLD | ХББР | порог проверки на понижение версии (бит/мс) |
| КЛЮЧ | ГББС/ГББР | если установлено, принудительно использовать определенный ключ, если установлено значение **"\_"** Принудительное использование любой клавиши |
| LIMIT_SPEED | ХББР | ограничение скорости (в Мб/с) |
| OAUTH2\_CONFIG_FILE | ХББС | Путь к файлу конфигурации OAuth2 |
| OAUTH2\_CREATE_USER | ХББС | если установлено значение **"1"** Создание пользователя, если его нет |
| ПОРТ | ГББС/ГББР | Порт прослушивания (21116 для HBBS - 21117 для HBBR) |
| РЕЛЕ | ХББС | IP-адрес/DNS-имя компьютеров под управлением hbbr (через запятую) |
| RUST_LOG | все | установить уровень отладки (error|warn|info|debug|trace) |
| S3CONFIG_FILE | ХББС | Путь к файлу конфигурации S3 |
| SINGLE_BANDWIDTH | ХББР | максимальная пропускная способность для одного соединения (в Мбит/с) |
| TOTAL_BANDWIDTH | ХББР | максимальная общая пропускная способность (в Мбит/с) |
