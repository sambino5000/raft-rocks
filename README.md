# raft-rocks
 kv store on raft
 
NOT yet implemented-> raft

change [PATH](https://github.com/sambino5000/raft-rocks/blob/main/src/main.rs#L11) to your where
ever you want to store database

write key-values to rocksdb with enpoints: 

 ```0.0.0.0:3000/<key>/<value>```
 
 get value with key:
 
  ```0.0.0.0:3000/get/<key>```
