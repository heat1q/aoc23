open Core

let read_lines file =
  Stdio.In_channel.with_file file ~f:(fun channel ->
      let x = In_channel.input_all channel in
      String.split_lines x)

let sum values = List.fold ~init:0 ~f:(fun acc x -> acc + x) values
