<h1 align="center">
    PROJETO VALIDAÇÃO DE E-MAIL
</h1>

<h4 align="center">
    API de autenticação Rust com Actix-Web
</h4>

## Índice do projeto

-   [Visão geral do projeto](#project-overview)
-   [Estrutura do Projeto](#project-structure)
-   [Instruções de configuração](#setup-instructions)
    -   [Pré-requisitos](#prerequisites)
    -   [Variáveis ​​de ambiente](#environment-variables)
    -   [Migrações de Banco de Dados](#database-migrations)
    -   [Executando o servidor](#running-the-server)
-   [Endpoints da API](#api-endpoints)
    -   [Endpoint de autenticação](#authentication-endpoints)
    -   [Endpoint do usuário](#user-endpoints)
-   [Swagger UI](#swagger-ui)
-   [Middleware](#middleware)
-   [Teste de unidade](#unit-testing)
-   [Documentação OpenAPI](#openapi-documentation)
-   [Contribuindo](#contributing)
-   [Licença](#license)
-   [Contato](#contact)

## Visão geral do projeto

Este repositório contém um projeto abrangente de API na linguagem Rust usando Actix-Web.
A API inclui autenticação robusta baseada em JWT, gerenciamento de usuários e integração de Swagger UI para consultar a documentação interativa de API.

### Principais características:

-   **Autenticação de usuário:** Registro seguro de usuário, login e logout usando JWT.
-   **Gerenciamento de usuário:** Recupere informações de usuário e liste todos os usuários.
-   **Integração de banco de dados:** PostgreSQL com SQLx para manuseio eficiente de banco de dados.
-   **Configurações de ambiente:** Gerencie configurações por meio de variáveis ​​de ambiente.
-   **Tratamento de erro:** Tratamento abrangente de erro com tipos de erro personalizados.
-   **Documentação de API:** Interface de usuário Swagger integrada para documentação de API interativa.
-   **Middleware:** Middleware personalizado para autenticação e controle de acesso baseado em função.

## Estrutura do Projeto

```plaintext
├── src
│   ├── handlers           # Manipuladores de rotas de API
│   │   ├── mod.rs         # Módulo de funções utilitárias
│   │   ├── auth.rs        # Manipuladores relacionados à autenticação
│   │   ├── users.rs       # Manipuladores relacionados ao usuário
│   ├── utils              # Funções de utilidade
│   │   ├── mod.rs         # Módulo de funções utilitárias
│   │   ├── password.rs    # Hashing e verificação de senha
│   │   ├── token.rs       # Assinatura e verificação JWT
│   ├── auth.rs            # Implementações de middleware
│   ├── config.rs          # Arquivo de configuração para variáveis ​​de ambiente
│   ├── db.rs              # Camada de acesso ao banco de dados
│   ├── dtos.rs            # Objetos de Transferência de Dados (DTOs)
│   ├── error.rs           # Módulo de tratamento de erros
│   ├── main.rs            # Ponto de entrada do aplicativo
│   ├── models.rs          # Modelos de banco de dados
├── migrations             # Pasta de migrações de banco de dados (criada pelo SQLx)
├── .env                   # Arquivo de variáveis ​​de ambiente
├── .gitignore             # Arquivo ignora a subida para github
├── docker-compose.yml     # Cria banco de dados postgres com docker no container
├── LICENSE                # Arquivo de licença de usabilidade
├── Cargo.toml             # Dependências do Rust e metadados do projeto
├── README.md              # Documentação do projeto
```

# Instruções de configuração

## Pré-requisitos

Certifique-se de ter as seguintes ferramentas instaladas:

-   [Rust](https://www.rust-lang.org/tools/install)
-   [PostgreSQL](https://www.postgresql.org/download/)
-   [Swagger UI](https://docs.rs/swagger-ui/latest/swagger_ui/)
-   [Docker](https://www.docker.com/products/docker-desktop/)
-   [SQLx-CLI](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli) (para migrações de banco de dados)

## Variáveis ​​de ambiente

Crie um arquivo `.env` no diretório raiz com o seguinte conteúdo:

```plaintext
    DATABASE_URL=postgres://username:password@localhost/dbname
    JWT_SECRET=your_jwt_secret_key
    JWT_EXPIRATION=60 # Tempo de expiração do JWT em minutos
```

Substitua os espaços reservados pelas suas credenciais reais do banco de dados e pela configuração desejada do JWT.

## Migrações de Banco de Dados

Execute o seguinte comando para executar migrações de banco de dados:

```bash
    sqlx migrate run
```

Isso configurará o esquema de banco de dados necessário para o aplicativo.

## Executando o servidor

Inicie o servidor usando o comando:

```bash
    cargo run
```

A API estará acessível no endereço: http://localhost:8000.

## Endpoints da API

### Endpoint de autenticação

-   **Registrar usuário:** `POST /api/auth/register`
-   **Login do usuário:** `POST /api/auth/login`
-   **Sair do usuário:** `POST /api/auth/logout`

### Endpoint do usuário

-   **Obter usuário autenticado:** `GET /api/users/me`
-   **Listar usuários:** `GET /api/users`

Cada endpoint de acesso é protegido por autenticação baseada em JWT, garantindo acesso seguro na API.

## Swagger UI

O Swagger UI é integrado para exploração e documentação interativa de API. Acesse-o navegando para:

```bash
    http://localhost:8000/swagger-ui
```

Aqui, você pode visualizar todos os endpoints disponíveis, juntamente com informações detalhadas sobre formatos de solicitação e resposta.

## Middleware

### Authentication Middleware Guard

O middleware de autenticação personalizado guarda rotas verificando a presença e validade de tokens JWT. Ele garante que somente usuários autenticados possam acessar determinados endpoints.

### Role-Based Access Control

Além da autenticação, algumas rotas impõem controle de acesso baseado em função (RBAC) usando o middleware `RequireAuth`, que verifica funções de usuário como `Admin`, `Moderator` ou `User`.

## Documentação OpenAPI

O projeto oferece suporte ao OpenAPI 3.0, com geração de esquema e documentação de endpoint fornecida por meio do pacote `utoipa`.

### Adicionando e personalizando a documentação OpenAPI

-   **Registre o Esquema OpenAPI:** O objeto `openapi` é configurado em `main.rs`.
-   **Registre o Manipulador de API como Caminho OpenAPI:** Cada manipulador é registrado como um caminho OpenAPI com descrições detalhadas.
-   **Servindo a IU do Swagger:** O objeto OpenAPI é servido por meio de um servidor web, acessível por meio da IU do Swagger.


## Licença 📝

Este projeto é licenciado sob [CC0 1.0 Universal]. Consulte o arquivo [LICENSE](https://github.com/SilvaneiMartins/validacao-email-rust/blob/master/LICENSE) para obter detalhes.

## Doações

Se você achar este projeto útil e quiser apoiar seu desenvolvimento contínuo, você pode fazer uma doação via `PIX` para e-mail `silvaneimartins@hotmail.com`.

Muito ❤️ pelo apoio!

## Contato 📩

<a href="https://github.com/SilvaneiMartins">
    <img
        style="border-radius:50%"
        src="https://github.com/SilvaneiMartins.png"
        width="100px;"
        alt="Silvanei Martins"
    />
    <br />
    <sub>
        <b>Silvanei de Almeida Martins</b>
    </sub>
</a>
     <a href="https://github.com/SilvaneiMartins" title="Silvanei martins" >
 </a>
<br />
🚀 Feito com ❤️ por Silvanei Martins
