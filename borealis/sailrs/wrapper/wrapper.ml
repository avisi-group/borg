open Libsail
open Sail_plugin_isla
open Ast_util

type error = Err_exception of string * string | Err_sail of Reporting.error

let exception_to_result f =
  Printexc.record_backtrace true;

  try Ok (f ()) with
  | Reporting.Fatal_error inner ->
      let _ = print_endline "fatal sail error" in
      Error (Err_sail inner)
  | e ->
      let _ = print_endline "exception" in
      Error (Err_exception (Printexc.to_string e, Printexc.get_backtrace ()))

let bindings_to_list map = map |> Ast_util.Bindings.to_seq |> List.of_seq
let list_to_bindings list = list |> List.to_seq |> Ast_util.Bindings.of_seq

let get_plugin_dir () =
  match Sys.getenv_opt "SAIL_PLUGIN_DIR" with
  | Some path -> path :: Libsail_sites.Sites.plugins
  | None -> Libsail_sites.Sites.plugins

let load_plugin opts plugin =
  try Dynlink.loadfile_private plugin
  with Dynlink.Error msg ->
    prerr_endline
      ("Failed to load plugin " ^ plugin ^ ": " ^ Dynlink.error_message msg)

let sail_dir =
  match Manifest.dir with
  | Some opam_dir -> opam_dir
  | None -> "/root/.opam/4.14.1+options/share/sail"

let file_to_string filename =
  let chan = open_in filename in
  let buf = Buffer.create 4096 in
  try
    let rec loop () =
      let line = input_line chan in
      Buffer.add_string buf line;
      Buffer.add_char buf '\n';
      loop ()
    in
    loop ()
  with End_of_file ->
    close_in chan;
    Buffer.contents buf

let run_sail filepaths =
  (* register isla target *)
  let tgt =
    Target.register ~name:"isla" ~options:isla_options
      ~pre_parse_hook:isla_initialize ~rewrites:isla_rewrites isla_target
  in

  let options = ref [] in
  let opt_file_out = ref None in
  let opt_free_arguments = ref [] in
  let opt_project_files = ref [] in
  let opt_variable_assignments = ref [] in
  let opt_all_modules = ref true in
  let opt_just_parse_project = ref false in
  let opt_splice = ref [] in
  let config = None in

  Rewrites.opt_mono_rewrites := true;
  Constant_fold.optimize_constant_fold := true;
  Util.opt_verbosity := 2;
  Profile.opt_profile := true;
  Jib_compile.opt_memo_cache := true;

  (* plugins broken with linker errors, removing lem and coq lines from sail stdlib as patch fix *)
  (* (match Sys.getenv_opt "SAIL_NO_PLUGINS" with
     | Some _ -> ()
     | None -> (
         match get_plugin_dir () with
         | dir :: _ ->
             List.iter
               (fun plugin ->
                 let path = Filename.concat dir plugin in
                 if Filename.extension plugin = ".cmxs" then
                   load_plugin options path)
               (Array.to_list (Sys.readdir dir))
         | [] -> ())); *)
  Constraint.load_digests ();

  (* rest is copied from sail.ml:run_sail *)
  Target.run_pre_parse_hook tgt ();

  let project_files, frees =
    List.partition
      (fun free -> Filename.check_suffix free ".sail_project")
      !opt_free_arguments
  in

  let ctx, ast, env, effect_info =
    match (project_files, !opt_project_files) with
    | [], [] ->
        (* If there are no provided project files, we concatenate all
           the free file arguments into one big blob like before *)
        Frontend.load_files ~target:tgt sail_dir !options Type_check.initial_env
          filepaths
    (* Allows project files from either free arguments via suffix, or
       from -project, but not both as the ordering between them would
       be unclear. *)
    | project_files, [] | [], project_files ->
        let t = Profile.start () in
        let defs =
          List.map
            (fun project_file ->
              let root_directory = Filename.dirname project_file in
              let contents = file_to_string project_file in
              Project.mk_root root_directory
              :: Initial_check.parse_project ~filename:project_file ~contents ())
            project_files
          |> List.concat
        in
        let variables = ref Util.StringMap.empty in
        List.iter
          (fun assignment ->
            if not (Project.parse_assignment ~variables assignment) then
              raise
                (Reporting.err_general Parse_ast.Unknown
                   ("Could not parse assignment " ^ assignment)))
          !opt_variable_assignments;
        let proj = Project.initialize_project_structure ~variables defs in
        let mod_ids =
          if !opt_all_modules then Project.all_modules proj
          else
            List.map
              (fun mod_name ->
                match Project.get_module_id proj mod_name with
                | Some id -> id
                | None ->
                    raise
                      (Reporting.err_general Parse_ast.Unknown
                         ("Unknown module " ^ mod_name)))
              frees
        in
        Profile.finish "parsing project" t;
        if !opt_just_parse_project then exit 0;
        let env = Type_check.initial_env_with_modules proj in
        Frontend.load_modules ~target:tgt sail_dir !options env proj mod_ids
    | _, _ ->
        raise
          (Reporting.err_general Parse_ast.Unknown
             "Module files (.sail_project) should either be specified with the \
              appropriate option, or as free arguments with the appropriate \
              extension, but not both!")
  in
  let ast, env = Frontend.initial_rewrite effect_info env ast in
  let ast, env =
    match !opt_splice with
    | [] -> (ast, env)
    | files -> Splice.splice_files ctx ast (List.rev files)
  in
  let effect_info =
    Effects.infer_side_effects (Target.asserts_termination tgt) ast
  in

  (* Don't show warnings during re-writing for now *)
  Reporting.suppressed_warning_info ();
  Reporting.opt_warnings := false;

  Target.run_pre_rewrites_hook tgt ast effect_info env;
  let ctx, ast, effect_info, env =
    Rewrites.rewrite ctx effect_info env (Target.rewrites tgt) ast
  in

  Target.action tgt !opt_file_out
    { ctx; ast; effect_info; env; default_sail_dir = sail_dir; config };

  Constraint.save_digests ();

  let props = Property.find_properties ast in
  Bindings.bindings props |> List.map fst |> IdSet.of_list
  |> Specialize.add_initial_calls;

  (* let ast, env = Specialize.(specialize typ_ord_specialization env ast) in *)
  let cdefs, ctx = jib_of_ast env ast effect_info in
  let cdefs, _ = Jib_optimize.remove_tuples cdefs ctx in
  let cdefs = remove_casts cdefs |> remove_extern_impls |> fix_cons in
  cdefs

let () =
  (* Primary functions *)
  Callback.register "run_sail" (fun filepaths ->
      exception_to_result (fun () -> run_sail filepaths));

  (* Utility *)
  Callback.register "util_dedup" (fun a ->
      exception_to_result (fun () -> Util.remove_duplicates a));

  Callback.register "bindings_to_list" (fun a ->
      exception_to_result (fun () -> bindings_to_list a));
  Callback.register "list_to_bindings" (fun a ->
      exception_to_result (fun () -> list_to_bindings a));

  Callback.register "effectset_elements" (fun set ->
      exception_to_result (fun () -> Effects.EffectSet.elements set));
  Callback.register "effectset_of_list" (fun list ->
      exception_to_result (fun () -> Effects.EffectSet.of_list list));

  Callback.register "bigint_to_string" (fun i ->
      exception_to_result (fun () -> Nat_big_num.to_string i));
  Callback.register "string_to_bigint" (fun i ->
      exception_to_result (fun () -> Nat_big_num.of_string i));

  Callback.register "add_num" (fun a b ->
      exception_to_result (fun () -> Nat_big_num.add a b))
