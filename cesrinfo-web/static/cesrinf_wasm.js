let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let cachedUint32Memory0 = null;

function getUint32Memory0() {
    if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32Memory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getUint32Memory0();
    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
    const result = [];
    for (let i = 0; i < slice.length; i++) {
        result.push(takeObject(slice[i]));
    }
    return result;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getUint32Memory0();
    for (let i = 0; i < array.length; i++) {
        mem[ptr / 4 + i] = addHeapObject(array[i]);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
* @returns {number}
*/
export function constant() {
    const ret = wasm.constant();
    return ret >>> 0;
}

const CesrParserFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cesrparser_free(ptr >>> 0));
/**
*/
export class CesrParser {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CesrParserFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cesrparser_free(ptr);
    }
    /**
    * @param {string} stream
    */
    constructor(stream) {
        const ptr0 = passStringToWasm0(stream, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cesrparser_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {WrappedParsedData}
    */
    parse() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.cesrparser_parse(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return WrappedParsedData.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const WrappedCodeageFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wrappedcodeage_free(ptr >>> 0));
/**
*/
export class WrappedCodeage {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WrappedCodeage.prototype);
        obj.__wbg_ptr = ptr;
        WrappedCodeageFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WrappedCodeageFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wrappedcodeage_free(ptr);
    }
    /**
    * @returns {string}
    */
    get selector() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wrappedcodeage_selector(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set selector(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wrappedcodeage_selector(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get description() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wrappedcodeage_description(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} arg0
    */
    set description(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wrappedcodeage_description(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {number}
    */
    get hs() {
        const ret = wasm.__wbg_get_wrappedcodeage_hs(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set hs(arg0) {
        wasm.__wbg_set_wrappedcodeage_hs(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get ss() {
        const ret = wasm.__wbg_get_wrappedcodeage_ss(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set ss(arg0) {
        wasm.__wbg_set_wrappedcodeage_ss(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get fs() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wrappedcodeage_fs(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {number | undefined} [arg0]
    */
    set fs(arg0) {
        wasm.__wbg_set_wrappedcodeage_fs(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number}
    */
    get cs() {
        const ret = wasm.__wbg_get_wrappedcodeage_cs(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set cs(arg0) {
        wasm.__wbg_set_wrappedcodeage_cs(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get vs() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wrappedcodeage_vs(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {number | undefined} [arg0]
    */
    set vs(arg0) {
        wasm.__wbg_set_wrappedcodeage_vs(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number}
    */
    get ls() {
        const ret = wasm.__wbg_get_wrappedcodeage_ls(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set ls(arg0) {
        wasm.__wbg_set_wrappedcodeage_ls(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get ps() {
        const ret = wasm.__wbg_get_wrappedcodeage_ps(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set ps(arg0) {
        wasm.__wbg_set_wrappedcodeage_ps(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get os() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wrappedcodeage_os(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {number | undefined} [arg0]
    */
    set os(arg0) {
        wasm.__wbg_set_wrappedcodeage_os(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get ms() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wrappedcodeage_ms(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {number | undefined} [arg0]
    */
    set ms(arg0) {
        wasm.__wbg_set_wrappedcodeage_ms(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
}

const WrappedMsgFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wrappedmsg_free(ptr >>> 0));
/**
*/
export class WrappedMsg {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WrappedMsg.prototype);
        obj.__wbg_ptr = ptr;
        WrappedMsgFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WrappedMsg)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WrappedMsgFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wrappedmsg_free(ptr);
    }
    /**
    * @returns {any}
    */
    get msg_type() {
        const ret = wasm.__wbg_get_wrappedmsg_msg_type(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set msg_type(arg0) {
        wasm.__wbg_set_wrappedmsg_msg_type(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {WrappedCodeage}
    */
    get codeage() {
        const ret = wasm.__wbg_get_wrappedmsg_codeage(this.__wbg_ptr);
        return WrappedCodeage.__wrap(ret);
    }
    /**
    * @param {WrappedCodeage} arg0
    */
    set codeage(arg0) {
        _assertClass(arg0, WrappedCodeage);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wrappedmsg_codeage(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {number}
    */
    get istart() {
        const ret = wasm.__wbg_get_wrappedmsg_istart(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set istart(arg0) {
        wasm.__wbg_set_wrappedmsg_istart(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {(WrappedMsg)[] | undefined}
    */
    get indexed() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wrappedmsg_indexed(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v1;
            if (r0 !== 0) {
                v1 = getArrayJsValueFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 4, 4);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {(WrappedMsg)[] | undefined} [arg0]
    */
    set indexed(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wrappedmsg_indexed(this.__wbg_ptr, ptr0, len0);
    }
}

const WrappedParsedDataFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wrappedparseddata_free(ptr >>> 0));
/**
*/
export class WrappedParsedData {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WrappedParsedData.prototype);
        obj.__wbg_ptr = ptr;
        WrappedParsedDataFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WrappedParsedDataFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wrappedparseddata_free(ptr);
    }
    /**
    * @returns {(WrappedMsg)[]}
    */
    get msgs() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_wrappedparseddata_msgs(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {(WrappedMsg)[]} arg0
    */
    set msgs(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wrappedparseddata_msgs(this.__wbg_ptr, ptr0, len0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_wrappedmsg_new = function(arg0) {
        const ret = WrappedMsg.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_wrappedmsg_unwrap = function(arg0) {
        const ret = WrappedMsg.__unwrap(takeObject(arg0));
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_73a5987615ec8862 = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('cesrinf_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
