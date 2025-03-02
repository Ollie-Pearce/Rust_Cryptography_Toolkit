pub struct ConfigStruct {
    pub caesar_shift: Option<u8>,
    pub vignere_key: Option<String>,
    pub rsa_key: Option<(u64, u64)>,
    pub md5: bool, // Add this field
    pub file_path: String,
}

pub fn parse_args(mut args: Vec<String>) -> Result<ConfigStruct, String> {
    args.remove(0);

    if args.len() < 2 {
        return Err("Not enough arguments. Usage: program [options] <file_path>".to_string());
    }

    let file_path = args[0].clone();

    let mut caesar_shift: Option<u8> = None;
    let mut vignere_key: Option<String> = None;
    let mut rsa_key = None;
    let mut md5 = false; // Initialize md5 flag as false

    let mut iter = args[1..].iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-caesar" => {
                let shift_str = iter.next().ok_or_else(|| "Missing shift value after caesar")?;
                let shift = shift_str.parse().map_err(|_| "Invalid shift value for caesar")?;
                caesar_shift = Some(shift);
            }
            "-vignere" => {
                let key = iter.next().ok_or_else(|| "Missing key after vignere")?;
                vignere_key = Some(key.to_string());
            }
            "-rsa" => {
                let key_1_str = iter.next().ok_or_else(|| "Missing RSA encryption key")?;
                let key_2_str = iter.next().ok_or_else(|| "Missing second RSA key")?;
                let key_1 = key_1_str.parse::<u64>().map_err(|_| "Error parsing RSA key 1 into integer")?;
                let key_2 = key_2_str.parse::<u64>().map_err(|_| "Error parsing RSA key 2 into integer")?;
                rsa_key = Some((key_1, key_2));
            }
            "-md5" => {
                md5 = true; // Set md5 flag to true if the argument is present
            }
            _ => {
                return Err(format!("Unknown argument: {}", arg));
            }
        }
    }

    Ok(ConfigStruct {
        caesar_shift,
        vignere_key,
        rsa_key,
        md5,
        file_path,
    })
}