let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_0.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}
/**
 * @param {Pos} pos
 * @returns {number}
 */
export function x(pos) {
    const ret = wasm.x((__wbindgen_enum_Pos.indexOf(pos) + 1 || 38) - 1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
}

/**
 * @param {Pos} pos
 * @returns {number}
 */
export function y(pos) {
    const ret = wasm.y((__wbindgen_enum_Pos.indexOf(pos) + 1 || 38) - 1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
}

/**
 * @param {Pos} from
 * @param {Pos} to
 * @returns {number}
 */
export function distance(from, to) {
    const ret = wasm.distance((__wbindgen_enum_Pos.indexOf(from) + 1 || 38) - 1, (__wbindgen_enum_Pos.indexOf(to) + 1 || 38) - 1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_export_0.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}
/**
 * @enum {0 | 1 | 2 | 3}
 */
export const BeegoneError = Object.freeze({
    InvalidColor: 0, "0": "InvalidColor",
    InvalidSpecies: 1, "1": "InvalidSpecies",
    InvalidAction: 2, "2": "InvalidAction",
    InvalidPos: 3, "3": "InvalidPos",
});

const __wbindgen_enum_Color = ["light", "dark"];

const __wbindgen_enum_Pos = ["A4", "A5", "A6", "A7", "B3", "B4", "B5", "B6", "B7", "C2", "C3", "C4", "C5", "C6", "C7", "D1", "D2", "D3", "D4", "D5", "D6", "D7", "E1", "E2", "E3", "E4", "E5", "E6", "F1", "F2", "F3", "F4", "F5", "G1", "G2", "G3", "G4"];

const __wbindgen_enum_Species = ["drone", "worker", "nurse", "explorer", "builder", "queen", "guard"];

const ActionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_action_free(ptr >>> 0, 1));

export class Action {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Action.prototype);
        obj.__wbg_ptr = ptr;
        ActionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ActionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_action_free(ptr, 0);
    }
    /**
     * @returns {WasmMoveAction | undefined}
     */
    get move() {
        const ret = wasm.__wbg_get_action_move(this.__wbg_ptr);
        return ret === 0 ? undefined : WasmMoveAction.__wrap(ret);
    }
    /**
     * @returns {WasmSpawnAction | undefined}
     */
    get spawn() {
        const ret = wasm.__wbg_get_action_spawn(this.__wbg_ptr);
        return ret === 0 ? undefined : WasmSpawnAction.__wrap(ret);
    }
    /**
     * @param {Pos} from
     * @param {Pos} to
     * @returns {Action}
     */
    static move(from, to) {
        const ret = wasm.action_move((__wbindgen_enum_Pos.indexOf(from) + 1 || 38) - 1, (__wbindgen_enum_Pos.indexOf(to) + 1 || 38) - 1);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Action.__wrap(ret[0]);
    }
    /**
     * @param {Pos} on
     * @param {Piece} spawn
     * @returns {Action}
     */
    static spawn(on, spawn) {
        _assertClass(spawn, Piece);
        const ret = wasm.action_spawn((__wbindgen_enum_Pos.indexOf(on) + 1 || 38) - 1, spawn.__wbg_ptr);
        return Action.__wrap(ret);
    }
}

const BeeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_bee_free(ptr >>> 0, 1));

export class Bee {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Bee.prototype);
        obj.__wbg_ptr = ptr;
        BeeFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
            color: this.color,
            species: this.species,
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BeeFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bee_free(ptr, 0);
    }
    /**
     * @returns {Color}
     */
    get color() {
        const ret = wasm.__wbg_get_bee_color(this.__wbg_ptr);
        return __wbindgen_enum_Color[ret];
    }
    /**
     * @returns {Species}
     */
    get species() {
        const ret = wasm.__wbg_get_bee_species(this.__wbg_ptr);
        return __wbindgen_enum_Species[ret];
    }
    /**
     * @param {Color} color
     * @param {Species} species
     */
    constructor(color, species) {
        const ret = wasm.bee_new((__wbindgen_enum_Color.indexOf(color) + 1 || 3) - 1, (__wbindgen_enum_Species.indexOf(species) + 1 || 8) - 1);
        this.__wbg_ptr = ret >>> 0;
        BeeFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const BoardFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_board_free(ptr >>> 0, 1));

export class Board {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Board.prototype);
        obj.__wbg_ptr = ptr;
        BoardFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BoardFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_board_free(ptr, 0);
    }
    /**
     * @param {Pos} pos
     * @returns {Piece | undefined}
     */
    get(pos) {
        const ret = wasm.board_get(this.__wbg_ptr, (__wbindgen_enum_Pos.indexOf(pos) + 1 || 38) - 1);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] === 0 ? undefined : Piece.__wrap(ret[0]);
    }
    /**
     * @returns {Pos[]}
     */
    static positions() {
        const ret = wasm.board_positions();
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
}

const PieceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_piece_free(ptr >>> 0, 1));

export class Piece {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Piece.prototype);
        obj.__wbg_ptr = ptr;
        PieceFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
            bee: this.bee,
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PieceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_piece_free(ptr, 0);
    }
    /**
     * If `None`, the piece is a wall.
     * @returns {Bee | undefined}
     */
    get bee() {
        const ret = wasm.__wbg_get_piece_bee(this.__wbg_ptr);
        return ret === 0 ? undefined : Bee.__wrap(ret);
    }
}

const StateFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_state_free(ptr >>> 0, 1));

export class State {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(State.prototype);
        obj.__wbg_ptr = ptr;
        StateFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StateFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_state_free(ptr, 0);
    }
    /**
     * @returns {Board}
     */
    get board() {
        const ret = wasm.__wbg_get_state_board(this.__wbg_ptr);
        return Board.__wrap(ret);
    }
    /**
     * @returns {Color}
     */
    get turn() {
        const ret = wasm.__wbg_get_bee_color(this.__wbg_ptr);
        return __wbindgen_enum_Color[ret];
    }
    constructor() {
        const ret = wasm.state_new();
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        StateFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Pos} pos
     * @returns {Action[]}
     */
    actionsFrom(pos) {
        const ret = wasm.state_actionsFrom(this.__wbg_ptr, (__wbindgen_enum_Pos.indexOf(pos) + 1 || 38) - 1);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Action} action
     * @returns {State}
     */
    perform(action) {
        _assertClass(action, Action);
        const ret = wasm.state_perform(this.__wbg_ptr, action.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return State.__wrap(ret[0]);
    }
}

const WasmMoveActionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmmoveaction_free(ptr >>> 0, 1));

export class WasmMoveAction {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmMoveAction.prototype);
        obj.__wbg_ptr = ptr;
        WasmMoveActionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmMoveActionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmmoveaction_free(ptr, 0);
    }
    /**
     * @returns {Pos}
     */
    get from() {
        const ret = wasm.__wbg_get_wasmmoveaction_from(this.__wbg_ptr);
        return __wbindgen_enum_Pos[ret];
    }
    /**
     * @returns {Pos}
     */
    get to() {
        const ret = wasm.__wbg_get_wasmmoveaction_to(this.__wbg_ptr);
        return __wbindgen_enum_Pos[ret];
    }
}

const WasmSpawnActionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmspawnaction_free(ptr >>> 0, 1));

export class WasmSpawnAction {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmSpawnAction.prototype);
        obj.__wbg_ptr = ptr;
        WasmSpawnActionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmSpawnActionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmspawnaction_free(ptr, 0);
    }
    /**
     * @returns {Pos}
     */
    get on() {
        const ret = wasm.__wbg_get_wasmspawnaction_on(this.__wbg_ptr);
        return __wbindgen_enum_Pos[ret];
    }
    /**
     * @returns {Piece}
     */
    get spawn() {
        const ret = wasm.__wbg_get_wasmspawnaction_spawn(this.__wbg_ptr);
        return Piece.__wrap(ret);
    }
}

export function __wbg_action_new(arg0) {
    const ret = Action.__wrap(arg0);
    return ret;
};

export function __wbg_new_78feb108b6472713() {
    const ret = new Array();
    return ret;
};

export function __wbg_push_737cfc8c1432c2c6(arg0, arg1) {
    const ret = arg0.push(arg1);
    return ret;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_0;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

export function __wbindgen_number_new(arg0) {
    const ret = arg0;
    return ret;
};

export function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

