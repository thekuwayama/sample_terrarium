# sample_terrarium

[![MIT licensed](https://img.shields.io/badge/license-MIT-brightgreen.svg)](https://raw.githubusercontent.com/thekuwayama/sample_terrarium/master/LICENSE.txt)

sample_terrarium is sample using `http_guest::kvstore::KVStore`.

- https://wasm.fastlylabs.com/


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
