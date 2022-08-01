fn main() {
    shadow_rs::shadow!(build);

    println!("{}", build::BUILD_TIME);
    println!("{}", build::BUILD_TIME_2822);
    println!("{}", build::BUILD_TIME_3339);
}
