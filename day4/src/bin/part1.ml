type date = int
type id = int
type action =
  | FallsAsleep
  | WakesUp
  | BeginShift of id

type event = { date: date; action: action; line: string }

let parse_date date =
  let date = Str.split (Str.regexp "[][: -]") date in
        let date = List.map (fun el -> int_of_string (String.trim el)) date in
        let (_year, month, day, hour, minute) = (List.nth date 0, List.nth date 1, List.nth date 2, List.nth date 3, List.nth date 4) in
        month * 40 * 60 * 60 + day * 60 * 60 + hour * 60 + minute

let parse_action action =
        let action = Str.split (Str.regexp " #?") action in
        match List.nth action 0 with
        | "falls" -> FallsAsleep
        | "wakes" -> WakesUp
        | "Guard" -> BeginShift (int_of_string (List.nth action 1))
        | _ -> Printf.printf "PROUT\n"; FallsAsleep

let parse_line line =
          let (date, action) = (Str.string_before line 18, Str.string_after line 19) in
          {date = parse_date date; action = parse_action action; line = line}

let rec _parse_input l: event list =       
        try
                _parse_input (parse_line (read_line ()) :: l)
        with
                End_of_file -> l

let parse_input (): event list =       
        _parse_input []

let rec compute l map id =
  match l with
  | [] -> ()
  | { action = BeginShift id; _ } :: tail -> compute tail map id
  | { date = d1; action = FallsAsleep; _ } :: { date = d2; action = WakesUp; _ } :: tail ->
    if Hashtbl.mem map id then
      Hashtbl.replace map id ((Hashtbl.find map id) + d2 - d1 - 1)
    else
      Hashtbl.add map id (d2 - d1 - 1);
    compute tail map id
  | _ -> Printf.printf "caca\n"
;;

let events = List.fast_sort (fun a b -> compare a.date b.date) (parse_input ());;
(* List.iter (fun e -> Printf.printf "%s\n" e.line) events;; *)

let map = Hashtbl.create 100;;
compute events map 0;;
let tmp k v (av, aid): (int * int) = if v > av then (v, k) else (av, aid);;
let res = Hashtbl.fold tmp map (0, 0);;

let minutes = Array.make 60 0;;

let rec inc (arr: int array) index ending =
        match index with
        | _ when index <= ending -> arr.(index) <- (arr.(index) + 1); inc arr (index + 1) ending
        | _ -> ()

let rec compute_minutes l (arr: int array) current_id good_id =
        match l with
        | [] -> ()
        | { action = BeginShift id; _ } :: tail -> compute_minutes tail arr id good_id
        | { date = d1; action = FallsAsleep; _ } :: { date = d2; action = WakesUp; _ } :: tail when current_id = good_id ->
                inc arr (d1 mod 60) (d2 mod 60 - 1);
                compute_minutes tail arr current_id good_id
        | { action = FallsAsleep; _ } :: { action = WakesUp; _ } :: tail -> compute_minutes tail arr current_id good_id
        | _ -> Printf.printf "caca\n"
;;

compute_minutes events minutes 0 (res |> snd);;

let rec index_of_max arr current_idx max max_idx =
        match arr with
        | [] -> max_idx
        | head :: tail when head > max -> index_of_max tail (current_idx + 1) head current_idx
        | _ :: tail -> index_of_max tail (current_idx + 1) max max_idx

let best = index_of_max (Array.to_list minutes) 0 0 0;;

Printf.printf "The guard with the id %d has sleep for %d minutes, the best minute is %d, it generate a score of %d\n" (res |> snd) (res |> fst) best (best * (res |> snd));;
