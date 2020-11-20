# quick-alias
  
  quick-alias will be usefull for creating aliases for part of the command or full command.<br>
  
  All of the alias mapping will be store in toml file on user machine. <br>
  
  User can add/remove these aliasing via command.<br>
  
  Example:<br>
  ===== qa.toml ==== <br>
 `bb: bazel build` <br>
 `db_path: my/db/path/here/` <br>
 `merge_path: merge/path/here/` <br>
  ===== /qa.toml ===
  
  Now you can use it as `qa {bb} {db_path}/new/{merge_path}`<br>
  
 `qa {bb} {db_path}/new/{merge_path}  => bazel build my/db/path/here/new/my/merge/path/here/` <br>
 
