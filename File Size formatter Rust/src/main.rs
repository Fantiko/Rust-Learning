use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_size(size_string: String) -> String {
    let partes:Vec<&str> = size_string.split(' ').collect();

    if partes.len() < 2 {
        return String::from("Erro: Formato inválido. Use '<tamanho> <unidade>' (ex: '1024 KB').");
    }

    
    let size_raw: u64 = match partes[0].parse::<u64>() {
        Ok(numero) => numero,
        Err(e) => {
            println!("Erro ao converter: {}", e);
            0 // Valor padrão em caso de erro
        },
    };

    let size: u64 = match partes[1] {
        "bytes" => size_raw * 1,
        "KB" => size_raw * 1000,
        "MB" => size_raw * 1_000_000,
        "GB" => size_raw * 1_000_000_000,
        _ => {
            println!("Erro: Unidade desconhecida '{}'", partes[1]);
            return String::from("Erro: Unidade desconhecida. Use 'bytes', 'KB', 'MB', ou 'GB'.");
        }
    };

    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();
    
        if args.len() < 2 {
            println!("Erro: Por favor, forneça o tamanho e a unidade. Exemplo: 'cargo run -- 1024 KB'");
            return;
        }
        
    let resposta = format_size(args[1].clone());
    // The first argument is the size that was used to call the program. Must use quotes to
    // read this as a single argument
    println!("Size is: {}.", resposta);
}