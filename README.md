### qa(quick-alias)
  
A CLI tool for creating aliases for long and repeated command(s).<br> 

All of the alias mapping will be store in toml file on user machine.<br>

User can add/remove these aliasing via command.<br>
  
#### Example:<br> 
```
qa -a "bb" "bazel build"
qa -a "db_path" "my/db/path/here/"
qa -a "merge_path" "merge/path/here/"

qa {bb} {db_path}/new/{merge_path}  == Will resolve to=> bazel build my/db/path/here/new/my/merge/path/here/
``` 
#### How to use ?
>
*qa -a "sort_path" "some/full/long/path"* to add quick aliasing.
*qa bazel build {sort_path}* to run command with aliases.
