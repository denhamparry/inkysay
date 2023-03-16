use anyhow::Result;
use std::io::{self, Write};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Message to be displayed
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        let message = "Hello, world!".to_string();
        let opt = Opt { message };
        let expected_output = format!(r#"
    ______________
    < {} >
    --------------
          \
            \
              \
                \   ▄æΦΦΦ¥╗▄
                 ,▀╨ ...... ╨▄
                ▄╙╙▀╙.........╨p
               ╫ ..............╙µ
               ▀...█▓.......▄█▌.╫
              j▒.. ██.......╨██.║
               ▀,...............╜
          ╓e⌐7░ ▓^."w...^..zL."╫ j7Tw▄
         ╙▄,..▄╜ ...... ─ ..... ╨▄..,▄▀
             ▌ .... ╣......▌ .... ▓
             ▀╪▄▄╧╙  ▌....▄ └╙╧▄▄╝▀
                      ▀¥╨▀
        "#, opt.message);
        let actual_output = inksay_output(&opt);
        assert_eq!(expected_output, actual_output);

        Ok(())
    }
}


fn main() -> Result<()> {
    let opt = Opt::from_args();
    let _inky = inksay_output(&opt);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(_inky.as_bytes())?;

    Ok(())
}

fn inksay_output(opt: &Opt) -> String {
    let message = format!(r#"
    ______________
    < {} >
    --------------
          \
            \
              \
                \   ▄æΦΦΦ¥╗▄
                 ,▀╨ ...... ╨▄
                ▄╙╙▀╙.........╨p
               ╫ ..............╙µ
               ▀...█▓.......▄█▌.╫
              j▒.. ██.......╨██.║
               ▀,...............╜
          ╓e⌐7░ ▓^."w...^..zL."╫ j7Tw▄
         ╙▄,..▄╜ ...... ─ ..... ╨▄..,▄▀
             ▌ .... ╣......▌ .... ▓
             ▀╪▄▄╧╙  ▌....▄ └╙╧▄▄╝▀
                      ▀¥╨▀
        "#, opt.message);
    return message;
}