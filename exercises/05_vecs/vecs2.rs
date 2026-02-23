fn vec_loop(input: &[i32]) -> Vec<i32> {
    // 'input' é recebido como uma referência (ponteiro) para um slice de inteiros.
    let mut output: Vec<i32> = Vec::new();

    for element in input {
        // 'element' é uma referência (&i32) para cada item do slice.
        // O operador '*' desreferencia o ponteiro para obter o valor original.
        // O resultado da multiplicação é um valor (i32) que é movido para o push.
        output.push(*element * 2);
    }
    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // .iter() cria um iterador que produz referências (&i32).
    // O Rust realiza "deref coercion" automaticamente no operador '+':
    // 'element + 1' funciona mesmo que 'element' seja uma referência.
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // .map() recebe o ponteiro 'element'.
    // A multiplicação aqui também utiliza coerção automática ou
    // pode ser escrita como (*element * 2) para ser explícita.
    input.iter().map(|element| element * 2).collect()
}

fn main() {
    let input = [1, 2, 3, 4, 5];

    // Passamos '&input' (um ponteiro/referência) para não transferir a posse dos dados.
    let result = vec_loop(&input);

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        // Passagem de ponteiro (&input).
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
