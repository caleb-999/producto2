use std::env;

// Version 3: Convert number to words in Spanish using native Rust library
// Usage: cargo run --bin version3 -- <number>
// Example: cargo run --bin version3 -- 10

fn number_to_spanish_words(n: i64) -> String {
    if n == 0 {
        return "cero".to_string();
    }
    
    let units = ["", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve"];
    let teens = ["diez", "once", "doce", "trece", "catorce", "quince", "dieciséis", "diecisiete", "dieciocho", "diecinueve"];
    let tens = ["", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa"];
    let hundreds = ["", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos", "seiscientos", "setecientos", "ochocientos", "novecientos"];
    
    if n < 10 {
        return units[n as usize].to_string();
    }
    if n < 20 {
        return teens[(n - 10) as usize].to_string();
    }
    if n < 100 {
        let t = (n / 10) as usize;
        let u = (n % 10) as usize;
        if u == 0 {
            return tens[t].to_string();
        }
        return format!("{} y {}", tens[t], units[u]);
    }
    if n < 1000 {
        let h = (n / 100) as usize;
        let rest = n % 100;
        if h == 1 && rest == 0 {
            return "cien".to_string();
        }
        if rest == 0 {
            return hundreds[h].to_string();
        }
        return format!("{} {}", hundreds[h], number_to_spanish_words(rest));
    }
    if n < 1000000 {
        let thousands = n / 1000;
        let rest = n % 1000;
        if thousands == 1 {
            if rest == 0 {
                return "mil".to_string();
            }
            return format!("mil {}", number_to_spanish_words(rest));
        }
        if rest == 0 {
            return format!("{} mil", number_to_spanish_words(thousands));
        }
        return format!("{} mil {}", number_to_spanish_words(thousands), number_to_spanish_words(rest));
    }
    
    n.to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let number = args.get(1).unwrap_or(&"10".to_string()).clone();
    
    match number.parse::<i64>() {
        Ok(num) => {
            let result = number_to_spanish_words(num);
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
