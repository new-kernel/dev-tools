VERSION =?
CRATE =?
TARGET =?

install:
	@ git clone https://github.com/new-kernel/novusk/
	@ cd novusk/ && git checkout $(VERSION)

build:
	@ cd novusk/ && cargo build -p $(CRATE) --target targets/$(TARGET).json
	@ mv novusk/target/$(TARGET)/debug/lib$(CRATE).rlib $(CRATE).rlib

clean:
	@ rm -rf novusk/
	@ rm -rf *.rlib
