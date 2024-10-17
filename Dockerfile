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

# Atualize e instale algumas ferramentas básicas
RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm git vim
RUN pacman -Scc --noconfirm

# Copie o binário do builder para a imagem final
COPY --from=builder /usr/src/smart_calculator/target/release/smart_calculator /usr/local/bin/smart_calculator

# Exponha a porta usada pela aplicação (modifique conforme necessário)
EXPOSE 3000

# Execute o binário
CMD ["/usr/local/bin/smart_calculator"]
