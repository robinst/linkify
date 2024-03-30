(()=>{var e,t,n,r,o,a,i={},l={};function c(e){var t=l[e];if(void 0!==t)return t.exports;var n=l[e]={id:e,loaded:!1,exports:{}};return i[e](n,n.exports,c),n.loaded=!0,n.exports}let s;function p(e,t){const n=performance.now();f.innerHTML=function(e,t){return void 0!==s?s.linkify_text(e,t):void 0}(e,t);const r=Math.ceil(performance.now()-n);if(0===e.length)g.innerHTML="";else{const t=f.getElementsByTagName("a").length;g.innerHTML="✓ <strong>"+r.toLocaleString("en-US")+" ms</strong> to linkify "+t+" links in text with "+e.length+" characters"}}function u(e){m.value=e,p(e,h.checked)}function d(e,t){let n="";for(let r=1;r<=t;r++)n+="Example: https://example.org/"+e+"/link-"+r+"\n";u(n)}c.m=i,e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",n="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},c.a=(o,a,i)=>{var l;i&&((l=[]).d=-1);var c,s,p,u=new Set,d=o.exports,m=new Promise(((e,t)=>{p=t,s=e}));m[t]=d,m[e]=e=>(l&&e(l),u.forEach(e),m.catch((e=>{}))),o.exports=m,a((o=>{var a;c=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var a=[];a.d=0,o.then((e=>{i[t]=e,r(a)}),(e=>{i[n]=e,r(a)}));var i={};return i[e]=e=>e(a),i}}var l={};return l[e]=e=>{},l[t]=o,l})))(o);var i=()=>c.map((e=>{if(e[n])throw e[n];return e[t]})),s=new Promise((t=>{(a=()=>t(i)).r=0;var n=e=>e!==l&&!u.has(e)&&(u.add(e),e&&!e.d&&(a.r++,e.push(a)));c.map((t=>t[e](n)))}));return a.r?s:i()}),(e=>(e?p(m[n]=e):s(d),r(l)))),l&&l.d<0&&(l.d=0)},c.d=(e,t)=>{for(var n in t)c.o(t,n)&&!c.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},c.f={},c.e=e=>Promise.all(Object.keys(c.f).reduce(((t,n)=>(c.f[n](e,t),t)),[])),c.u=e=>e+".index.js",c.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),c.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),c.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),o={},a="linkify-demo:",c.l=(e,t,n,r)=>{if(o[e])o[e].push(t);else{var i,l;if(void 0!==n)for(var s=document.getElementsByTagName("script"),p=0;p<s.length;p++){var u=s[p];if(u.getAttribute("src")==e||u.getAttribute("data-webpack")==a+n){i=u;break}}i||(l=!0,(i=document.createElement("script")).charset="utf-8",i.timeout=120,c.nc&&i.setAttribute("nonce",c.nc),i.setAttribute("data-webpack",a+n),i.src=e),o[e]=[t];var d=(t,n)=>{i.onerror=i.onload=null,clearTimeout(m);var r=o[e];if(delete o[e],i.parentNode&&i.parentNode.removeChild(i),r&&r.forEach((e=>e(n))),t)return t(n)},m=setTimeout(d.bind(null,void 0,{type:"timeout",target:i}),12e4);i.onerror=d.bind(null,i.onerror),i.onload=d.bind(null,i.onload),l&&document.head.appendChild(i)}},c.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},c.v=(e,t,n,r)=>{var o=fetch(c.p+""+n+".module.wasm"),a=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((t=>Object.assign(e,t.instance.exports)));return o.then((t=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(t,r).then((t=>Object.assign(e,t.instance.exports)),(e=>{if("application/wasm"!==t.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),a();throw e})):a()))},(()=>{var e;c.g.importScripts&&(e=c.g.location+"");var t=c.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var n=t.getElementsByTagName("script");if(n.length)for(var r=n.length-1;r>-1&&(!e||!/^http(s?):/.test(e));)e=n[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),c.p=e})(),(()=>{var e={792:0};c.f.j=(t,n)=>{var r=c.o(e,t)?e[t]:void 0;if(0!==r)if(r)n.push(r[2]);else{var o=new Promise(((n,o)=>r=e[t]=[n,o]));n.push(r[2]=o);var a=c.p+c.u(t),i=new Error;c.l(a,(n=>{if(c.o(e,t)&&(0!==(r=e[t])&&(e[t]=void 0),r)){var o=n&&("load"===n.type?"missing":n.type),a=n&&n.target&&n.target.src;i.message="Loading chunk "+t+" failed.\n("+o+": "+a+")",i.name="ChunkLoadError",i.type=o,i.request=a,r[1](i)}}),"chunk-"+t,t)}};var t=(t,n)=>{var r,o,[a,i,l]=n,s=0;if(a.some((t=>0!==e[t]))){for(r in i)c.o(i,r)&&(c.m[r]=i[r]);l&&l(c)}for(t&&t(n);s<a.length;s++)o=a[s],c.o(e,o)&&e[o]&&e[o][0](),e[o]=0},n=self.webpackChunklinkify_demo=self.webpackChunklinkify_demo||[];n.forEach(t.bind(null,0)),n.push=t.bind(null,n.push.bind(n))})(),c.e(300).then(c.bind(c,300)).then((e=>{s=e,p(m.value,h.checked)})).catch((e=>console.error("Failed to load WebAssembly module",e)));const m=document.getElementById("input"),h=document.getElementById("allow-without-scheme"),f=document.getElementById("output"),g=document.getElementById("timing");m.oninput=e=>p(e.target.value,h.checked),h.oninput=e=>p(m.value,e.target.checked),document.getElementById("example-urls").onclick=()=>u("Some links: https://example.org, https://example.com/a.\nSee how ',' and '.' are not included in the link?\n\nWhat about parentheses (https://example.org)? What if they are part of the link?: https://en.wikipedia.org/wiki/Link_(The_Legend_of_Zelda)\n\nWithout scheme: example.com or example.com/a"),document.getElementById("example-emails").onclick=()=>u("abc@example.org, foo+bar@example.org;hi@example.org"),document.getElementById("example-long").onclick=()=>d("long",100),document.getElementById("example-very-long").onclick=()=>d("very-long",5e3)})();