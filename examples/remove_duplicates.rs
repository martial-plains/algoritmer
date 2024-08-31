use algoritmer::strings::RemoveDuplicates;

fn main() {
    let value = "Peter Piper picked a peck of pickled peppers
            A peck of pickled peppers Peter Piper picked
            If Peter Piper picked a peck of pickled peppers
            Where’s the peck of pickled peppers Peter Piper picked?"
        .remove_duplicates(' ');

    println!("{}", value);
}
