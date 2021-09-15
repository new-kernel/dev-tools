# Novusk Development Tools

This is used for compiling specific crates in Novusk, so they can be linked to other files

To install Novusk run:
```commandline
make install
```

To compile a crate:
```commandline
make build CRATE=crate-name TARGET=arch-novusk
```

If you're using arm32 or any RISCV architectures, use the proper target from the ``rustc`` target list.

Example:
To compile unistd:
```commandline
make build CRATE=unistd TARGET=x86_64-novusk
```

You should now see a file called ``libunistd.rlib`` in this directory
