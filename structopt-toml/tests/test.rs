use serde_derive::Deserialize;
use structopt::StructOpt;
use structopt_toml::StructOptToml;

#[derive(Debug, Deserialize, StructOpt, StructOptToml)]
#[serde(default)]
struct Test {
    #[structopt(default_value = "0", long = "a0")]
    a0: i32,
    #[structopt(default_value = "1", long = "a1")]
    a1: i32,
    #[structopt(default_value = "2", long = "a2")]
    a2: i32,
    #[structopt(default_value = "3", long = "a3")]
    a3: i32,

    #[structopt(name = "B0", default_value = "10", long = "b0")]
    b0: i32,
    #[structopt(name = "B1", default_value = "11", long = "b1")]
    b1: i32,
    #[structopt(name = "B2", default_value = "12", long = "b2")]
    b2: i32,
    #[structopt(name = "B3", default_value = "13", long = "b3")]
    b3: i32,

    #[structopt(long = "c0")]
    c0: Option<i32>,
    #[structopt(long = "c1")]
    c1: Option<i32>,
    #[structopt(long = "c2")]
    c2: Option<i32>,
    #[structopt(long = "c3")]
    c3: Option<i32>,

    #[structopt(long = "d0")]
    d0: Vec<i32>,
    #[structopt(long = "d1")]
    d1: Vec<i32>,
    #[structopt(long = "d2")]
    d2: Vec<i32>,
    #[structopt(long = "d3")]
    d3: Vec<i32>,

    #[structopt(long = "quiet")]
    quiet: bool,
}

#[test]
fn test() {
    let toml_str = r#"
        a2 = 102
        a3 = 103
        b2 = 112
        b3 = 113
        c2 = 122
        c3 = 123
        d2 = [132]
        d3 = [133]
    "#;
    let args = vec![
        "test", "--a1", "201", "--a3", "203", "--b1", "211", "--b3", "213", "--c1", "221", "--c3",
        "223", "--d1", "231", "--d3", "233",
    ];
    let test = Test::from_iter_with_toml(toml_str, args.iter()).unwrap();
    assert_eq!(test.a0, 0);
    assert_eq!(test.a1, 201);
    assert_eq!(test.a2, 102);
    assert_eq!(test.a3, 203);
    assert_eq!(test.b0, 10);
    assert_eq!(test.b1, 211);
    assert_eq!(test.b2, 112);
    assert_eq!(test.b3, 213);
    assert_eq!(test.c0, None);
    assert_eq!(test.c1, Some(221));
    assert_eq!(test.c2, Some(122));
    assert_eq!(test.c3, Some(223));
    assert_eq!(test.d0, vec![]);
    assert_eq!(test.d1, vec![231]);
    assert_eq!(test.d2, vec![132]);
    assert_eq!(test.d3, vec![233]);
}

static POSSIBLE_VALUES: &[&str] = &["one", "two"];

#[derive(Debug, Deserialize, StructOpt, StructOptToml)]
#[serde(default)]
struct Bar {
    #[structopt(possible_values = POSSIBLE_VALUES, name = "bar")]
    val: Option<String>,
}

#[test]
fn test_args_with_other_attributes() {
    let toml_str = r#"
    bar = "one"
    "#;
    let test = Bar::from_args_with_toml(toml_str);
    match dbg!(test) {
        Err(_) => assert!(false),
        _ => assert!(true),
    }
}

#[derive(Debug, Deserialize, StructOpt, StructOptToml)]
#[serde(default)]
struct Outer {
    #[structopt(long = "one", default_value = "1")]
    one: u32,
    #[structopt(flatten)]
    two: Inner,
}

#[derive(Debug, Deserialize, StructOpt, StructOptToml)]
struct Inner {
    #[structopt(long = "three", default_value = "1")]
    three: u32,
    #[structopt(long = "four", default_value = "1")]
    four: u32,
}

#[test]
fn test_flatten_args() {
    let toml_str = r#"
        one = 2
        two.three = 2
        two.four = 2
    "#;
    let args = vec!["test", "--four", "3"];
    let test = Outer::from_iter_with_toml(toml_str, args.iter()).unwrap();
    assert_eq!(test.one, 2);
    assert_eq!(test.two.three, 2);
    assert_eq!(test.two.four, 3);
}

#[test]
fn test_toml_failed() {
    let toml_str = r#"
        a2 = "aaa"
        a3 = [102]
        c3 = 123
        d2 = 132
    "#;
    let args = vec!["test"];
    let test = Test::from_iter_with_toml(toml_str, args.iter());
    match test {
        Err(_) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_toml_args() {
    let toml_str = r#"
        a2 = 102
        a3 = 103
        b2 = 112
        b3 = 113
        c2 = 122
        c3 = 123
        d2 = [132]
        d3 = [133]
    "#;
    let test = Test::from_args_with_toml(toml_str).unwrap();
    assert_eq!(test.a0, 0);
    assert_eq!(test.a1, 1);
    assert_eq!(test.a2, 102);
    assert_eq!(test.a3, 103);
    assert_eq!(test.b0, 10);
    assert_eq!(test.b1, 11);
    assert_eq!(test.b2, 112);
    assert_eq!(test.b3, 113);
    assert_eq!(test.c0, None);
    assert_eq!(test.c1, None);
    assert_eq!(test.c2, Some(122));
    assert_eq!(test.c3, Some(123));
    assert_eq!(test.d0, vec![]);
    assert_eq!(test.d1, vec![]);
    assert_eq!(test.d2, vec![132]);
    assert_eq!(test.d3, vec![133]);
}
