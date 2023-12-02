let rec first_char (line : char list) : char =
  match line with
  | [] -> '0'
  | h :: t -> if h >= '0' && h <= '9' then h else first_char t

let first_digit (line : string) : char =
  let char_list = line |> String.to_seq |> List.of_seq in
  first_char char_list

let last_digit (line : string) : char =
  let char_list = line |> String.to_seq |> List.of_seq |> List.rev in
  first_char char_list

let parse_calibration (line : string) : int =
  let first_digit = first_digit line |> String.make 1 in
  let last_digit = last_digit line |> String.make 1 in
  let result = first_digit ^ last_digit |> int_of_string_opt in
  match result with Some v -> v | None -> 0

let rec parse_doc (input : in_channel) : int list =
  match In_channel.input_line input with
  | Some l -> parse_calibration l :: parse_doc input
  | None -> []

let sum values = List.fold_left (fun acc x -> acc + x) 0 values

let () =
  let in_file = open_in "./day1/in.txt" in
  let values = parse_doc in_file in
  let result = sum values in
  Printf.printf "Total sum: %d\n" result

(*let out_file = open_out "./day1/out.txt" in*)
(*let () =*)
(*  List.iter (fun i -> Printf.fprintf out_file "%s\n" i) (parse_doc in_file)*)
(*in*)
(*close_out out_file*)
