use sudo_sol::SudokuGrid;

const GAMES: &str = include_str!("../data/examples.txt");

fn main() {
    for (sudo, sol) in GAMES
        .lines()
        .step_by(2)
        .zip(GAMES.lines().skip(1).step_by(2))
        .take(1000)
    {
        println!("Solving {}", sudo);
        let mut grid = SudokuGrid::from(sudo);
        let res = grid.solve();
        let final_state = grid.serialize();
        println!("Result: {}, Final: {}", res, final_state);
        if res {
            assert_eq!(sol, final_state);
        }
        println!()
    }
}

/*200006009070008500860950037100030090589400371006090425607040010010780964400603002*/
/*245376189973128546861954237124537698589462371736891425657249813312785964498613752*/
