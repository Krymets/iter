
// fn print_elements(elemets: &[String]) {};
// print_elements(&colors[1..3]);


fn print_elements(elements: &Vec<String>) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements
        .iter()
        .map(|el|format!("{} {}", el, el))
        .for_each(|element| println!("{}", element));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: & [String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // let mut colors_iter = colors.iter();
    //
    // println!("{:?}", colors_iter.next());
    // println!("{:?}", colors_iter.next());
    // println!("{:?}", colors_iter.next());
    // println!("{:?}", colors_iter.next());

    // print_elements(&colors);

    shorten_strings(&mut colors[1..3]);
    println!("{:#?}", colors);

    let uppercased = to_uppercase(&colors);
    println!("{:#?}", uppercased);
}