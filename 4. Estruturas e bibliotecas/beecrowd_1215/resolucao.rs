// Questão 1215 - Primeiro Dicionário de Andy - Nível 6

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut linhas: Vec<String> = Vec::new();
    
    for linha in stdin.lock().lines() {
        if let Ok(texto) = linha {
            if texto.is_empty() {
                continue;
            }
            linhas.push(texto.to_lowercase());
        }
    }

    let mut palavra = String::new();
    let mut palavras: Vec<String> = Vec::new();
    for linha in linhas {
        for c in linha.chars() {
            if (c as u32) > 96 && (c as u32) < 123 {
                palavra.push(c);
            } else if palavra.len() > 0 {
                if !palavras.contains(&palavra) {
                    palavras.push(palavra.clone());
                }
                palavra.clear();
            }
        }
        if palavra.len() > 0 {
            if !palavras.contains(&palavra) {
                palavras.push(palavra.clone());
            }
            palavra.clear();
        }
    }

    palavras.sort();
    for i in palavras {
        println!("{}", i);
    }
}
