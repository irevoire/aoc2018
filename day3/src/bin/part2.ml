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

let map_inc_entry map entry id: (coord, int list) Hashtbl.t =
        match Hashtbl.mem map entry with
        | false -> Hashtbl.add map entry [id]; map
        | true -> Hashtbl.replace map entry (id :: Hashtbl.find map entry); map

let rec _insert_claim (claim: claim) (current: coord) (map): (coord, int list) Hashtbl.t =
        match current.x, current.y with
        | x, y when claim.c.x + claim.d.w - 1 = x && claim.c.y + claim.d.h - 1 = y -> map_inc_entry map current claim.id
        | x, _ when claim.c.x + claim.d.w = x -> _insert_claim claim {x = claim.c.x; y = current.y + 1 } map
        | _ -> _insert_claim claim { x = current.x + 1; y = current.y } (map_inc_entry map current claim.id)
        
let insert_claim (claim: claim) (map): (coord, int list) Hashtbl.t = _insert_claim claim {x = claim.c.x; y = claim.c.y } map

let rec _init_hashtbl map =
        try
                _init_hashtbl (insert_claim (parse_claim ()) map)
        with
                End_of_file -> map

let init_hashtbl () = _init_hashtbl (Hashtbl.create 10000)

let map = init_hashtbl ()
let id_map: (int, int) Hashtbl.t = Hashtbl.create 1000


let map_id_inc_entry (map: (int, int) Hashtbl.t) (l: int list) =
        match List.length l = 1 with
        | true when Hashtbl.mem map (List.hd l) -> ()
        | true -> Hashtbl.add map (List.hd l) 1
        | false -> List.iter (fun v -> if Hashtbl.mem map v then Hashtbl.replace map v 2 else Hashtbl.add map v 2) l

let count_id map id_map =
  Hashtbl.iter (fun _ v -> map_id_inc_entry id_map v) map
;;
count_id map id_map


let res = Hashtbl.fold (fun id v acc -> if v = 1 then (Printf.printf "%d\n" id; id) else acc) id_map 0
;;

Printf.printf "%d\n" res
