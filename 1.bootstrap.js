(window.webpackJsonp=window.webpackJsonp||[]).push([[1],[,function(e,t,n){"use strict";n.r(t);var o=n(2);let r,a=null,i=null;function c(){return null!==a&&a.buffer===r.memory.buffer||(a=new Uint8Array(r.memory.buffer)),a}function l(){return null!==i&&i.buffer===r.memory.buffer||(i=new Uint32Array(r.memory.buffer)),i}fetch("./ncmdump_rs_bg.wasm").then(e=>e.arrayBuffer()).then(e=>WebAssembly.instantiate(e,{env:{emscripten_memcpy_big:(e,t,n)=>{c().copyWithin(e,t,t+n)},emscripten_resize_heap:e=>{let t=c().length;return e>t&&r.memory.grow(1+(2*e-t>>>16)),!0},__cxa_atexit:()=>{console.log("__cxa_atexit")},abort:()=>{console.log("abort")},__cxa_allocate_exception:()=>{console.log("__cxa_allocate_exception")},__cxa_throw:()=>{console.log("__cxa_throw")}},"./ncmdump_rs_bg.js":{__wbindgen_throw:()=>{console.log("__wbindgen_throw")}},wasi_snapshot_preview1:{environ_sizes_get:()=>{console.log("environ_sizes_get")},environ_get:()=>{console.log("environ_get")}}})).then(e=>{r=e.instance.exports,r.__wasm_call_ctors();let t=["UnexpectedEof","BadMagic","BadAes","BadBase64","BadLength","BadMetadata"],n=document.getElementById("result_list");function a(e){let t=e.length,n=r.__wbindgen_malloc(t);try{c().set(e,n),r.work(n,t);let o=r.RET.value/4,a=l()[o+2],i=l()[o+4];return console.log(l().subarray(o,o+10)),[l()[o],l()[o+1],c().subarray(a,a+i)]}finally{r.__wbindgen_free(n,t)}}window.handle_files=e=>{n.innerHTML="";for(let r of e){let e=new FileReader;e.onload=e=>{let i,c=new Uint8Array(e.target.result),l=Date.now(),[s,u,f]=a(c),d=Date.now()-l;if(0===s){let e=r.name.replace(".ncm",u?".flac":".mp3");i=`${e}: ${d}ms`,Object(o.saveAs)(new Blob([f]),e)}else i=`${r.name}: ${t[s-1]}`;let p=document.createElement("li");p.appendChild(document.createTextNode(i)),n.appendChild(p)},e.readAsArrayBuffer(r)}}})},function(e,t,n){(function(n){var o,r,a;r=[],void 0===(a="function"==typeof(o=function(){"use strict";function t(e,t,n){var o=new XMLHttpRequest;o.open("GET",e),o.responseType="blob",o.onload=function(){i(o.response,t,n)},o.onerror=function(){console.error("could not download file")},o.send()}function o(e){var t=new XMLHttpRequest;t.open("HEAD",e,!1);try{t.send()}catch(e){}return 200<=t.status&&299>=t.status}function r(e){try{e.dispatchEvent(new MouseEvent("click"))}catch(n){var t=document.createEvent("MouseEvents");t.initMouseEvent("click",!0,!0,window,0,0,0,80,20,!1,!1,!1,!1,0,null),e.dispatchEvent(t)}}var a="object"==typeof window&&window.window===window?window:"object"==typeof self&&self.self===self?self:"object"==typeof n&&n.global===n?n:void 0,i=a.saveAs||("object"!=typeof window||window!==a?function(){}:"download"in HTMLAnchorElement.prototype?function(e,n,i){var c=a.URL||a.webkitURL,l=document.createElement("a");n=n||e.name||"download",l.download=n,l.rel="noopener","string"==typeof e?(l.href=e,l.origin===location.origin?r(l):o(l.href)?t(e,n,i):r(l,l.target="_blank")):(l.href=c.createObjectURL(e),setTimeout((function(){c.revokeObjectURL(l.href)}),4e4),setTimeout((function(){r(l)}),0))}:"msSaveOrOpenBlob"in navigator?function(e,n,a){if(n=n||e.name||"download","string"!=typeof e)navigator.msSaveOrOpenBlob(function(e,t){return void 0===t?t={autoBom:!1}:"object"!=typeof t&&(console.warn("Deprecated: Expected third argument to be a object"),t={autoBom:!t}),t.autoBom&&/^\s*(?:text\/\S*|application\/xml|\S*\/\S*\+xml)\s*;.*charset\s*=\s*utf-8/i.test(e.type)?new Blob(["\ufeff",e],{type:e.type}):e}(e,a),n);else if(o(e))t(e,n,a);else{var i=document.createElement("a");i.href=e,i.target="_blank",setTimeout((function(){r(i)}))}}:function(e,n,o,r){if((r=r||open("","_blank"))&&(r.document.title=r.document.body.innerText="downloading..."),"string"==typeof e)return t(e,n,o);var i="application/octet-stream"===e.type,c=/constructor/i.test(a.HTMLElement)||a.safari,l=/CriOS\/[\d]+/.test(navigator.userAgent);if((l||i&&c)&&"object"==typeof FileReader){var s=new FileReader;s.onloadend=function(){var e=s.result;e=l?e:e.replace(/^data:[^;]*;/,"data:attachment/file;"),r?r.location.href=e:location=e,r=null},s.readAsDataURL(e)}else{var u=a.URL||a.webkitURL,f=u.createObjectURL(e);r?r.location=f:location.href=f,r=null,setTimeout((function(){u.revokeObjectURL(f)}),4e4)}});a.saveAs=i.saveAs=i,e.exports=i})?o.apply(t,r):o)||(e.exports=a)}).call(this,n(3))},function(e,t){var n;n=function(){return this}();try{n=n||new Function("return this")()}catch(e){"object"==typeof window&&(n=window)}e.exports=n}]]);