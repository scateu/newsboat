use libnewsboat::cliargsparser::CliArgsParser;
use std::{env, path::PathBuf};
use tempfile::TempDir;

#[test]
fn t_cliargsparser_dash_capital_c_resolves_tilde_to_homedir() {
    let tmp = TempDir::new().unwrap();

    env::set_var("HOME", tmp.path());

    let filename = "newsboat-config";
    let arg = format!("~/{}", filename);

    let check = |opts| {
        let args = CliArgsParser::new(opts);
        assert_eq!(
            args.config_file,
            Some(PathBuf::from(tmp.path().join(filename)))
        );
    };

    check(vec![
        "newsboat".to_string(),
        "-C".to_string(),
        arg.to_string(),
    ]);

    check(vec![
        "newsboat".to_string(),
        "--config-file=".to_string() + &arg,
    ]);
}