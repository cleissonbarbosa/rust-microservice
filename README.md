# rust-microservice

Microserviço em Rust utilizando Actix-web.

## Descrição

Este projeto é um microserviço simples desenvolvido em Rust usando o framework Actix-web. Ele fornece uma API REST básica com endpoints para criar usuários e uma mensagem de boas-vindas.

## Endpoints

### GET /

Retorna uma mensagem de boas-vindas.

- **URL:** `/`
- **Método:** `GET`
- **Resposta de Sucesso:**
  - **Código:** `200 OK`
  - **Conteúdo:** `"Bem-vindo ao meu microserviço em Rust!"`

### POST /usuarios

Cria um novo usuário.

- **URL:** `/usuarios`
- **Método:** `POST`
- **Corpo da Requisição:**
  - **Formato:** JSON
  - **Exemplo:**
    ```json
    {
      "nome": "João Silva",
      "email": "joao.silva@example.com"
    }
    ```
- **Resposta de Sucesso:**
  - **Código:** `201 Created`

## Como Executar

1. Certifique-se de ter o Rust instalado. Você pode instalar o Rust a partir do [site oficial](https://www.rust-lang.org/).
2. Clone o repositório:
   ```sh
   git clone https://github.com/cleissonbarbosa/rust-microservice.git
   ```
3. Navegue até o diretório do projeto:
   ```sh
   cd rust-microservice
   ```
4. Execute o servidor:
   ```sh
   cargo run
   ```
5. O servidor estará disponível em `http://127.0.0.1:8080`.

## Licença

Este projeto está licenciado sob os termos das licenças MIT ou Apache-2.0. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.