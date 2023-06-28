# esp32s2-nvs-linker-failure

Generated from [esp-idf-template](https://github.com/esp-rs/esp-idf-template/tree/master) with option `std`.

Fails to link for target `xtensa-esp32s2-espidf` with the following error:

```text
  = note: [ldproxy] Running ldproxy
          Error: Linker /home/x/.rustup/toolchains/esp/xtensa-esp32s2-elf/esp-12.2.0_20230208/xtensa-esp32s2-elf/bin/xtensa-esp32s2-elf-gcc failed: exit status: 1
          STDERR OUTPUT:
          /home/x/.rustup/toolchains/esp/xtensa-esp32s2-elf/esp-12.2.0_20230208/xtensa-esp32s2-elf/bin/../lib/gcc/xtensa-esp32s2-elf/12.2.0/../../../../xtensa-esp32s2-elf/bin/ld: /home/x/.rustup/toolchains/esp/xtensa-esp32s2-elf/esp-12.2.0_20230208/xtensa-esp32s2-elf/bin/../lib/gcc/xtensa-esp32s2-elf/12.2.0/../../../../xtensa-esp32s2-elf/lib/no-rtti/libstdc++.a(atomicity.o):(.literal._ZN9__gnu_cxx18__exchange_and_addEPVii+0x8): undefined reference to `atexit'
          /home/x/.rustup/toolchains/esp/xtensa-esp32s2-elf/esp-12.2.0_20230208/xtensa-esp32s2-elf/bin/../lib/gcc/xtensa-esp32s2-elf/12.2.0/../../../../xtensa-esp32s2-elf/bin/ld: /home/x/.rustup/toolchains/esp/xtensa-esp32s2-elf/esp-12.2.0_20230208/xtensa-esp32s2-elf/bin/../lib/gcc/xtensa-esp32s2-elf/12.2.0/../../../../xtensa-esp32s2-elf/lib/no-rtti/libstdc++.a(atomicity.o): in function `_ZN9__gnu_cxx18__exchange_and_addEPVii':
          /builds/idf/crosstool-NG/.build/xtensa-esp32s2-elf/build/build-cc-gcc-final/xtensa-esp32s2-elf/no-rtti/libstdc++-v3/libsupc++/atomicity.cc:33: undefined reference to `atexit'
          collect2: error: ld returned 1 exit status


          Stack backtrace:
             0: <unknown>
             1: <unknown>
             2: <unknown>
             3: <unknown>
             4: <unknown>
             5: <unknown>
             6: __libc_start_main
             7: <unknown>

  = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)

error: could not compile `aaaa` (bin "aaaa") due to previous error
```
