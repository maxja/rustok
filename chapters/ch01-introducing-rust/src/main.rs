fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let china = "你好，世界";
    let india = "नमस्ते दुनिया";
    let ukraine = "Привіт, світ!";
    let finland = "Hei maailma!";

    let regions = [southern_germany, japan, china, india, ukraine, finland];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
