let () =
  Callback.register "internal_dedup" Util.remove_duplicates;
  Callback.register "internal_load_files" Process_file.load_files
