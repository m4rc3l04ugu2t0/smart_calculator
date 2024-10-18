# Etapa 1: Build
FROM rust:latest AS builder

WORKDIR /usr/src/smart_calculator

COPY . .

# Instale as dependências e compile o projeto
RUN cargo install --path .
RUN cargo build --release

# Etapa 2: Final
FROM archlinux:latest

WORKDIR /usr/src

# Atualiza e instala NGINX, git, vim e outras ferramentas necessárias
RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm git vim nginx
RUN pacman -Scc --noconfirm

# Copie o binário do builder para a imagem final
COPY --from=builder /usr/src/smart_calculator/target/release/smart_calculator /usr/local/bin/smart_calculator

# Copia o arquivo de configuração do NGINX (a ser criado na próxima seção)
COPY nginx.conf /etc/nginx/nginx.conf

# Exponha a porta 3000 (da aplicação Rust) e a porta 80 (para o NGINX)
EXPOSE 3000
EXPOSE 80

# Inicia o NGINX e o binário Rust simultaneamente
CMD ["sh", "-c", "nginx && /usr/local/bin/smart_calculator"]