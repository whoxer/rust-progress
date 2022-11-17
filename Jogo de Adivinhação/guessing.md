# Jogo de Adivinhação

Eis um mini joguinho explicado linha por linha de como funciona que dá prosseguimento nos ensinamentos dentro do [livro](https://rust-br.github.io/rust-book-pt-br/ch02-00-guessing-game-tutorial.html).

## Iniciando um novo projeto

Abra o seu terminal favorito e digite:

```
	$~ cargo new guessing_game --bin
	$~ cd jogo_de_adivinhacao
```

Aqui fizemos muita coisa. Criamos um projeto Rust utilizando o Cargo, passando o argumento do nome do projeto pelo comando ```cargo new```. A flag ```---bin``` diz para o Cargo que faça um projeto binário, já o segundo comando só nos leva até a pasta do projeto.

Com o cargo ja inserimos muita coisa no nosso projeto, inclusive um dos arquivos mais importantes que é o arquivo *Cargo.toml*:

```toml

[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Seu Nome <voce@exemplo.com>"]

[dependencies]

```

Mais pra frente ele será muito necessário para a inserção de novas dependências dentro do seus programas.


## Receber strings

