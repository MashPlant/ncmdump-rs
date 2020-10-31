.PHONY: all

all: taglib
	 EMSDK=$$EMSDK CC=emcc CXX=em++ AR=emar wasm-pack build
	 cp pkg/ncmdump_rs_bg.wasm www

taglib:
	git clone https://github.com/taglib/taglib
	cd taglib && \
	sed -i '2,$$ d' ConfigureChecks.cmake && \
	mkdir build && \
	cd build && \
	CC=emcc CXX=em++ AR=emar cmake .. -DCMAKE_BUILD_TYPE=Release && \
	make && \
	emranlib taglib/libtag.a