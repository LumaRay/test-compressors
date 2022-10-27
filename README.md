# Benchmarking different compression algorithms

The testing has been performed on Windows 7x64 host with Ubuntu 18.04 x64 virtual machine with 8Gb RAM and 4 cores, Intel-VT enabled.

Host:
- CPU: Intel Core i7-4790 3.6 GHz 4 cores / 8 threads
- RAM: 32 Gb

AVX2 is used whenever possible.

Full release optimizations.

Compress/Decompress is used over several files:

```
mondial-3.0.xml (1'784'825 bytes)
mondial-3.0-mini.xml (1'174'507 bytes)
mondial-3.0-mini.xml.lz4 (187'707 bytes)

mondial-3.0.json (3'580'567 bytes)
mondial-3.0-mini.json (1'174'212 bytes)
mondial-3.0-mini.json.lz4 (190'047 bytes)
```

1'000 cycles per test.

All tests use 1 core.

Test results are given in milliseconds of total execution time (compress/decompress), in descending order, worst to best.

You can find links to the algorithms' web pages in the first comments of the corresponding main source files.

## Test Results

| Language | Author  | File | Ratio | Time, ms  |
|:-------:|:---------:|:---------:|:---------:|:---------:|
|  rust  | lz4 lvl2  | mondial-3.0.xml | 16% | 2238 /  871 |
|  rust  | zstd lvl1  | mondial-3.0.xml | 10% | 3236 / 1227 |
|  rust  | lz4 lvl2  | mondial-3.0-mini.xml | 21% | 1875 /  605 |
|  rust  | zstd lvl1  | mondial-3.0-mini.xml | 14% | 2718 /  994 |
|  rust  | lz4 lvl2  | mondial-3.0.json | 10% | 3715 / 1734 |
|  rust  | zstd lvl1  | mondial-3.0.json | 6% | 4597 / 1777 |
|  rust  | lz4 lvl2  | mondial-3.0-mini.json | 22% | 1962 /  639 |
|  rust  | zstd lvl1  | mondial-3.0-mini.json | 14% | 2725 / 1022 |

## Conslusions:

* zstd has ~x1.5 slower compression than lz4 
* zstd has ~x1.5 better compression rate than lz4 
