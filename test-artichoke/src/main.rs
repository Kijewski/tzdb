use spinoso_time::tzrs::Time;

fn main() -> anyhow::Result<()> {
    let now = Time::now()?;
    println!("now = {}", &now);
    println!("now? = {:#?}", &now);
    Ok(())
}
