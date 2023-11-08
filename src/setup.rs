use run_script::ScriptOptions;

pub fn setup() {
    let options = ScriptOptions::new();

    let _ = run_script::run_script!(
        r#"
        sh ./setup.sh
        "#,
        &options
    );
}