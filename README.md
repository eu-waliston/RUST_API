# 🚀 API Rust de Alta Performance

![Rust](https://img.shields.io/badge/Rust-1.75-orange?logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-0.7-blue)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-15-blue?logo=postgresql)

Uma API **assíncrona, rápida e escalável** construída com **Rust**, **Axum**, **Tokio** e **PostgreSQL**.  
Projetada para rodar em **Linux** com foco em performance, confiabilidade e facilidade de manutenção.

---

## 🌟 Funcionalidades

- CRUD básico de itens (`/items`)  
- Endpoint de saúde (`/health`)  
- Conexão assíncrona com PostgreSQL  
- Pool de conexões eficiente para alta concorrência  
- Logging estruturado com `tracing`  
- Middleware de compressão, CORS e trace de requisições  

---

## 🛠 Tecnologias

- **Rust**: linguagem de sistemas de alta performance  
- **Tokio**: runtime assíncrono multi-thread  
- **Axum**: framework web moderno e seguro  
- **SQLx**: acesso async a banco de dados com compile-time checks  
- **PostgreSQL**: banco relacional confiável e performático  
- **Tower / Tower-HTTP**: middlewares de CORS, trace e compressão  

---

## ⚡ Rodando localmente

### 1. Clone o repositório:

```bash
git clone https://github.com/seu-usuario/api-rust.git
cd api-rust
```

### 2. Configure o banco:

```bash

export DATABASE_URL=postgres://user:password@127.0.0.1:5432/mydb

# Crie a tabela de itens
psql $DATABASE_URL -c "
CREATE TABLE IF NOT EXISTS items (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  value DOUBLE PRECISION NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);"

```

### 3 Build e run:

```bash

cargo run --release

```

A API estará disponível em http://localhost:3000.


### 📦 Endpoints principais

| Método | Endpoint    | Descrição              |
| ------ | ----------- | ---------------------- |
| GET    | /health     | Verifica status da API |
| GET    | /items      | Lista itens            |
| POST   | /items      | Cria um novo item      |
| GET    | /items/\:id | Busca item por ID      |

### Exemplo de POST:

```bash

curl -X POST http://localhost:3000/items \
-H "Content-Type: application/json" \
-d '{"name": "Item A", "value": 123.45}'

```

### 🐳 Docker

```bash

docker build -t api-rust .
docker run -p 3000:3000 -e DATABASE_URL=postgres://user:pass@host:5432/dbname api-rust

# Ou usando docker-compose (se houver arquivo):

docker-compose up

```

### 💡 Boas práticas

```bash

cargo build --release

```

- Use TLS / proxy reverso em produção

- Monitore com tracing / metrics / Prometheus

- Ajuste pool de conexões e threads conforme carga

### 🤝 Contribuindo

1) Fork o repositório

2) Crie uma branch feature: git checkout -b feature/nova-funcionalidade

3) Faça commit das mudanças: git commit -m "Descrição da mudança"

4) Push para a branch: git push origin feature/nova-funcionalidade

5) Abra um Pull Request
