<div align="center">
  <h1>Humanizar - Units (Microservice)</h1>
  <p>Gestão das unidades da clínica no ecossistema Humanizar.</p>

  <img alt="Java" src="https://img.shields.io/badge/Java-25-ED8B00?style=for-the-badge&logo=openjdk&logoColor=white" />
  <img alt="Spring Boot" src="https://img.shields.io/badge/Spring_Boot-4.0.5-6DB33F?style=for-the-badge&logo=spring-boot&logoColor=white" />
  <img alt="GraalVM" src="https://img.shields.io/badge/GraalVM_Native-25-E76F00?style=for-the-badge&logo=oracle&logoColor=white" />
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white" />
</div>

<br/>

Serviço REST síncrono responsável pelo CRUD de unidades da clínica Humanizar. Protegido por OAuth2 JWT com controle de acesso baseado em roles (RBAC), persistência via JPA/Hibernate e opção de runtime em binário nativo (GraalVM Native Image).

## Arquitetura e Padrões

- Arquitetura MVC (`controller`, `service`, `repository`, `model`).
- DTOs imutáveis com Java Records para contratos de entrada e saída.
- Mapper centralizado com validação de campos obrigatórios (`UnitMapper`).
- Exception handler global com respostas padronizadas (`UnitExceptionHandler`).
- Controle de acesso por role via `@PreAuthorize` (RBAC).
- Retry automático em falhas transientes de banco (`@Retry`).
- Execução otimizada com Virtual Threads e opção de runtime em binário nativo (GraalVM Native Image).

## Interfaces internas protegidas (REST)

Base path: `/api/v1`

- `POST /unit/register`
    - Cria uma nova unidade.
    - Body obrigatório: `UnitDTO`.
    - Autorização: `ROLE_ADMINISTRADOR`.
    - Response: `201 Created` com `UnitDTO`.
- `GET /units`
    - Lista todas as unidades cadastradas.
    - Autorização: qualquer usuário autenticado.
    - Retry automático em falhas transientes de banco (`@Retry`, max 2, timeout 30s).
    - Response: `200 OK` com `List<UnitDTO>`.
- `PUT /unit/update/{unitId}`
    - Atualiza uma unidade existente.
    - Body obrigatório: `UnitDTO`.
    - Path variable: `unitId` (UUID).
    - Autorização: `ROLE_ADMINISTRADOR`.
    - Response: `200 OK` com `UnitDTO`.
- `DELETE /unit/delete/{unitId}`
    - Remove uma unidade existente.
    - Path variable: `unitId` (UUID).
    - Autorização: `ROLE_ADMINISTRADOR`.
    - Response: `200 OK`.

## ⛓️‍💥 Resiliência e Tolerância a Falhas

### Retry transiente (endpoint GET)

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
- `VALIDATION_ERROR` (400) — não retentável.
- `AUTHENTICATION_FAILURE` (401) — não retentável.
- `AUTHORIZATION_FAILURE` (403) — não retentável.
- `PERSISTENCE_FAILURE` (503) — retentável.

## 🔐 Segurança

- API interna protegida por OAuth2 Resource Server JWT.
- JWK configurado por `AUTH_SERVER_URL`.
- RBAC: operações de escrita requerem `ROLE_ADMINISTRADOR`; leitura requer apenas autenticação.
- CORS restrito a origens localhost.
- Sessão stateless (sem estado no servidor).
- Sem exposição de endpoint público para uso externo.

## Estrutura do Projeto

```text
src/main/java/com/humanizar/units/
|-- config/                        # CorsConfig, SecurityConfig, ObjectMapperConfig, ResilientMethodsConfig
|-- controller/                    # controllers CRUD (Create, Retrieve, Update, Delete)
|   |-- dto/                       # UnitErrorResponseDTO
|   `-- handler/                   # UnitExceptionHandler
|-- dto/                           # UnitDTO (Java Record)
|-- exception/                     # UnitException
|-- mapper/                        # UnitMapper (validação e conversão)
|-- model/                         # entidade Units + enums (ReasonCode)
|-- repository/                    # UnitsRepository (JPA)
`-- service/                       # services CRUD (Create, Retrieve, Update, Delete)
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
