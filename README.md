# sample_terrarium

sample_terrarium is sample using `http_guest::kvstore::KVStore`.


## Usage

```bash
$ make
```

```bash
$ curl https://xxxx-xxxx-xxxx-xxxx.fastly-terrarium.com

$ curl -X POST -d 'value1' https://xxxx-xxxx-xxxx-xxxx.fastly-terrarium.com/key1

$ curl https://xxxx-xxxx-xxxx-xxxx.fastly-terrarium.com/key1
value1
```
