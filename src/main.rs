
fn main() {
    // Withdout recursion
    let numero_de_sequencias: i128 = 10000;
    let mut actual:i128= 1;
    let mut primeiro_anterior: i128 = 1;
    let mut segundo_anterior: i128 = 1;

    let mut count = 0;
    while count < numero_de_sequencias {
        actual = primeiro_anterior + segundo_anterior;
        primeiro_anterior = segundo_anterior;
        segundo_anterior = actual;
        count = count + 1;
    }


    // With recutsion
    fn fibonachi(value: i64) -> i64 {
        if value < 1 {
            return 1
        }
    
        fibonachi(value - 1) + fibonachi(value - 2)
    }

    let _valor = fibonachi(20);
    println!("O nono valor da sequencia de fibonachi e {}", actual)
}


