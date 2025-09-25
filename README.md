# `docs-as-code`

[![MIT License](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](./LICENSE.md)
[![Build Status](https://img.shields.io/github/actions/workflow/status/your-username/docs-as-code/rust.yml?branch=main&style=flat-square)](https://github.com/your-username/docs-as-code/actions)

Uma ferramenta de linha de comando (CLI) rápida, escrita em Rust, para listar e visualizar seus documentos Markdown diretamente no terminal.

## Demonstração

`docs-as-code` transforma seu diretório de documentação em uma lista organizada e legível, e permite visualizar qualquer documento instantaneamente.

```bash
$ docs-as-code list ./_docs
+------+--------------------------------+-----------------+
| ID   | Título                         | Nome do Arquivo |
+=========================================================+
| doc1 | Este é o Título do Documento 1 | doc1.md         |
|------+--------------------------------+-----------------|
| doc2 | Este é o Título do Documento 2 | doc2.md         |
|------+--------------------------------+-----------------|

```

## Features

* **Listagem em Tabela:** Apresenta um sumário claro de todos os documentos `.md` encontrados recursivamente.
* **Visualização Rápida:** Renderiza o conteúdo de qualquer documento diretamente no terminal com formatação e cores.
* **Extração Inteligente de Título:** Determina o título de um documento automaticamente, usando a seguinte prioridade:
    1.  Metadados YAML (`title:`).
    2.  Primeiro cabeçalho de Nível 1 (`# Título`).
    3.  Nome do arquivo.
* **Performático e Seguro:** Construído em Rust para garantir velocidade e confiabilidade.
* **CLI Intuitiva:** Comandos e argumentos claros, com ajuda gerada automaticamente.

## Instalação

### A partir do código-fonte (atualmente)

Você precisará do toolchain do Rust instalado.

1.  Clone o repositório:
    ```bash
    git clone [https://github.com/your-username/docs-as-code.git](https://github.com/your-username/docs-as-code.git)
    cd docs-as-code
    ```
2.  Compile em modo de release (otimizado):
    ```bash
    cargo build --release
    ```
3.  O binário estará em `target/release/docs-as-code`. Você pode copiar este arquivo para um diretório no seu `PATH` (ex: `/usr/local/bin`) para usá-lo de qualquer lugar.

### A partir do Crates.io (futuramente)

Quando o projeto for publicado, a instalação será tão simples quanto:

```bash
cargo install docs-as-code
```

## Uso

`docs-as-code` funciona com subcomandos. Os principais são `list` e `view`.

**Listar Documentos**

O comando `list` é o padrão. Ele escaneia um diretório e exibe a tabela de sumário.

```bash
# Roda no diretório atual
docs-as-code list

# Roda em um diretório específico
docs-as-code list /caminho/para/seus/docs

# Como é o padrão, você pode omitir 'list'
docs-as-code /caminho/para/seus/docs
```

**Visualizar um Documento**

O comando `view` procura por um documento com um ID específico (o nome do arquivo sem a extensão) e exibe seu conteúdo.

```bash
# Procura por 'doc1.md' no diretório atual e o exibe
docs-as-code view doc1

# Procura por 'ADR-005.md' em um diretório específico
docs-as-code view ADR-005 --path /caminho/para/seus/docs
```

**Obter Ajuda**

Para ver todos os comandos e opções disponíveis:

```bash
docs-as-code --help
```

## Desenvolvimento

Para contribuir ou rodar o projeto em modo de desenvolvimento:

Build: `cargo build`

Run: `cargo run -- <COMMAND>` (ex: cargo run -- list ./_docs)

## Licença

Este projeto é distribuído sob a licença MIT. Veja o arquivo LICENSE.md para mais detalhes.
