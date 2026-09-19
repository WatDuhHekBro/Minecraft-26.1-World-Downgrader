const ERROR_SLINT_BUILD: &str = "Slint build failed";

fn main() {
    slint_build::compile("ui/main.slint").expect(ERROR_SLINT_BUILD);
}
