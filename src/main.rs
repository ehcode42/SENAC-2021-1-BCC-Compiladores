use std::env;
use std::iter::Peekable;

fn main() {
    let codigo = env::args().nth(1).expect("Expressão não fornecida");
    if let Ok(resultado) = avaliar(&codigo) {
        println!("Expressão {} = {}", codigo, resultado);
    } else {
        eprintln!("Não foi possível avaliar a expressão");
    }
}

#[derive(Debug, Clone)]
enum ItemLexico {
    Operador(char),
    Inteiro(i64),
}

fn lexer(input: &str) -> Result<Vec<ItemLexico>, String> {
    let mut tokens = Vec::new();
    let mut iterador = input.chars().peekable();
    while let Some(&c) = iterador.peek() {
        match c {
            '0'..='9' => {
                iterador.next();
                let n = obter_numero(c, &mut iterador);
                tokens.push(ItemLexico::Inteiro(n));
            }
            ' ' | '\t' | '\n' => {
                iterador.next();
            }
            '+' | '-' => {
                tokens.push(ItemLexico::Operador(c));
                iterador.next();
            }
            _ => {
                return Err(format!("Caracter inesperado: {}", c));
            }
        }
    }
    Ok(tokens)
}

fn obter_numero<T: Iterator<Item = char>>(c: char, iterador: &mut Peekable<T>) -> i64 {
    let mut numero = c
        .to_string()
        .parse::<i64>()
        .expect("Função chamada com argumentos errados");

    while let Some(Ok(digito)) = iterador.peek().map(|c| c.to_string().parse::<i64>()) {
        numero = numero * 10 + digito;
        iterador.next();
    }
    numero
}

fn avaliar(codigo: &str) -> Result<i64, String> {
    let mut resultado = 0;
    let tokens = lexer(&codigo).expect("Esperada uma expressão");
    if !tokens.is_empty() {
        resultado = expressao(&tokens, 0).expect("Erro no parsing");
    }
    Ok(resultado)
}

fn expressao(tokens: &Vec<ItemLexico>, mut delta: usize) -> Result<i64, String> {
    let resultado;

    let n1 = match termo(tokens, delta) {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    delta += 1;

    if delta == tokens.len() {
        return Ok(n1);
    }

    let &operador = match tokens.get(delta) {
        Some(ItemLexico::Operador(op)) => op,
        token @ Some(_) => {
            return Err(format!("Token não esperado: {:?}", token));
        }
        None => {
            return Err(format!("Expressão incompleta!"));
        }
    };

    delta += 1;

    let n2 = match expressao(tokens, delta) {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    match operador {
        '+' => resultado = n1 + n2,
        _ => {
            return Err(format!(
                "Símbolo não é um operador reconhecido: {:?}",
                operador
            ));
        }
    }

    Ok(resultado)
}

fn termo(tokens: &Vec<ItemLexico>, delta: usize) -> Result<i64, String> {
    let &n = match tokens.get(delta + 0) {
        Some(ItemLexico::Inteiro(n)) => n,
        token @ Some(_) => {
            return Err(format!("Token não esperado: {:?}", token));
        }
        None => {
            return Err(format!("Expressão incompleta!"));
        }
    };
    Ok(n)
}
