# EDR_Endpoint
USAGE:
    edr_endpoint [FLAGS] [OPTIONS]

FLAGS:
    -C               Create File or Reg Key/value
    -D               Delete File or Reg Key/value
    -h, --help       Prints help information
    -M               Modify File or Reg Key/value with random data If no file modify will create one
    -V, --version    Prints version information

OPTIONS:
    -F, --FILE <FILE_PATH>                      Depending on flags will create,delete or modify.
    -N, --NETSEND <IP_ADDRESS> <PORT> <FILE>    Sends a file to server over TCP.
    -P, --PROCESS <PROCESS_PATH>                Starts an executable.





file:
 create - will make a file at a given path and do nothing if already exists
   example  edr_endpoint -F foo.txt -C
 modify - will append hello world to the end of a given file if no file will create one at given path
   example  edr_endpoint -F foo.txt -M
 delete - will delete a given file
   example  edr_endpoint -F foo.txt -d

logging: every time the program successfully runs will append a new event
to edr_endpoint_log.tsv in the directory that the program is run from.

every log event begins with
[time stamp] [username] [pid] [full program path] [[cli arguments]] [requested cmd]
every log event ends with EOE - End Of Event


file arguments will add
[file status] == Exists Create and No_File
[file info]
[cmd activity]
