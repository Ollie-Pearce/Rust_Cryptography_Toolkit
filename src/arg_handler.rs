pub struct ConfigStruct {
    pub caesar_shift: Option<u8>,
    pub vignere_key: Option<String>,
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

    let mut i = 1;
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

            _ => {
                {
                }
            }
        }
    }

    Ok(ConfigStruct {
        caesar_shift,
        vignere_key,
        file_path,
    })
}
