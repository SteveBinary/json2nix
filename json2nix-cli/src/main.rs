use json2nix;

fn main() {
    let input = r#"
        {
            "hello": "world",
            "null_value": null,
            "number": 123,
            "list": [],
            "empty_object": {},
            "object": {
                "key": "value"
            }
        }
    "#;

    println!("{}", json2nix::json2nix(input).unwrap());
}
