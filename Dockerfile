FROM rust:1.85 AS builder

# Define o diretório de trabalho
WORKDIR /app

# Copia o Cargo.toml e Cargo.lock para o cache eficiente
COPY Cargo.toml Cargo.lock ./

# Copia o código da aplicação
COPY . .

# Compila a aplicação em modo release
RUN cargo build --release

# Etapa final: cria uma imagem enxuta
FROM debian:bookworm-slim

# Instala dependências necessárias
RUN apt-get update && apt-get install -y \
libssl3 ca-certificates libc6 && rm -rf /var/lib/apt/lists/*

# Define o diretório de trabalho
WORKDIR /app

# Copia apenas o binário compilado da etapa anterior
COPY --from=builder /app/target/release/blog-api /app/

# Expõe a porta usada pelo Actix Web
EXPOSE 8080

# Comando para rodar a aplicação
CMD ["/app/blog-api"]