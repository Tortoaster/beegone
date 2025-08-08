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

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
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

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_0.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}
/**
 * @enum {0 | 1}
 */
export const Color = Object.freeze({
    Light: 0, "0": "Light",
    Dark: 1, "1": "Dark",
});
/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6}
 */
export const Species = Object.freeze({
    Drone: 0, "0": "Drone",
    Worker: 1, "1": "Worker",
    Nurse: 2, "2": "Nurse",
    Builder: 3, "3": "Builder",
    Explorer: 4, "4": "Explorer",
    Guard: 5, "5": "Guard",
    Queen: 6, "6": "Queen",
});

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
        _assertClass(from, Pos);
        _assertClass(to, Pos);
        const ret = wasm.action_move(from.__wbg_ptr, to.__wbg_ptr);
        return Action.__wrap(ret);
    }
    /**
     * @param {Pos} on
     * @param {Piece} spawn
     * @returns {Action}
     */
    static spawn(on, spawn) {
        _assertClass(on, Pos);
        _assertClass(spawn, Piece);
        const ret = wasm.action_spawn(on.__wbg_ptr, spawn.__wbg_ptr);
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
        return ret;
    }
    /**
     * @returns {Species}
     */
    get species() {
        const ret = wasm.__wbg_get_bee_species(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Color} color
     * @param {Species} species
     */
    constructor(color, species) {
        const ret = wasm.bee_new(color, species);
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
        _assertClass(pos, Pos);
        const ret = wasm.board_get(this.__wbg_ptr, pos.__wbg_ptr);
        return ret === 0 ? undefined : Piece.__wrap(ret);
    }
    /**
     * @returns {Pos[]}
     */
    static positions() {
        const ret = wasm.board_positions();
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
}

const InvalidActionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_invalidaction_free(ptr >>> 0, 1));

export class InvalidAction {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(InvalidAction.prototype);
        obj.__wbg_ptr = ptr;
        InvalidActionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InvalidActionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_invalidaction_free(ptr, 0);
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

const PosFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_pos_free(ptr >>> 0, 1));

export class Pos {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Pos.prototype);
        obj.__wbg_ptr = ptr;
        PosFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PosFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pos_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get q() {
        const ret = wasm.__wbg_get_pos_q(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get r() {
        const ret = wasm.__wbg_get_pos_r(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} q
     * @param {number} r
     */
    constructor(q, r) {
        const ret = wasm.pos_new(q, r);
        this.__wbg_ptr = ret >>> 0;
        PosFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get s() {
        const ret = wasm.pos_s(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.pos_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.pos_y(this.__wbg_ptr);
        return ret;
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
        const ret = wasm.__wbg_get_state_turn(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} players
     */
    constructor(players) {
        const ret = wasm.state_new(players);
        this.__wbg_ptr = ret >>> 0;
        StateFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Pos} pos
     * @returns {Action[]}
     */
    actionsFrom(pos) {
        _assertClass(pos, Pos);
        const ret = wasm.state_actionsFrom(this.__wbg_ptr, pos.__wbg_ptr);
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
        return Pos.__wrap(ret);
    }
    /**
     * @returns {Pos}
     */
    get to() {
        const ret = wasm.__wbg_get_wasmmoveaction_to(this.__wbg_ptr);
        return Pos.__wrap(ret);
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
        const ret = wasm.__wbg_get_wasmmoveaction_from(this.__wbg_ptr);
        return Pos.__wrap(ret);
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

export function __wbg_invalidaction_new(arg0) {
    const ret = InvalidAction.__wrap(arg0);
    return ret;
};

export function __wbg_pos_new(arg0) {
    const ret = Pos.__wrap(arg0);
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

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

