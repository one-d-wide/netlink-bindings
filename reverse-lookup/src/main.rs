fn main() {
    let res = reverse_lookup::run(
        argh::from_env(),
        reverse_lookup::ApiArgs {
            redirect_signals: true,
            ..Default::default()
        },
    );
    if !res {
        std::process::exit(1);
    }
}
