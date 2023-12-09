open Core

(*let rec first_char (line : char list) : char =*)
(*  match line with*)
(*  | [] -> '0'*)
(*  | h :: t -> ( match h with '0' .. '9' -> h | _ -> first_char t)*)

(*let first_digit (line : string) : char =*)
(*  let char_list = line |> String.to_list in*)
(*  first_char char_list*)

(*let last_digit (line : string) : char =*)
(*  let char_list = line |> String.to_list |> List.rev in*)
(*  first_char char_list*)

let cases =
  [
    ("one", "1");
    ("two", "2");
    ("three", "3");
    ("four", "4");
    ("five", "5");
    ("six", "6");
    ("seven", "7");
    ("eight", "8");
    ("nine", "9");
    ("1", "1");
    ("2", "2");
    ("3", "3");
    ("4", "4");
    ("5", "5");
    ("6", "6");
    ("7", "7");
    ("8", "8");
    ("9", "9");
  ]

let map_to_digit str pos =
  List.find_map cases ~f:(fun (substr, value) ->
      match String.substr_index ~pos str ~pattern:substr with
      | Some matched when matched = pos -> Some value
      | _ -> None)

let str_to_numbers str =
  let map_to_number = map_to_digit str in
  List.range 0 (String.length str) |> List.filter_map ~f:map_to_number

let parse_calibration (line : string) : int =
  let numbers = str_to_numbers line in
  let number = Fmt.str "%s%s" (List.hd_exn numbers) (List.last_exn numbers) in
  let number = int_of_string_opt number in
  match number with Some v -> v | None -> 0

let rec parse_doc (input : In_channel.t) : int list =
  match In_channel.input_line input with
  | Some l -> parse_calibration l :: parse_doc input
  | None -> []

let sum values = List.fold ~init:0 ~f:(fun acc x -> acc + x) values

let () =
  let in_file = In_channel.create "./inputs/day01.txt" in
  let values = parse_doc in_file in
  let result = sum values in
  Printf.printf "Total sum: %d\n" result
