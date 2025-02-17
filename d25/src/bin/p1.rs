fn main() {
    let (row, column) = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .next()
        .map(|line| {
            let splits: Vec<&str> = line.split_whitespace().rev().collect();
            let column = splits.first().expect("Invalid pattern");
            let column = &column[0..column.len() - 1];
            let column = column.parse::<u64>().expect("Invalid pattern");
            let row = splits.get(2).expect("Invalid pattern");
            let row = &row[0..row.len() - 1];
            let row = row.parse::<u64>().expect("Invalid pattern");
            (row, column)
        })
        .expect("Empty input");
    let diagonal = row + column - 1 - 1;
    let final_index = column + (diagonal * (diagonal + 1)) / 2;
    let output: u64 = (2..=final_index).fold(20151125, |n, _| (n * 252533) % 33554393);
    println!("{}", output);
}
