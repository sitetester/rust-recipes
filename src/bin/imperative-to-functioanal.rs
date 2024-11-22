fn bar(value: &str) -> Option<&str> {
    if value == "Paris" || value == "Rome" {
        return Some(value);
    }
    None
}

fn foo<'a>(input: &Vec<&'a str>) -> Vec<(usize, &'a str)> {
    let mut output = Vec::new();
    let mut input_index = 0;
    let mut output_index = 0;
    while input_index < input.len() {
        let value = bar(input[input_index]);
        if value.is_some() {
            output.push((output_index, value.unwrap()));
            output_index += 1;
        }
        input_index += 1;
    }
    output
}


fn foo_using_iter<'a>(input: &Vec<&'a str>) -> Vec<(usize, &'a str)> {
    let mut output = Vec::new();
    let mut output_index = 0;
    for value in input.into_iter() {
        let value = bar(value);
        if value.is_some() {
            output.push((output_index, value.unwrap()));
            output_index += 1;
        }
    }
    output
}

fn foo_using_filter_and_map_with_explicit_match<'a>(input: &Vec<&'a str>) -> Vec<(usize, &'a str)> {
    let mut output_index = 0;
    let vec: Vec<(usize, &str)> = input.into_iter()
        .filter(|v| { // filter boolean values
            match bar(v) {
                Some(_v) => true,
                _ => false
            }
        })
        .map(|v| {
            let tuple = (output_index, *v); // closure can access `output_index` from parent scope
            output_index += 1;
            tuple // return this tuple
        }).collect(); // eventually convert it to the desired format ```Vec<(usize, &str)>```
    vec
}

// Same as above, but explicit `match` part is removed,
// since it's auto handled by `filter`, as `is_some()` returns bool
fn foo_using_filter_and_map<'a>(input: &Vec<&'a str>) -> Vec<(usize, &'a str)> {
    let mut output_index = 0;
    let vec: Vec<(usize, &str)> = input.into_iter()
        .filter(|v| bar(v).is_some())
        .map(|v| {
            let tuple = (output_index, *v);
            output_index += 1;
            tuple
        })
        .collect();
    vec
}

// We can also combine separate `filter` & `map` operations with a single `filter_map`
// doesn't look so elegant though in this particular case
fn foo_using_filter_map<'a>(input: &Vec<&'a str>) -> Vec<(usize, &'a str)> {
    let mut output_index = 0;
    let vec: Vec<(usize, &str)> = input.into_iter()
        .filter_map(|v| bar(v)
            // repeated map :(
            // probably this part could be improved
            .map(|v| {
                let tuple = (output_index, v);
                output_index += 1;
                tuple
            })
        )
        .collect();
    vec // return this `vec`
}

// here we are calling multiple functions & they all give same output
// each next function is an improved version of previous
// NOTE: Original `input: Vec<&'a str>` has changed to `input: &Vec<&'a str>` to avoid passing a clone each time  (i.e, vec.clone())
fn main() {
    let vec = vec!["London", "Paris", "Berlin", "Rome"];
    let expected_vec = vec![(0, "Paris"), (1, "Rome")];

    assert_eq!(foo(&vec), expected_vec);
    assert_eq!(foo_using_iter(&vec), expected_vec);

    assert_eq!(foo_using_filter_and_map_with_explicit_match(&vec), expected_vec);
    assert_eq!(foo_using_filter_and_map(&vec), expected_vec);

    assert_eq!(foo_using_filter_map(&vec), expected_vec);

}

