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

let get_lexbuf_string s filename =
  let lexbuf = Lexing.from_string s in
  lexbuf.Lexing.lex_curr_p <-
    {
      Lexing.pos_fname = filename;
      Lexing.pos_lnum = 1;
      Lexing.pos_bol = 0;
      Lexing.pos_cnum = 0;
    };
  lexbuf

let parse_file ?loc:(l = Parse_ast.Unknown) (source : string) (filename : string)
    : Lexer.comment list * Parse_ast.def list =
  try
    let lexbuf = get_lexbuf_string source filename in
    try
      Lexer.comments := [];
      let defs = Parser.file Lexer.token lexbuf in

      (!Lexer.comments, defs)
    with Parser.Error ->
      let pos = Lexing.lexeme_start_p lexbuf in
      let tok = Lexing.lexeme lexbuf in
      raise (Reporting.err_syntax pos ("current token: " ^ tok))
  with Sys_error err -> raise (Reporting.err_general l err)

let load_files ?target defs comments type_envs =
  let t = Profile.start () in

  let ast = Initial_check.process_ast ~generate:true defs in
  let ast = { ast with comments } in

  (* The separate loop measures declarations would be awkward to type check, so
     move them into the definitions beforehand. *)
  let ast = Rewrites.move_loop_measures ast in
  Profile.finish "parsing" t;

  let t = Profile.start () in
  let asserts_termination =
    Option.fold ~none:false ~some:Target.asserts_termination target
  in
  let ast, type_envs, side_effects =
    Frontend.check_ast asserts_termination type_envs ast
  in
  Profile.finish "type checking" t;

  (ast, type_envs, side_effects)

let run_sail definitions comments =
  let tgt =
    Target.register ~name:"isla" ~options:isla_options
      ~pre_parse_hook:isla_initialize ~rewrites:isla_rewrites isla_target
  in
  let () = Target.run_pre_parse_hook tgt () in

  let env = Type_check.initial_env in
  let manifest_dir = "" in
  let opt_splice = [] in
  let opt_file_out = None in


  let ast, env, effect_info = load_files ~target:tgt definitions comments env in

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

  let () =
    Target.action tgt None manifest_dir opt_file_out ast effect_info env
  in

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
  Callback.register "parse_file" (fun contents filename ->
      exception_to_result (fun () -> parse_file contents filename));

  Callback.register "preprocess" (fun sail_dir target options file_ast ->
      exception_to_result (fun () ->
          Preprocess.preprocess sail_dir target options file_ast));

  Callback.register "run_sail" (fun definitions comments ->
      exception_to_result (fun () -> run_sail (Parse_ast.Defs definitions) comments));

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
      exception_to_result (fun () -> Nat_big_num.add a b));
