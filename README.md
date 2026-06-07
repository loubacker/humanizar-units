<div align="center">
  <h1>Humanizar - Units (Microservice)</h1>
  <p>Gestão das unidades da clínica no ecossistema Humanizar.</p>

  <img alt="Java" src="https://img.shields.io/badge/Java-25-ED8B00?style=for-the-badge&logo=openjdk&logoColor=white" />
  <img alt="Spring Boot" src="https://img.shields.io/badge/Spring_Boot-4.0.5-6DB33F?style=for-the-badge&logo=spring-boot&logoColor=white" />
  <img alt="GraalVM" src="https://img.shields.io/badge/GraalVM_Native-25-E76F00?style=for-the-badge&logo=oracle&logoColor=white" />
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white" />
</div>

<br/>

Serviço REST síncrono responsável pelo CRUD de unidades da clínica Humanizar, organizado por município em modelo **multi-tenant** (município = tenant). Protegido por OAuth2 JWT com controle de acesso baseado em roles (RBAC), persistência via JPA/Hibernate e opção de runtime em binário nativo (GraalVM Native Image).

## Arquitetura e Padrões

- Arquitetura MVC (`controller`, `service`, `repository`, `model`).
- Multi-tenancy por coluna discriminadora `tenant_id` em `units`, com `municipioId` informado **explicitamente** nas rotas (sem `@TenantId` e sem resolução automática por claim de JWT).
- DTOs imutáveis com Java Records para contratos de entrada e saída.
- Mappers manuais com validação de campos obrigatórios (`UnitMapper`, `MunicipioMapper`).
- Exception handler global com respostas padronizadas (`UnitExceptionHandler`).
- Controle de acesso por role via `@PreAuthorize` (RBAC).
- Retry automático em falhas transientes de banco nos endpoints GET (`@Retry`).
- Execução otimizada com Virtual Threads e opção de runtime em binário nativo (GraalVM Native Image).

## Multi-tenancy (município = tenant)

- Cada **município** é um tenant; um município registra **N unidades** (1:N).
- Isolamento por coluna discriminadora `tenant_id` em `units` (mapeada do campo `municipioId` da entidade `Units`).
- O `municipioId` é informado **explicitamente** como path variable em todas as rotas de unidades (`/api/v1/municipio/{municipioId}/units/...`).
- `Municipio` é dado mestre **global** (não escopado por tenant); `codigo_ibge` é a chave natural única.
- CNPJ é único por tenant (`uk_units_tenant_cnpj` em `tenant_id` + `cnpj`); leituras por tenant são indexadas (`idx_units_tenant`).
- A rota `GET /api/v1/units?ids=...` é a exceção **cross-tenant**: resolve unidades por id sem exigir município (ver Interfaces).

## Interfaces internas protegidas (REST)

Base path: `/api/v1`

### Unidades (escopadas por município)

- `GET /municipio/{municipioId}/units`
    - Lista as unidades do município.
    - Autorização: qualquer usuário autenticado.
    - Retry automático em falhas transientes de banco (`@Retry`, max 2, timeout 30s).
    - Response: `200 OK` com `List<UnitDTO>`.
- `POST /municipio/{municipioId}/units/register`
    - Cria uma nova unidade no município.
    - Body obrigatório: `UnitDTO`.
    - Autorização: `ROLE_ADMINISTRADOR`.
    - Response: `201 Created` com `UnitDTO`.
- `PUT /municipio/{municipioId}/units/update/{unitId}`
    - Atualiza uma unidade existente do município.
    - Body obrigatório: `UnitDTO`. Path variables: `municipioId`, `unitId` (UUID).
    - Autorização: `ROLE_ADMINISTRADOR`.
    - Response: `200 OK` com `UnitDTO`.
- `DELETE /municipio/{municipioId}/units/delete/{unitId}`
    - Remove uma unidade existente do município.
    - Path variables: `municipioId`, `unitId` (UUID).
    - Autorização: `ROLE_ADMINISTRADOR`.
    - Response: `200 OK`.

### Unidades por id (lookup cross-tenant)

- `GET /units?ids={uuid1},{uuid2},...`
    - Resolve unidades por lista de ids, **sem** exigir município.
    - Usado por consumidores que armazenam apenas `unitId` (ex.: cadastro de paciente).
    - Autorização: qualquer usuário autenticado. Retry transiente (`@Retry`).
    - Response: `200 OK` com `List<UnitDTO>` (lista vazia quando `ids` ausente ou vazio).

### Municípios

- `GET /municipio`
    - Lista os municípios cadastrados.
    - Autorização: qualquer usuário autenticado.
    - Response: `200 OK` com `List<MunicipioDTO>`.
- `GET /municipio/{municipioId}`
    - Obtém um município por id.
    - Autorização: qualquer usuário autenticado.
    - Response: `200 OK` com `MunicipioDTO`.
- `POST /municipio/register`
    - Cadastra um novo município (tenant).
    - Body obrigatório: `MunicipioDTO` (`codigoIbge`, `nome`, `uf`).
    - Autorização: `ROLE_ADMINISTRADOR`.
    - Response: `201 Created` com `MunicipioDTO`.
- `PUT /municipio/update/{municipioId}`
    - Atualiza um município.
    - Autorização: `ROLE_ADMINISTRADOR`.
    - Response: `200 OK` com `MunicipioDTO`.
- `DELETE /municipio/delete/{municipioId}`
    - Remove um município sem unidades vinculadas.
    - Autorização: `ROLE_ADMINISTRADOR`.
    - Response: `200 OK` (ou `409 MUNICIPIO_HAS_UNITS` se houver unidades vinculadas).

> Leitura requer apenas autenticação; escrita exige `ROLE_ADMINISTRADOR`.

## ⛓️‍💥 Resiliência e Tolerância a Falhas

### Retry transiente (endpoints GET)

`@Retry` via `ResilientMethodsConfig` (Spring Framework 7 `@Retryable`).

- Max retries: 2, timeout: 30s.
- Predicate: `TransientDataAccessException`, `RecoverableDataAccessException`, `CannotCreateTransactionException`, `QueryTimeoutException`.

### Connection pool (HikariCP)

- Pool name: `humanizar-units-service`.
- Connection timeout: 30s, idle timeout: 300s, max lifetime: 600s.
- Min idle: 3, max pool size: 10.

### Exception handler global

`UnitExceptionHandler` captura exceções e retorna `UnitErrorResponseDTO` com status, reason code, mensagem, path e timestamp.

Códigos de erro mapeados (`ReasonCode`):
- `UNIT_NOT_FOUND` (404) — não retentável.
- `MUNICIPIO_NOT_FOUND` (404) — não retentável.
- `MUNICIPIO_DUPLICATED` (409) — não retentável.
- `MUNICIPIO_HAS_UNITS` (409) — não retentável.
- `TENANT_MISSING` (400) — não retentável.
- `VALIDATION_ERROR` (400) — não retentável.
- `AUTHENTICATION_FAILURE` (401) — não retentável.
- `AUTHORIZATION_FAILURE` (403) — não retentável.
- `PERSISTENCE_FAILURE` (503) — retentável.

## 🔐 Segurança

- API interna protegida por OAuth2 Resource Server JWT.
- JWK configurado por `AUTH_SERVER_URL` (`${AUTH_SERVER_URL}/oauth2/jwks`).
- RBAC: operações de escrita requerem `ROLE_ADMINISTRADOR`; leitura requer apenas autenticação.
- CORS restrito a origens localhost.
- Sessão stateless (sem estado no servidor).
- Único endpoint público: `/actuator/health`.

## Estrutura do Projeto

```text
src/main/java/com/humanizar/units/
|-- config/                        # CorsConfig, SecurityConfig, ObjectMapperConfig, ResilientMethodsConfig
|-- controller/
|   |-- dto/                       # UnitErrorResponseDTO
|   |-- handler/                   # UnitExceptionHandler
|   |-- municipio/                 # MunicipioController{Create,Retrieve,Update,Delete}
|   `-- units/                     # UnitsController{Create,Retrieve,BatchRetrieve,Update,Delete}
|-- dto/                           # UnitDTO, MunicipioDTO (Java Records)
|-- exception/                     # UnitException, Throwables (utilitario de causa)
|-- mapper/                        # UnitMapper, MunicipioMapper (validação e conversão)
|-- model/                         # entidades Units, Municipio
|   `-- enums/                     # ReasonCode
|-- repository/                    # UnitsRepository, MunicipioRepository (JPA)
`-- service/
    |-- municipio/                 # MunicipioService{Create,Retrieve,Update,Delete}
    `-- units/                     # UnitsService{Create,Retrieve,Update,Delete}
```

## Como executar localmente

### Pré-requisitos
- JDK 25
- Maven 3.9+
- PostgreSQL

### Variáveis de Ambiente (`.env`)

```env
DB_URL=jdbc:postgresql://localhost:5432/db
DB_USERNAME=postgres
DB_PASSWORD=secret
AUTH_SERVER_URL=http://localhost:8080
```

### Execução local (JVM)

```bash
./mvnw clean install -DskipTests
./mvnw spring-boot:run
```

Porta padrão: `9095`
Health check: `http://localhost:9095/actuator/health`

## 🐳 Docker Native (GraalVM)

O Dockerfile do módulo usa build multi-stage com GraalVM Native Image:

1. Build stage (`ghcr.io/graalvm/native-image-community:25`) compila com:
   - `./mvnw -Pnative -DskipTests native:compile`
2. Runtime stage (`debian:bookworm-slim`) executa binário nativo:
   - `/app/app-binario`

Exemplo:

```bash
docker build -t humanizar-units:native .
docker run --rm -p 9095:9095 --env-file .env humanizar-units:native
```
