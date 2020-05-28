module SI = Set.Make(struct type t = char list let compare = compare end)

let rec remove_n (l: 'char list) (index: int) : 'char list =
        match l, index with
        | [], _ -> []
        | _::tail, 0 -> tail
        | head::tail, _ -> head :: remove_n tail (index - 1)

let explode (s: string): 'char list =
        List.init (String.length s) (String.get s)

let rec input (): 'char list list =
        try
                input () @ [explode (read_line ())]
        with
        End_of_file -> []

let rec _doublon (input: 'char list list) (set: SI.t): 'char list option =
        match input with
        | [] -> None
        | head :: _ when SI.mem head set -> Option.some head
        | head :: tail -> _doublon tail (SI.add head set)

let doublon (input: 'char list list): 'char list option =
        _doublon input (SI.empty)

let rec _compute (input: 'char list list) (index: int): 'char list =
        match doublon (List.map (fun el -> remove_n el index) input) with
        | Some res -> res
        | None -> _compute input (index + 1)

let compute (input: 'char list list): 'char list =
        _compute input 0

let print_list l = List.iter (Printf.printf "%c") l; Printf.printf "\n";
;;

Printf.printf "the correct box id is: ";
print_list (compute (input ()));
