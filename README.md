# Coding challenge "async api gateway"

## Task

Merge two endpoints into one response:

For a given `id`, call `/users/:id` and `/users/:id/posts` then merge the responses.

## Solution

1. run `docker run -p 3000:3000 svenwal/jsonplaceholder` to start a mock backend for better performance
2. run `cargo run`

## Benchmark

**Tweaks**: Check your `ulimits` to prevent errors from your OS while running the server or the benchmark!

```shell script
$ ulimit -n
# test your ulimit with
256 # this may not be enough
```

Update those limits as described [here](https://unix.stackexchange.com/a/221988/10435). Maybe restart your system. 

```shell script
# set 
ulimit -n 65536

# test your ulimit with
$ ulimit -n
65536 # better!
```


benchmark (100 concurrent users, 30 seconds) with 
```shell script
$ siege -c 100 -t30s  http://127.0.0.1:8080/1

Lifting the server siege...
Transactions:		       16302 hits
Availability:		      100.00 %
Elapsed time:		       29.36 secs
Data transferred:	       37.08 MB
Response time:		        0.10 secs
Transaction rate:	      555.25 trans/sec
Throughput:		        1.26 MB/sec
Concurrency:		       58.06
Successful transactions:       16302
Failed transactions:	           0
Longest transaction:	        0.20
Shortest transaction:	        0.01
```