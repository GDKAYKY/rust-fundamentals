use std::env;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

enum FileSize {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

impl FileSize {
    fn from_string(s: &str) -> Result<FileSize, String> {
        match s.to_lowercase().as_str() {
            "b" | "bytes" => Ok(FileSize::Bytes),
            "kb" | "kilobytes" => Ok(FileSize::Kilobytes),
            "mb" | "megabytes" => Ok(FileSize::Megabytes),
            "gb" | "gigabytes" => Ok(FileSize::Gigabytes),
            _ => Err(format!("Unrecognized size unit: {}", s)),
        }
    }
}

fn parse_input(input: &str) -> Result<(f64, FileSize), String> {
    // Find where the number ends and the unit begins
    let trimmed = input.trim();
    
    // Split the string into number and unit parts
    let mut num_part = "";
    let mut unit_part = "";
    
    // Find the boundary between number and unit
    let chars: Vec<char> = trimmed.chars().collect();
    let mut num_end_idx = 0;
    
    // Look for the end of the number part (first non-numeric/space/non-dot character)
    for i in 0..chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            continue;
        } else if c.is_ascii_digit() || c == '.' {
            num_end_idx = i + 1;
        } else {
            break;
        }
    }
    
    // Extract number part
    if num_end_idx > 0 {
        num_part = trimmed[..num_end_idx].trim();
    } else {
        return Err("No number found in input".to_string());
    }
    
    // Extract unit part
    if num_end_idx < chars.len() {
        unit_part = trimmed[num_end_idx..].trim();
    } else {
        return Err("No unit found in input".to_string());
    }
    
    // Parse the number
    let number = num_part.parse::<f64>()
        .map_err(|_| format!("Could not parse number: {}", num_part))?;
        
    // Parse the unit
    let unit = FileSize::from_string(unit_part)?;
    
    Ok((number, unit))
}

fn calculate_sizes(value: f64, unit: FileSize) -> Sizes {
    // Convert everything to bytes first
    let bytes_value = match unit {
        FileSize::Bytes => value,
        FileSize::Kilobytes => value * 1000.0,
        FileSize::Megabytes => value * 1000.0 * 1000.0,
        FileSize::Gigabytes => value * 1000.0 * 1000.0 * 1000.0,
    };
    
    // Calculate all other units based on bytes
    let kilobytes_value = bytes_value / 1000.0;
    let megabytes_value = bytes_value / (1000.0 * 1000.0);
    let gigabytes_value = bytes_value / (1000.0 * 1000.0 * 1000.0);
    
    Sizes {
        bytes: format!("{} bytes", bytes_value as u64),
        kilobytes: format!("{} kilobytes", kilobytes_value as u64),
        megabytes: format!("{} megabytes", megabytes_value as u64),
        gigabytes: format!("{} gigabytes", gigabytes_value as u64),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: cargo run --bin file_size_formatter \"<size> <unit>\"");
        println!("Example: cargo run --bin file_size_formatter \"24 mb\"");
        return;
    }
    
    // Join all arguments after the first one to handle cases where input has spaces
    let input = args[1..].join(" ");
    
    match parse_input(&input) {
        Ok((value, unit)) => {
            let sizes = calculate_sizes(value, unit);
            println!("{:?}", sizes);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_mb() {
        let result = parse_input("24 mb").unwrap();
        assert_eq!(result.0, 24.0);
        match result.1 {
            FileSize::Megabytes => assert!(true),
            _ => assert!(false, "Expected Megabytes"),
        }
    }

    #[test]
    fn test_parse_input_kb() {
        let result = parse_input("300 kb").unwrap();
        assert_eq!(result.0, 300.0);
        match result.1 {
            FileSize::Kilobytes => assert!(true),
            _ => assert!(false, "Expected Kilobytes"),
        }
    }

    #[test]
    fn test_calculate_sizes_mb() {
        let sizes = calculate_sizes(24.0, FileSize::Megabytes);
        assert_eq!(sizes.bytes, "24000000 bytes");
        assert_eq!(sizes.kilobytes, "24000 kilobytes");
        assert_eq!(sizes.megabytes, "24 megabytes");
        assert_eq!(sizes.gigabytes, "0 gigabytes");
    }

    #[test]
    fn test_calculate_sizes_bytes() {
        let sizes = calculate_sizes(1000.0, FileSize::Bytes);
        assert_eq!(sizes.bytes, "1000 bytes");
        assert_eq!(sizes.kilobytes, "1 kilobytes");
        assert_eq!(sizes.megabytes, "0 megabytes");
        assert_eq!(sizes.gigabytes, "0 gigabytes");
    }
}