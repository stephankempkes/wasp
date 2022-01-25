// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package wasmtypes

import (
	"github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib/wasmcodec"
)

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

func DecodeBool(dec *wasmcodec.WasmDecoder) bool {
	return dec.Byte() != 0
}

func EncodeBool(enc *wasmcodec.WasmEncoder, value bool) {
	if value {
		enc.Byte(1)
		return
	}
	enc.Byte(0)
}

func BoolFromBytes(buf []byte) bool {
	if buf == nil {
		return false
	}
	if len(buf) != 1 {
		Panic("invalid Bool length")
	}
	return buf[0] != 0
}

func BytesFromBool(value bool) []byte {
	if value {
		return []byte{1}
	}
	return []byte{0}
}

func StringFromBool(value bool) string {
	if value {
		return "1"
	}
	return "0"
}

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

type ScImmutableBool struct {
	proxy Proxy
}

func NewScImmutableBool(proxy Proxy) ScImmutableBool {
	return ScImmutableBool{proxy: proxy}
}

func (o ScImmutableBool) Exists() bool {
	return o.proxy.Exists()
}

func (o ScImmutableBool) String() string {
	return StringFromBool(o.Value())
}

func (o ScImmutableBool) Value() bool {
	return BoolFromBytes(o.proxy.Get())
}

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

type ScMutableBool struct {
	ScImmutableBool
}

func NewScMutableBool(proxy Proxy) ScMutableBool {
	return ScMutableBool{ScImmutableBool{proxy: proxy}}
}

func (o ScMutableBool) Delete() {
	o.proxy.Delete()
}

func (o ScMutableBool) SetValue(value bool) {
	o.proxy.Set(BytesFromBool(value))
}
