use anyhow::Result;
use std::io::{self, Write};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Message to be displayed
    message: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

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

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(message.as_bytes())?;

    Ok(())
}
