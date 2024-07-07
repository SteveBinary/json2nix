use json2nix;

fn main() {
    let input = r#"
        {
            "hello": "world",
            "null_value": null,
            "number": 123,
            "empty_list": [],
            "list": [
                true,
                false,
                123.456,
                {
                    "a": "b"
                }
            ],
            "empty_object": {},
            "object": {
                "key": "value",
                "abc": null,
                "null": null,
                "123": {
                    "a-b": 4
                }
            }
        }
    "#;

    println!("{}", json2nix::json2nix(input, 0, 2).unwrap());

    let input = r#"
        [
            true,
            false,
            123,
            123.456,
            null,
            "hello",
            [ ],
            [ true, false, 123 ],
            { },
            {
                "abc": "def",
                "sub-object": {
                    "a": "a",
                    "b": "b"
                }
            }
        ]
    "#;

    println!("{}", json2nix::json2nix(input, 0, 2).unwrap());
}
