extern crate gcc;

fn main() {
    gcc::Config::new()
                .file("src/foo.c")
                .define("FOO", None)
                .define("BAR", Some("1"))
                .compile("libfoo.a");

    gcc::Config::new()
                .file("src/bar1.c")
                .file("src/bar2.c")
                .include("src/include")
                .compile("libbar.a");

    gcc::Config::new()
                .file("src/bar1.c")
                .file("src/bar2.c")
                .compile("libbar.a");

    let target = std::env::var("TARGET").unwrap();
    let file = target.split("-").next().unwrap();
    let file = format!("src/{}.{}", file,
                       if target.contains("msvc") {"asm"} else {"S"});
    gcc::Config::new()
                .file(file)
                .compile("libasm.a");
}