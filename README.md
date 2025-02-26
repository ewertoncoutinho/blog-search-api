# Blog-api

Esta API atua como um proxy reverso para o Meilisearch, garantindo uma comunicação segura, controlando acessos e fornecer uma camada adicional de proteção e otimização.

## Requisitos

Certifique-se de ter os seguintes requisitos instalados:

- [Rust](https://www.rust-lang.org/) (versão 1.85 ou superior)
- [Docker](https://www.docker.com/)
- [Meilisearch](https://www.meilisearch.com/)

## Como Rodar a Aplicacao

### Rodando Localmente

1. Clone o repositório:

   ```sh
   git clone https://github.com/ewertoncoutinho/blog-api.git
   cd blog-api
   ```sh

2. Compile e execute a aplicacao:

   ```sh
   cargo run
   ```

### Rodando com Docker

1. Construa a imagem Docker:

   ```sh
   docker build -t blog-api .
   ```

2. Execute o container:

   ```sh
   docker run --name blog-api -e MEILI_API_KEY= -e MEILI_URL= --network meilinet -p 8080:8080 blog-api
   ```

## Estrutura do Projeto

``` sh
├── src/
│   └── main.rs
├── .env.example
├── .gitignore
├── Cargo.toml
├── Dockerfile
└── README.md
```

## Contribuindo

1. Fork este repositório
2. Crie um branch (`git checkout -b minha-feature`)
3. Commit suas mudanças (`git commit -am 'Adiciona nova funcionalidade'`)
4. Envie para o branch (`git push origin minha-feature`)
5. Abra um Pull Request

## Licenca

Este projeto está licenciado sob a [MIT License](LICENSE).
