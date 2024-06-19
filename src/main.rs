mod rope;

fn main() {
    let new_rope_1 = rope::Rope::new("hello_world");
    let new_rope_2 = rope::Rope::new("new_value");

    let new_rope_3 = rope::Rope::concat(new_rope_1, new_rope_2).unwrap();

    let _ = rope::Rope::char_at_index(&new_rope_3, 2).unwrap();

    let _ = new_rope_3.clone().split_at_index(2);

    let _ = new_rope_3.clone().delete_between_index(3, 6).unwrap();

    let add_at_index = new_rope_3
        .clone()
        .insert_at_index(4, "janijanijani")
        .unwrap();

    println!("{add_at_index:?}");
}
