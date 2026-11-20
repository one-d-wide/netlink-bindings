fn main() {
    let res = reverse_lookup::run(
        &mut argh::from_env(),
        &mut reverse_lookup::ApiArgs {
            redirect_signals: true,
            ..Default::default()
        },
    );
    if !res {
        std::process::exit(1);
    }
}
