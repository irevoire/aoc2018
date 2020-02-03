let rec apply (base) =
        try
                apply (base + (read_int ()))
        with
        End_of_file -> base
;;

let main () = 
        let base = 0 in
        Printf.printf "res: %d\n" (apply base)
;;

main ()

