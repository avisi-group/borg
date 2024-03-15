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

let run_sail filepaths =
  (* register isla target *)
  let tgt =
    Target.register ~name:"isla" ~options:isla_options
      ~pre_parse_hook:isla_initialize ~rewrites:isla_rewrites isla_target
  in

  let options = [] in
  let opt_splice = [] in
  let opt_file_out = None in
  let config = None in

  Util.opt_verbosity := 2;

  (match Sys.getenv_opt "SAIL_NO_PLUGINS" with
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
      | [] -> ()));

  Constraint.load_digests ();

  (* rest is copied from sail.ml:run_sail *)
  Target.run_pre_parse_hook tgt ();
  let ast, env, effect_info =
    Frontend.load_files ~target:tgt Manifest.dir options Type_check.initial_env
      filepaths
  in
  let ast, env = Frontend.initial_rewrite effect_info env ast in
  let ast, env =
    List.fold_right
      (fun file (ast, _) -> Splice.splice ast file)
      opt_splice (ast, env)
  in
  let effect_info =
    Effects.infer_side_effects (Target.asserts_termination tgt) ast
  in
  Reporting.opt_warnings := false;

  (* Don't show warnings during re-writing for now *)
  Target.run_pre_rewrites_hook tgt ast effect_info env;
  let ast, effect_info, env =
    Rewrites.rewrite effect_info env (Target.rewrites tgt) ast
  in

  Target.action tgt config Manifest.dir opt_file_out ast effect_info env;

  Constraint.save_digests ();

  (ast, env, effect_info)

let generate_jib ast env effect_info =
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

  Callback.register "generate_jib" (fun ast env effect_info ->
      exception_to_result (fun () -> generate_jib ast env effect_info));

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
