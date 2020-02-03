module SI = Set.Make(struct type t = int let compare = compare end)

let rec _parse (): 'int list =
        try
                (* this is shit but if we put read_int in the head we have a stack overflow *)
                _parse () @ [read_int ()]
        with
        End_of_file -> []


let rec rev (l: 'a list) : 'a list =
        match l with
        | [] -> []
        | head::tail -> (rev tail) @ [head]

let rec parse (): 'int list =
        rev (_parse ())

let rec _apply (base: int) (l: 'int list) (set ): int =
        match l with
        | [] -> base
        | head::tail ->
                        let freq = base + head in
                        if SI.mem freq set then freq
                        else _apply freq (tail @ [head]) (SI.add freq set)

let apply (l: 'int list): int =
        _apply 0 l (SI.singleton 0)
;;

Printf.printf "res: %d\n" (apply ((parse ())));
