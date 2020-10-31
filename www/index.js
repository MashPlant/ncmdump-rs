import {saveAs} from 'file-saver';

let wasm;
let cacheHeap8 = null, cacheHeap32 = null;

function HEAP8() {
  if (cacheHeap8 === null || cacheHeap8.buffer !== wasm.memory.buffer) {
    cacheHeap8 = new Uint8Array(wasm.memory.buffer);
  }
  return cacheHeap8;
}

function HEAP32() {
  if (cacheHeap32 === null || cacheHeap32.buffer !== wasm.memory.buffer) {
    cacheHeap32 = new Uint32Array(wasm.memory.buffer);
  }
  return cacheHeap32;
}

fetch('./ncmdump_rs_bg.wasm').then(x => x.arrayBuffer()).then(x => WebAssembly.instantiate(x, {
  env: {
    emscripten_memcpy_big: (dest, src, num) => { HEAP8().copyWithin(dest, src, src + num); },
    emscripten_resize_heap: (request) => {
      let old = HEAP8().length;
      if (request > old) {
        wasm.memory.grow(((2 * request - old) >>> 16) + 1);
      }
      return true;
    },
    __cxa_atexit: () => { console.log('__cxa_atexit'); },
    abort: () => { console.log('abort'); },
    __cxa_allocate_exception: () => { console.log('__cxa_allocate_exception'); },
    __cxa_throw: () => { console.log('__cxa_throw'); },
  },
  './ncmdump_rs_bg.js': {
    __wbindgen_throw: () => { console.log('__wbindgen_throw'); },
  },
  wasi_snapshot_preview1: {
    environ_sizes_get: () => { console.log('environ_sizes_get'); },
    environ_get: () => { console.log('environ_get'); },
  }
})).then(obj => {
  wasm = obj.instance.exports;
  wasm.__wasm_call_ctors();

  let err_list = ['UnexpectedEof', 'BadMagic', 'BadAes', 'BadBase64', 'BadLength', 'BadMetadata'];
  let result_list = document.getElementById('result_list');

  window.handle_files = (files) => {
    result_list.innerHTML = '';
    for (let f of files) {
      let reader = new FileReader();
      reader.onload = (event) => {
        let ncm = new Uint8Array(event.target.result);
        let beg = Date.now();
        let [err, format, buf] = work(ncm);
        let elapsed = Date.now() - beg;
        let msg;
        if (err === 0) {
          let save = f.name.replace('.ncm', format ? '.flac' : '.mp3');
          msg = `${save}: ${elapsed}ms`;
          saveAs(new Blob([buf]), save);
        } else {
          msg = `${f.name}: ${err_list[err - 1]}`;
        }
        let li = document.createElement('li');
        li.appendChild(document.createTextNode(msg));
        result_list.appendChild(li);
      };
      reader.readAsArrayBuffer(f);
    }
  };

  function work(ncm) {
    let len0 = ncm.length, ptr0 = wasm.__wbindgen_malloc(len0);
    try {
      HEAP8().set(ncm, ptr0);
      wasm.work(ptr0, len0);
      let ret = wasm.RET.value / 4;
      let ptr = HEAP32()[ret + 2], len = HEAP32()[ret + 4];
      console.log(HEAP32().subarray(ret, ret + 10));
      return [HEAP32()[ret], HEAP32()[ret + 1], HEAP8().subarray(ptr, ptr + len)];
    } finally {
      wasm.__wbindgen_free(ptr0, len0);
    }
  }
});
