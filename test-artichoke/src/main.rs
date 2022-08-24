use spinoso_time::tzrs::Time;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Time::now()?;
    println!("now = {}", &now);
    println!("now? = {:#?}", &now);
    Ok(())
}
