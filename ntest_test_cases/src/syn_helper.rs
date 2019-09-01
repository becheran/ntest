pub fn lit_to_str(lit: &syn::Lit) -> String {
    match lit {
        syn::Lit::Bool(s) => s.value.to_string(),
        syn::Lit::Str(s) => string_to_identifier(&s.value()),
        syn::Lit::Int(s) => number_to_identifier(s.base10_digits()),
        syn::Lit::Float(s) => number_to_identifier(s.base10_digits()),
        _ => unimplemented!("String conversion for literal. Only bool, str, positive int, and float values are supported."),
    }
}

fn number_to_identifier(num: &str) -> String {
    num.chars()
        .map(|x| match x {
            '.' => 'd',
            '0'...'9' => x,
            _ => panic!("This is not a valid number. Contains unknown sign {}", x),
        })
        .collect()
}

fn string_to_identifier(num: &str) -> String {
    num.chars()
        .map(|x| match x {
            '0'...'9' => x,
            'a'...'z' => x,
            'A'...'Z' => x,
            _ => '_',
        })
        .collect()
}
