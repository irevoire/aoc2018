type coord = { x: int; y : int }
type dimension = { w: int; h : int }
type claim = { id: int; c : coord; d : dimension }

let claim_print claim = Printf.printf "Claim { id: %d; coord: %d; dimension: %d }\n" claim.id claim.c.x claim.d.w

let parse_claim (): claim =
        let input = Str.split (Str.regexp "[#@,:x]") (read_line ()) in
        let input = List.map (fun el -> int_of_string (String.trim el)) input in
        let id = List.nth input 0 in
        let coord = { x = List.nth input 1; y = List.nth input 2 } in
        let dimension = { w = List.nth input 3; h = List.nth input 4 } in
        { id = id; c = coord; d = dimension }

let map_inc_entry map entry: (coord, int) Hashtbl.t =
        match Hashtbl.mem map entry with
        | false -> Hashtbl.add map entry 1; map
        | true -> Hashtbl.replace map entry (Hashtbl.find map entry + 1); map

let rec _insert_claim (claim: claim) (current: coord) (map): (coord, int) Hashtbl.t =
        match current.x, current.y with
        | x, y when claim.c.x + claim.d.w - 1 = x && claim.c.y + claim.d.h - 1 = y -> map_inc_entry map current
        | x, _ when claim.c.x + claim.d.w = x -> _insert_claim claim {x = claim.c.x; y = current.y + 1 } map
        | _ -> _insert_claim claim { x = current.x + 1; y = current.y } (map_inc_entry map current)
        
let insert_claim (claim: claim) (map): (coord, int) Hashtbl.t = _insert_claim claim {x = claim.c.x; y = claim.c.y } map

let rec _init_hashtbl map =
        try
                _init_hashtbl (insert_claim (parse_claim ()) map)
        with
                End_of_file -> map

let init_hashtbl () = _init_hashtbl (Hashtbl.create 10000)

let map = init_hashtbl ()
let res = Hashtbl.fold (fun _ v acc -> (if v > 1 then 1 else 0) + acc) map 0
;;

Printf.printf "There is %d square inch of overlapping fabric\n" res
