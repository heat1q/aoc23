let validate_games lines = List.map (fun e -> String.length e) lines

let () =
  let lines = Aoc.read_lines "./inputs/day01.txt" in
  let result = lines |> validate_games |> Aoc.sum
  let () = print_endline lines in
  Printf.printf "Total sum: %d\n" 1
