'use strict';
var crypto = require('crypto');
exports = module.exports = objectHash;
function objectHash(object, options) {
    options = applyDefaults(object, options);
    return hash(object, options);
}
exports.sha1 = function (object) {
    return objectHash(object);
};
exports.keys = function (object) {
    return objectHash(object, { excludeValues: true, algorithm: 'sha1', encoding: 'hex' });
};
exports.MD5 = function (object) {
    return objectHash(object, { algorithm: 'md5', encoding: 'hex' });
};
exports.keysMD5 = function (object) {
    return objectHash(object, { algorithm: 'md5', encoding: 'hex', excludeValues: true });
};
var hashes = crypto.getHashes ? crypto.getHashes().slice() : ['sha1', 'md5'];
hashes.push('passthrough');
var encodings = ['buffer', 'hex', 'binary', 'base64'];
function applyDefaults(object, sourceOptions) {
    sourceOptions = sourceOptions || {};
    var options = {};
    options.algorithm = sourceOptions.algorithm || 'sha1';
    options.encoding = sourceOptions.encoding || 'hex';
    options.excludeValues = sourceOptions.excludeValues ? true : false;
    options.algorithm = options.algorithm.toLowerCase();
    options.encoding = options.encoding.toLowerCase();
    options.ignoreUnknown = sourceOptions.ignoreUnknown !== true ? false : true;
    options.respectType = sourceOptions.respectType === false ? false : true;
    options.respectFunctionNames = sourceOptions.respectFunctionNames === false ? false : true;
    options.respectFunctionProperties = sourceOptions.respectFunctionProperties === false ? false : true;
    options.unorderedArrays = sourceOptions.unorderedArrays !== true ? false : true;
    options.unorderedSets = sourceOptions.unorderedSets === false ? false : true;
    options.unorderedObjects = sourceOptions.unorderedObjects === false ? false : true;
    options.replacer = sourceOptions.replacer || undefined;
    options.excludeKeys = sourceOptions.excludeKeys || undefined;
    if (typeof object === 'undefined') {
        throw new Error('Object argument required.');
    }
    for (var i = 0; i < hashes.length; ++i) {
        if (hashes[i].toLowerCase() === options.algorithm.toLowerCase()) {
            options.algorithm = hashes[i];
        }
    }
    if (hashes.indexOf(options.algorithm) === -1) {
        throw new Error('Algorithm "' + options.algorithm + '"  not supported. ' + 'supported values: ' + hashes.join(', '));
    }
    if (encodings.indexOf(options.encoding) === -1 &&
        options.algorithm !== 'passthrough') {
        throw new Error('Encoding "' + options.encoding + '"  not supported. ' + 'supported values: ' + encodings.join(', '));
    }
    return options;
}
function isNativeFunction(f) {
    if ((typeof f) !== 'function') {
        return false;
    }
    var exp = /^function\s+\w*\s*\(\s*\)\s*{\s+\[native code\]\s+}$/i;
    return exp.exec(Function.prototype.toString.call(f)) != null;
}
function hash(object, options) {
    var hashingStream;
    if (options.algorithm !== 'passthrough') {
        hashingStream = crypto.createHash(options.algorithm);
    } else {
        hashingStream = new PassThrough();
    }
    if (typeof hashingStream.write === 'undefined') {
        hashingStream.write = hashingStream.update;
        hashingStream.end = hashingStream.update;
    }
    var hasher = typeHasher(options, hashingStream);
    hasher.dispatch(object);
    if (!hashingStream.update) {
        hashingStream.end('');
    }
    if (hashingStream.digest) {
        return hashingStream.digest(options.encoding === 'buffer' ? undefined : options.encoding);
    }
    var buf = hashingStream.read();
    if (options.encoding === 'buffer') {
        return buf;
    }
    return buf.toString(options.encoding);
}
exports.writeToStream = function (object, options, stream) {
    if (typeof stream === 'undefined') {
        stream = options;
        options = {};
    }
    options = applyDefaults(object, options);
    return typeHasher(options, stream).dispatch(object);
};

function typeHasher(options, writeTo, context) {
    context = context || [];
    var write = function (str) {
        if (writeTo.update) {
            return writeTo.update(str, 'utf8');
        } else {
            return writeTo.write(str, 'utf8');
        }
    };
    return {
        dispatch: function (value) {
            if (options.replacer) {
                value = options.replacer(value);
            }
            var type = typeof value;
            if (value === null) {
                type = 'null';
            }
            return this['_' + type](value);
        },
        _object: function (object) {
            var pattern = (/\[object (.*)\]/i);
            var objString = Object.prototype.toString.call(object);
            var objType = pattern.exec(objString);
            if (!objType) {
                objType = 'unknown:[' + objString + ']';
            } else {
                objType = objType[1];
            }
            objType = objType.toLowerCase();
            var objectNumber = null;
            if ((objectNumber = context.indexOf(object)) >= 0) {
                return this.dispatch('[CIRCULAR:' + objectNumber + ']');
            } else {
                context.push(object);
            }
            if (typeof Buffer !== 'undefined' && Buffer.isBuffer && Buffer.isBuffer(object)) {
                write('buffer:');
                return write(object);
            }
            if (objType !== 'object' && objType !== 'function' && objType !== 'asyncfunction') {
                if (this['_' + objType]) {
                    this['_' + objType](object);
                } else if (options.ignoreUnknown) {
                    return write('[' + objType + ']');
                } else {
                    throw new Error('Unknown object type "' + objType + '"');
                }
            } else {
                var keys = Object.keys(object);
                if (options.unorderedObjects) {
                    keys = keys.sort();
                }
                if (options.respectType !== false && !isNativeFunction(object)) {
                    keys.splice(0, 0, 'prototype', '__proto__', 'constructor');
                }
                if (options.excludeKeys) {
                    keys = keys.filter(function (key) { return !options.excludeKeys(key); });
                }
                write('object:' + keys.length + ':');
                var self = this;
                return keys.forEach(function (key) {
                    self.dispatch(key);
                    write(':');
                    if (!options.excludeValues) {
                        self.dispatch(object[key]);
                    }
                    write(',');
                });
            }
        },
        _array: function (arr, unordered) {
            unordered = typeof unordered !== 'undefined' ? unordered :
                options.unorderedArrays !== false;
            var self = this;
            write('array:' + arr.length + ':');
            if (!unordered || arr.length <= 1) {
                return arr.forEach(function (entry) {
                    return self.dispatch(entry);
                });
            }
            var contextAdditions = [];
            var entries = arr.map(function (entry) {
                var strm = new PassThrough();
                var localContext = context.slice();
                var hasher = typeHasher(options, strm, localContext);
                hasher.dispatch(entry);
                contextAdditions = contextAdditions.concat(localContext.slice(context.length));
                return strm.read().toString();
            });
            context = context.concat(contextAdditions);
            entries.sort();
            return this._array(entries, false);
        },
        _date: function (date) {
            return write('date:' + date.toJSON());
        },
        _symbol: function (sym) {
            return write('symbol:' + sym.toString());
        },
        _error: function (err) {
            return write('error:' + err.toString());
        },
        _boolean: function (bool) {
            return write('bool:' + bool.toString());
        },
        _string: function (string) {
            write('string:' + string.length + ':');
            write(string.toString());
        },
        _function: function (fn) {
            write('fn:');
            if (isNativeFunction(fn)) {
                this.dispatch('[native]');
            } else {
                this.dispatch(fn.toString());
            }
            if (options.respectFunctionNames !== false) {
                this.dispatch("function-name:" + String(fn.name));
            }
            if (options.respectFunctionProperties) {
                this._object(fn);
            }
        },
        _number: function (number) {
            return write('number:' + number.toString());
        },
        _xml: function (xml) {
            return write('xml:' + xml.toString());
        },
        _null: function () {
            return write('Null');
        },
        _undefined: function () {
            return write('Undefined');
        },
        _regexp: function (regex) {
            return write('regex:' + regex.toString());
        },
        _uint8array: function (arr) {
            write('uint8array:');
            return this.dispatch(Array.prototype.slice.call(arr));
        },
        _uint8clampedarray: function (arr) {
            write('uint8clampedarray:');
            return this.dispatch(Array.prototype.slice.call(arr));
        },
        _int8array: function (arr) {
            write('int8array:');
            return this.dispatch(Array.prototype.slice.call(arr));
        },
        _uint16array: function (arr) {
            write('uint16array:');
            return this.dispatch(Array.prototype.slice.call(arr));
        },
        _int16array: function (arr) {
            write('int16array:');
            return this.dispatch(Array.prototype.slice.call(arr));
        },
        _uint32array: function (arr) {
            write('uint32array:');
            return this.dispatch(Array.prototype.slice.call(arr));
        },
        _int32array: function (arr) {
            write('int32array:');
            return this.dispatch(Array.prototype.slice.call(arr));
        },
        _float32array: function (arr) {
            write('float32array:');
            return this.dispatch(Array.prototype.slice.call(arr));
        },
        _float64array: function (arr) {
            write('float64array:');
            return this.dispatch(Array.prototype.slice.call(arr));
        },
        _arraybuffer: function (arr) {
            write('arraybuffer:');
            return this.dispatch(new Uint8Array(arr));
        },
        _url: function (url) {
            return write('url:' + url.toString(), 'utf8');
        },
        _map: function (map) {
            write('map:');
            var arr = Array.from(map);
            return this._array(arr, options.unorderedSets !== false);
        },
        _set: function (set) {
            write('set:');
            var arr = Array.from(set);
            return this._array(arr, options.unorderedSets !== false);
        },
        _file: function (file) {
            write('file:');
            return this.dispatch([file.name, file.size, file.type, file.lastModfied]);
        },
        _blob: function () {
            if (options.ignoreUnknown) {
                return write('[blob]');
            }
            throw Error('Hashing Blob objects is currently not supported\n' +
                '(see https://github.com/puleos/object-hash/issues/26)\n' +
                'Use "options.replacer" or "options.ignoreUnknown"\n');
        },
        _domwindow: function () { return write('domwindow'); },
        _bigint: function (number) {
            return write('bigint:' + number.toString());
        },
        _process: function () { return write('process'); },
        _timer: function () { return write('timer'); },
        _pipe: function () { return write('pipe'); },
        _tcp: function () { return write('tcp'); },
        _udp: function () { return write('udp'); },
        _tty: function () { return write('tty'); },
        _statwatcher: function () { return write('statwatcher'); },
        _securecontext: function () { return write('securecontext'); },
        _connection: function () { return write('connection'); },
        _zlib: function () { return write('zlib'); },
        _context: function () { return write('context'); },
        _nodescript: function () { return write('nodescript'); },
        _httpparser: function () { return write('httpparser'); },
        _dataview: function () { return write('dataview'); },
        _signal: function () { return write('signal'); },
        _fsevent: function () { return write('fsevent'); },
        _tlswrap: function () { return write('tlswrap'); },
    };
}
function PassThrough() {
    return {
        buf: '',
        write: function (b) {
            this.buf += b;
        },
        end: function (b) {
            this.buf += b;
        },
        read: function () {
            return this.buf;
        }
    };
}