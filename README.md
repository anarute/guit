# Guit

## Introdução

Bem-vindo ao Guit, um pacote que permite usar um subset de comandos do git no bom e velho português.

Baseado nessa [bobeira aqui](https://twitter.com/ngtv_bg/status/1763541475323695502).

## Instalação

Para instalar o Guit, é necessário ter Rust instalado. 

Instale o pacote usando `cargo install guit`.

Você pode usar direto `guit <comando>`, exemplo: `guit puxa` para rodar `git pull`.

Ou se quiser pode até criar um ~alias~ apelido para usar git direto passando os comandos em português, como `git puxar`.

Para isso, adicione `alias git='guit'` na configuração do seu shell, se for BASH, em `~/.bashrc`.

## Contribuindo

Para contribuir, clone o projeto localmente e use cargo para rodar: `cargo run <comando git>`. As traduções estão no arquivo `/src/list_comandos.rs`.

Aceito contribuições de quem quiser ajudar a traduzir mais comandos e expandir o wrapper para traduzir flags, subcomandos e as mensagens de retorno do git.

## Aviso

Esse pacote é uma brincadeira e foi feito como uma forma de aprender um pouco de Rust. Ele é bem simples e cobre apenas comandos básicos do `git`, portanto não é recomendado usá-lo no dia a dia.

Comandos que exigem interação do usuário como `commit` ou `rebase` interativo não funcionam muito bem porque é necessário tratar melhor o retorno dos comandos no Rust por enquanto é só praticmente um alias do primeiro argumento passado pelo usuário.