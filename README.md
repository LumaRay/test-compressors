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
|  rust  | zip-store  | mondial-3.0.xml | 100% | 354 / 659 |
|  rust  | zstd lvl1  | mondial-3.0.xml | 10% | 3236 / 1227 |
|  rust  | zip-zstd lvl+1  | mondial-3.0.xml | 10% | 3466 / 1959 |
|  rust  | zip-zstd lvl-6  | mondial-3.0.xml | 16% | 2722 / 1765 |
|  rust  | lz4 lvl2  | mondial-3.0.xml | 16% | 2238 /  871 |
|  rust  | zip>lz4 lvl2  | mondial-3.0.xml | 16% | 2592 /  1530 |
|:-------:|:---------:|:---------:|:---------:|:---------:|
|  rust  | zip-store  | mondial-3.0-mini.xml | 100% | 208 / 435 |
|  rust  | zstd lvl1  | mondial-3.0-mini.xml | 14% | 2718 /  994 |
|  rust  | zip-zstd lvl+1  | mondial-3.0-mini.xml | 14% | 2758 / 1590 |
|  rust  | zip-zstd lvl-6  | mondial-3.0-mini.xml | 21% | 2167 / 1313 |
|  rust  | lz4 lvl2  | mondial-3.0-mini.xml | 21% | 1875 /  605 |
|  rust  | zip>lz4 lvl2  | mondial-3.0-mini.xml | 21% | 2083 /  1040 |
|:-------:|:---------:|:---------:|:---------:|:---------:|
|  rust  | zip-store  | mondial-3.0.json | 100% | 1237 / 1899 |
|  rust  | zstd lvl1  | mondial-3.0.json | 6% | 4597 / 1777 |
|  rust  | zip-zstd lvl+1  | mondial-3.0.json | 6% | 4651 / 3561 |
|  rust  | zip-zstd lvl-6  | mondial-3.0.json | 10% | 3971 / 3574 |
|  rust  | lz4 lvl2  | mondial-3.0.json | 10% | 3715 / 1734 |
|  rust  | zip>lz4 lvl2  | mondial-3.0.json | 10% | 4952 / 3633 |
|:-------:|:---------:|:---------:|:---------:|:---------:|
|  rust  | zip-store  | mondial-3.0-mini.json | 100% | 269 / 420 |
|  rust  | zstd lvl1  | mondial-3.0-mini.json | 14% | 2725 / 1022 |
|  rust  | zip-zstd lvl+1  | mondial-3.0-mini.json | 14% | 2824 / 1546 |
|  rust  | zip-zstd lvl-6  | mondial-3.0-mini.json | 21% | 2188 / 1294 |
|  rust  | lz4 lvl2  | mondial-3.0-mini.json | 22% | 1962 /  639 |
|  rust  | zip>lz4 lvl2  | mondial-3.0-mini.json | 22% | 2231 /  1059 |

## Conclusions

* zstd has ~x1.5 slower compression than lz4 
* zstd has ~x1.5 better compression rate than lz4 
* When combined with zip-store, lz4 has less speed ratio, so zip>zstd or zip-zstd seem to be the best choices as theese are better than zip>lz4.
