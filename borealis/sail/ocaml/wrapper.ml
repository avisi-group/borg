let dedup l = Util.remove_duplicates l
let () = Callback.register "dedup" dedup
