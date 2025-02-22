use sudoku;

#[test]
fn parse_and_display() {
    let input = "
5  3  .  |  .  7  .  |  .  .  .
6  .  .  |  1  9  5  |  .  .  .
.  9  8  |  .  .  .  |  .  6  .
-------------------------------
8  .  .  |  .  6  .  |  .  .  3
4  .  .  |  8  .  3  |  .  .  1
7  .  .  |  .  2  .  |  .  .  6
-------------------------------
.  6  .  |  .  .  .  |  2  8  .
.  .  .  |  4  1  9  |  .  .  5
.  .  .  |  .  8  .  |  .  7  9
"
    .trim();

    let grid = sudoku::Grid::parse(input);
    assert_eq!(grid.unwrap().to_string(), input);
}
