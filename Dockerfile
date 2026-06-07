# -- Stage de Build (GraalVM Java 25) --
FROM ghcr.io/graalvm/native-image-community:25 AS builder

WORKDIR /app

# Copia Arquivos Necessários para o Build
COPY mvnw .
COPY .mvn/ .mvn/
COPY pom.xml .

# Permissão de execução no Wrapper mvnw
RUN chmod +x mvnw

# Baixa Dependências no Modo Nativo
RUN ./mvnw -Pnative -DskipTests dependency:go-offline

# Copia o Código-Fonte para o Container
COPY src ./src

# Args de build
ARG AUTH_SERVER_URL

# Compila a Aplicação para um Binário Nativo
RUN AUTH_SERVER_URL="${AUTH_SERVER_URL}" \
    ./mvnw -Pnative -DskipTests native:compile

# -- Stage Runtime (Debian Slim) --
FROM debian:bookworm-slim

WORKDIR /app

# Instala o Curl para Health Checks e Limpa Cache do APT
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

# Cria um user non-root
RUN groupadd -r appgroup && useradd -r -g appgroup appuser

# Copia o Binário Nativo do Stage Build para o Stage Runtime
COPY --from=builder --chown=appuser:appgroup /app/target/humanizar-units /app/app-binario

EXPOSE 9095
HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 CMD ["curl", "-fsS", "http://localhost:9095/actuator/health"]
USER appuser

# Ponto de Entrada no Docker, Executa o Binário Nativo com Configurações de TTL para DNS
ENTRYPOINT ["/app/app-binario", "-Dsun.net.inetaddr.ttl=30", "-Dsun.net.inetaddr.negative.ttl=2"]