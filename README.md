# quick-alias(qa)
  
  quick-alias(qa) is a usefull tool for creating aliases for long and repeated command text(s).<br> 
  We can use these aliasing dynamically to form many other commands.<br>
  
  All of the alias mapping will be store in toml file on user machine. <br>
  
  User can add/remove these aliasing via command.<br>
  
  Example:<br>
  ===== q_alias.ini ==== <br>
 `bb=bazel build` <br>
 `db_path=my/db/path/here/` <br>
 `merge_path=merge/path/here/` <br>
  ===== /q_alias.toml ===
  
  Now you can use it as `qa {bb} {db_path}/new/{merge_path}`<br>
  
 `qa {bb} {db_path}/new/{merge_path}  => bazel build my/db/path/here/new/my/merge/path/here/` <br>
 
# How to install ?

Download the binary from https://github.com/grv07/quick-alias/blob/master/example/qa <br>

You can make it executable with `sudo chmod +x qa`

# How to use ?
'./qa -a "sort_path" "some/full/long/path"' to add quick aliasing.
'./qa --help' for quick aliasing help.

'./qa bazel build {sort_path}' to run os command with aliases.

**NOTE: Aliases set in your bashrc are not gonna work. May be correct in future :) 
