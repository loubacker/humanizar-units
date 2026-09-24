<div align="center">
  <h1>Humanizar - Units (Microservice)</h1>
  <p>Gestão de municípios e unidades da clínica Humanizar em Rust, com API interna protegida e persistência PostgreSQL assíncrona.</p>

  <img alt="Rust" src="https://img.shields.io/badge/Rust-1.98-000000?style=for-the-badge&logo=rust&logoColor=white" />
  <img alt="Axum" src="https://img.shields.io/badge/Axum-0.8-7B1FA2?style=for-the-badge&logo=rust&logoColor=white" />
  <img alt="Tokio" src="https://img.shields.io/badge/Tokio-1.53-2C5BB4?style=for-the-badge&logo=rust&logoColor=white" />
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white" />
  <img alt="Docker" src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white" />
  <img alt="Keycloak" src="https://img.shields.io/badge/Keycloak-4D4D4D?style=for-the-badge&logo=keycloak&logoColor=white" />
</div>

<br/>

Serviço REST síncrono responsável pelo CRUD de municípios e unidades. A implementação preserva os contratos HTTP do serviço legado, com payloads de sucesso diretos, autenticação Bearer JWT, autorização por role e tratamento padronizado de erros. O runtime usa Axum e Tokio, acesso assíncrono ao PostgreSQL e composição explícita das dependências segundo Arquitetura Hexagonal.

## Arquitetura e Padrões

- Arquitetura Hexagonal organizada em `application`, `domain` e `infrastructure`.
- Domínio independente de Axum, Tokio, PostgreSQL, Serde e detalhes HTTP.
- DTOs imutáveis com campos privados, construtores e getters explícitos.
- Mappers manuais para as conversões DTO ↔ domínio e domínio ↔ entity.
- Ports de domínio implementados por adapters de infraestrutura, injetados com `Arc<dyn Trait>`.
- Repositories com SQL explícito por meio de `tokio-postgres`.
- Pool assíncrono com `bb8-postgres`, fila FIFO e validação da conexão no checkout.
- Retry assíncrono somente para leituras idempotentes, com BackON e classificação explícita de falhas PostgreSQL.
- Erros de domínio modelados por `ReasonCode` e transportados pelo handler HTTP genérico.
- Autenticação e autorização aplicadas por composição de routers do Axum.
- Graceful shutdown para `SIGTERM` e `Ctrl+C`.
- Código próprio protegido por `#![forbid(unsafe_code)]`, sem FFI ou bindings nativos.

## Regras de Domínio

### Município

- O código IBGE único.
- O identificador enviado no body não substitui o identificador definido no path.

### Unidade

- A unidade deve estar vinculada a um município.
- O CNPJ único.
- Municípios diferentes podem possuir unidades com o mesmo CNPJ.
- Update e delete exigem a combinação correta de `unitId` e `municipioId`.

## Interfaces Internas Protegidas (REST)

Base path: `/api/v1`

### Município

- `GET /municipio` — lista os municípios. Requer autenticação.
- `GET /municipio/{municipioId}` — retorna um município. Requer autenticação.
- `POST /municipio/register` — cria um município. Exige `ADMINISTRADOR`.
- `PUT /municipio/update/{municipioId}` — atualiza um município. Exige `ADMINISTRADOR`.
- `DELETE /municipio/delete/{municipioId}` — remove um município. Exige `ADMINISTRADOR`.

### Unidade

- `GET /municipio/{municipioId}/units` — lista as unidades do município. Requer autenticação.
- `GET /units?ids={unitId1},{unitId2}` — busca unidades por IDs. Requer autenticação.
- `POST /municipio/{municipioId}/units/register` — cria uma unidade. Exige `ADMINISTRADOR`.
- `PUT /municipio/{municipioId}/units/update/{unitId}` — atualiza uma unidade. Exige `ADMINISTRADOR`.
- `DELETE /municipio/{municipioId}/units/delete/{unitId}` — remove uma unidade. Exige `ADMINISTRADOR`.

Endpoint público:

- `GET /health` — retorna `200` com `{"status":"UP"}`.

As respostas de sucesso permanecem diretas para compatibilidade com os consumidores existentes. Erros usam o envelope compartilhado com `timestamp`, `status`, `error`, `reasonCode`, `message` e `path`.

## 🔐 Segurança

- Autenticação stateless exclusivamente por `Authorization: Bearer <jwt>`.
- Tokens aceitos somente com assinatura RSA e algoritmo `RS256`.
- Validação de `kid`, assinatura, `exp`, `nbf`, `iss`, `aud` e `sub`.
- JWKS carregado diretamente da URL completa configurada em `KEYCLOAK_ISSUER` durante o startup; nenhum sufixo é acrescentado.
- `KEYCLOAK_ISSUER` não pode conter credenciais, query ou fragmento.
- Consulta JWKS com timeout de conexão e de resposta explícitos, no startup e nas atualizações.
- Cache JWKS concorrente com atualização controlada para rotação de chaves.
- Claims Keycloak `azp` e `realm_access.roles` são consumidas nativamente; `client_id`, `role` e `roles` permanecem compatíveis com tokens legados.
- Leituras exigem usuário autenticado; escritas exigem `ADMINISTRADOR`.
- Respostas `401` usam `AUTHENTICATION_FAILURE` e respostas `403` usam `AUTHORIZATION_FAILURE`.
- Bearer token, credenciais, senha e conteúdo completo do JWT não são registrados em log.

## ⛓️‍💥 Resiliência

### Retry de leitura

- Uma tentativa inicial e até duas novas tentativas.
- Backoff exponencial e jitter.
- Timeout, incluindo execução e esperas.
- Retry somente para falhas transitórias.
- Escritas não possuem retry automático.

### ReasonCode

| Status | ReasonCode |
|--------|------------|
| `400` | `INVALID_REQUEST`, `VALIDATION_ERROR` |
| `401` | `AUTHENTICATION_FAILURE` |
| `403` | `AUTHORIZATION_FAILURE` |
| `404` | `UNIT_NOT_FOUND`, `MUNICIPIO_NOT_FOUND` |
| `409` | `UNIT_DUPLICATED`, `MUNICIPIO_DUPLICATED`, `MUNICIPIO_HAS_UNITS` |
| `500` | `UNEXPECTED_ERROR` |
| `503` | `PERSISTENCE_FAILURE` |

Erros `4xx` são registrados como `warn`; erros `5xx`, como `error`. A causa técnica permanece apenas no encadeamento e nos logs, nunca no JSON público.

## Diagnóstico de Startup

O bootstrap é executado em etapas ordenadas, e cada etapa registra o destino configurado antes de tentar usá-lo:

1. Configuração de ambiente, servidor e retry.
2. Pool PostgreSQL, com `banco`, `usuario`, tamanhos e timeout de conexão.
3. Cache JWKS, com `jwks`, `emissor`, `audiencia` e timeouts HTTP.
4. Listener HTTP, com `host` e `port`.

## Estrutura do Projeto

```text
src/
|-- application/
|   |-- dto/                       # contratos imutáveis de município e unidade
|   |-- mapper/                    # validação e mappings manuais
|   |-- service/                   # orquestração DTO ↔ use case
|   `-- usecase/                   # regras de aplicação por agregado
|-- domain/
|   |-- exception/                 # exceções e contratos de ReasonCode
|   |-- model/                     # Unit, Municipio e enums
|   `-- port/                      # UnitPort e MunicipioPort
`-- infrastructure/
|   |-- adapter/                   # implementação dos ports
|   |-- config/                    # ambiente, servidor, JWT, pool e retry
|   |-- controller/                # controllers, extractors, DTOs HTTP e router
|   |-- diagnostics/               # URL sanitizada e relatório de falha de startup
|   |-- handler/                   # classificação de erros PostgreSQL
|   |-- persistence/               # entities e repositories SQL
|   |-- resilience/                # executor de retry assíncrono
|   |-- security/                  # claims, JWKS, validação e autorização
|   `-- server.rs                  # bootstrap e graceful shutdown
|-- lib.rs                         # declaração e exposição dos módulos
|-- main.rs                        # entrada Tokio sem lógica de composição

tests/
|-- application/
|-- domain/
|-- infrastructure/
`-- support/
```

O projeto não utiliza `mod.rs`. Os módulos públicos de primeiro nível são organizados em `lib.rs`, e os arquivos de produção não contêm módulos de teste.

## Como Executar Localmente

### Pré-requisitos

- Rust `1.98+` com Cargo.
- PostgreSQL acessível pela aplicação.
- Keycloak com endpoint JWKS disponível.

O serviço não cria tabelas nem executa migrations automaticamente. Antes do startup, o schema deve conter `public.municipio` e `public.units`, com `units.municipio_id` referenciando `municipio.id`.

### Execução local
```b
cargo run
```
Porta padrão: `9095`
Health check: `http://localhost:9095/health`

### Quality gates
```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check
```
## 🐳 Docker

O Dockerfile usa build multi-stage:

1. `rust:1.98.0-bookworm` compila o binário release com `cargo build --locked --release`.
2. `debian:bookworm-slim` executa `/app/humanizar-units` com usuário sem privilégios.
3. O health check consulta `/health` a cada 30 segundos.

```bash
docker build -t humanizar-units .
docker run --rm -p 9095:9095 --env-file .env humanizar-units:latest
```

O `.env` não é copiado para a imagem: as variáveis chegam por `--env-file` ou pelo orquestrador. Falhas de configuração encerram o container com código `1`, e `docker logs` mostra a etapa que falhou, o destino configurado e a cadeia técnica descrita em [Diagnóstico de Startup](#diagnóstico-de-startup).
