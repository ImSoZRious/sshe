use std::{collections::HashMap, io::{BufRead, BufWriter, Write}};

use crate::sshconfig::{Config, Key};

/// For each parameter, the first obtained value will be used. The
/// configuration files contain sections separated by ''Host'' specifications,
/// and that section is only applied for hosts that match one of the patterns
/// given in the specification. The matched host name is the one given on the
/// command line.
///
/// Since the first obtained value for each parameter is used, more
/// host-specific declarations should be given near the beginning of the file,
/// and general defaults at the end. The configuration file has the following
/// format:
///
/// Empty lines and lines starting with '#' are comments. Otherwise a line is
/// of the format ''keyword arguments''. Configuration options may be
/// separated by whitespace or optional whitespace and exactly one '='; the
/// latter format is useful to avoid the need to quote whitespace when
/// specifying configuration options using the ssh, scp, and sftp -o option.
/// Arguments may optionally be enclosed in double quotes (") in order to
/// represent arguments containing spaces.
pub fn parse<R: BufRead>(reader: R) -> Result<Vec<Config>, Box<dyn std::error::Error>> {
    let mut context: Option<(String, HashMap<Key, String>)> = None;
    let lines = reader.lines();

    let mut result = vec![];

    for line in lines {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line
            .split_once(" =")
            .or_else(|| line.split_once('='))
            .or_else(|| line.split_once(' '))
            .ok_or("invalid key value syntax")?;

        match key {
            "Host" => {
                if let Some((host, map)) = context {
                    result.push(Config { host, columns: map });
                };
                context = Some((value.to_owned(), HashMap::new()));
            }
            other => {
                let key = match other.try_into() {
                    Ok(x) => x,
                    Err(_) => return Err(format!("Unknown key: `{}`", other).into()),
                };

                if let Some(ctx) = context.as_mut() {
                    ctx.1.insert(key, value.to_owned());
                }
            },
        }
    }

    if let Some((host, map)) = context {
        result.push(Config { host, columns: map });
    };

    Ok(result)
}

pub fn save_config<T: Write>(writer: &mut BufWriter<T>, cfg: &[Config]) -> Result<(), Box<dyn std::error::Error>> {
    for cfg in cfg {
        writeln!(writer, "Host {}", cfg.host)?;

        for (k, v) in &cfg.columns {
            writeln!(writer, "  {} {}", k.str(), v)?;
        }

        writeln!(writer)?;
    }

    Ok(())
}