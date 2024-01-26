use tokenizers::tokenizer::{Result, Tokenizer};

fn main() -> Result<()> {
    // #[cfg(feature = "http")]
    {
        let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)?;
        let encoding = tokenizer.encode("Hey there!", false)?;
        println!("{:?}", encoding);
        println!("{:?}", encoding.get_tokens());
    }
    Ok(())
}

